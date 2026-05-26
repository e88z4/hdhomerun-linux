use std::collections::HashMap;

use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::dvr::{StaticDvrFixtures, StaticDvrRecordingFixture};
use hdhomerun_backend::models::{
    ContractEndpointStatus, DvrRecordingDeleteOutcome, DvrRecordingDeleteResponse,
    DvrRecordingSummary, DvrRecordingsResponse, DvrRecordingsState, PlaybackCommandResponse,
    PlaybackCurrentResponse, PlaybackMode, PlaybackSessionStatus, RememberedContext,
};
use hdhomerun_backend::playback::StaticPlayerAdapterFixtures;
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

fn recording_fixture(recording_id: &str) -> StaticDvrRecordingFixture {
    StaticDvrRecordingFixture {
        summary: DvrRecordingSummary {
            recording_id: recording_id.to_string(),
            title: "Example Show".to_string(),
            episode_title: Some("Pilot".to_string()),
            synopsis: Some("Episode synopsis".to_string()),
            image_url: None,
            channel_name: Some("News".to_string()),
            channel_number: Some("5.1".to_string()),
            record_start_time: 1_700_000_000,
            record_end_time: 1_700_000_600,
            resume_position: 120,
            watched: false,
            source_count: 2,
            preferred_local: true,
        },
        playback_url: Some("http://192.168.1.10:4999/play/recording-1".to_string()),
        cmd_url: Some("http://192.168.1.10:4999/cmd/recording-1?token=abc".to_string()),
    }
}

fn dvr_test_state(fixtures: StaticDvrFixtures) -> AppState {
    let temp = tempdir().expect("tempdir");
    let state_dir = temp.keep();
    let state = AppState::for_tests_with_playback_fixtures(
        state_dir,
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            device_auth: Some("device-auth".to_string()),
            storage_url: Some("http://192.168.1.10:4999/recorded_files.json".to_string()),
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 4,
            is_legacy: false,
        }],
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        fixtures,
        StaticPlayerAdapterFixtures::default(),
    );
    state
        .state_store()
        .save_context(&RememberedContext {
            device_ref: Some("hdhr-1234abcd".to_string()),
            channel_ref: Some("5.1".to_string()),
            auto_resume: false,
            updated_at: "2026-05-20T23:32:16Z".to_string(),
        })
        .expect("save state");
    state
}

#[tokio::test]
async fn dvr_recordings_endpoint_returns_ready_library() {
    let app = build_app(dvr_test_state(StaticDvrFixtures {
        recordings: vec![recording_fixture("recording-1")],
        ..StaticDvrFixtures::default()
    }));

    let response = app
        .oneshot(Request::builder().uri("/api/dvr/recordings").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: DvrRecordingsResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.status, ContractEndpointStatus::Available);
    assert_eq!(payload.state, DvrRecordingsState::Ready);
    assert_eq!(payload.recordings.len(), 1);
    assert_eq!(payload.recordings[0].recording_id, "recording-1");
}

#[tokio::test]
async fn dvr_recording_play_starts_recorded_playback_mode() {
    let app = build_app(dvr_test_state(StaticDvrFixtures {
        recordings: vec![recording_fixture("recording-1")],
        ..StaticDvrFixtures::default()
    }));

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/dvr/recordings/recording-1/play")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCommandResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Playing);
    assert_eq!(payload.session_state.playback_mode, PlaybackMode::Recorded);
    assert!(payload.current_channel.is_none());
    assert_eq!(payload.current_recording.as_ref().map(|recording| recording.recording_id.as_str()), Some("recording-1"));
}

#[tokio::test]
async fn dvr_recording_delete_removes_recording_from_subsequent_library_reads() {
    let app = build_app(dvr_test_state(StaticDvrFixtures {
        recordings: vec![recording_fixture("recording-1")],
        ..StaticDvrFixtures::default()
    }));

    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/dvr/recordings/recording-1/delete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("delete response");

    let delete_body = delete_response.into_body().collect().await.expect("body").to_bytes();
    let delete_payload: DvrRecordingDeleteResponse = serde_json::from_slice(&delete_body).expect("json");
    assert_eq!(delete_payload.outcome, DvrRecordingDeleteOutcome::Confirmed);

    let list_response = app
        .oneshot(Request::builder().uri("/api/dvr/recordings").body(Body::empty()).unwrap())
        .await
        .expect("list response");
    let list_body = list_response.into_body().collect().await.expect("body").to_bytes();
    let list_payload: DvrRecordingsResponse = serde_json::from_slice(&list_body).expect("json");

    assert!(list_payload.recordings.is_empty());
}

#[tokio::test]
async fn dvr_recording_delete_returns_missing_outcome_when_recording_was_already_removed() {
    let app = build_app(dvr_test_state(StaticDvrFixtures::default()));

    let delete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/dvr/recordings/recording-1/delete")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("delete response");

    assert!(delete_response.status().is_success());
    let delete_body = delete_response.into_body().collect().await.expect("body").to_bytes();
    let delete_payload: DvrRecordingDeleteResponse = serde_json::from_slice(&delete_body).expect("json");

    assert_eq!(delete_payload.outcome, DvrRecordingDeleteOutcome::MissingRecording);
    assert_eq!(delete_payload.status, ContractEndpointStatus::Available);
    assert_eq!(delete_payload.warnings.len(), 1);
    assert!(delete_payload.warnings[0].contains("refresh recordings and try again"));
}

#[tokio::test]
async fn playback_stop_after_recorded_playback_clears_recording_context() {
    let app = build_app(dvr_test_state(StaticDvrFixtures {
        recordings: vec![recording_fixture("recording-1")],
        ..StaticDvrFixtures::default()
    }));

    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/dvr/recordings/recording-1/play")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("play response");

    let stop_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/stop")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("stop response");

    let stop_body = stop_response.into_body().collect().await.expect("body").to_bytes();
    let stop_payload: PlaybackCommandResponse = serde_json::from_slice(&stop_body).expect("json");

    assert_eq!(stop_payload.session_state.status, PlaybackSessionStatus::Stopped);
    assert_eq!(stop_payload.session_state.playback_mode, PlaybackMode::Idle);
    assert!(stop_payload.current_recording.is_none());

    let current_response = app
        .oneshot(Request::builder().uri("/api/playback/current").body(Body::empty()).unwrap())
        .await
        .expect("current response");
    let current_body = current_response.into_body().collect().await.expect("body").to_bytes();
    let current_payload: PlaybackCurrentResponse = serde_json::from_slice(&current_body).expect("json");

    assert_eq!(current_payload.session_state.status, PlaybackSessionStatus::Stopped);
    assert_eq!(current_payload.session_state.playback_mode, PlaybackMode::Idle);
    assert!(current_payload.current_recording.is_none());
}