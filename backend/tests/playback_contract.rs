use std::collections::HashMap;

use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::models::{
    ChannelAvailability, ContractEndpointStatus, LineupChannel, PlaybackCommandRequest,
    PlaybackCommandResponse, PlaybackCurrentResponse, PlaybackSessionStatus,
    PlayerAdapterState, PlayerAdapterStatus,
};
use hdhomerun_backend::playback::{PlayerAdapterError, StaticPlayerAdapterFixtures};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

fn playback_test_state(playback_fixtures: StaticPlayerAdapterFixtures) -> AppState {
    let temp = tempdir().expect("tempdir");
    let mut lineups = HashMap::new();
    lineups.insert(
        "hdhr-1234abcd".to_string(),
        Ok(vec![
            LineupChannel {
                channel_ref: "channel:5.1".to_string(),
                guide_number: "5.1".to_string(),
                guide_name: "News".to_string(),
                current_program_title: None,
                image_url: None,
                tags: vec!["favorite".to_string()],
                playback_url: Some("http://192.168.1.10/auto/v5.1".to_string()),
                availability: ChannelAvailability::Playable,
                restriction_reason: None,
            },
            LineupChannel {
                channel_ref: "channel:7.2".to_string(),
                guide_number: "7.2".to_string(),
                guide_name: "Sports".to_string(),
                current_program_title: None,
                image_url: None,
                tags: Vec::new(),
                playback_url: Some("http://192.168.1.10/auto/v7.2".to_string()),
                availability: ChannelAvailability::Playable,
                restriction_reason: None,
            },
        ]),
    );

    AppState::for_tests_with_playback_fixtures(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            device_auth: None,
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 4,
            is_legacy: false,
        }],
        lineups,
        HashMap::new(),
        HashMap::new(),
        playback_fixtures,
    )
}

#[tokio::test]
async fn playback_current_reports_idle_before_any_commands() {
    let app = build_app(playback_test_state(StaticPlayerAdapterFixtures::default()));

    let response = app
        .oneshot(Request::builder().uri("/api/playback/current").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCurrentResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.status, ContractEndpointStatus::Available);
    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Idle);
    assert_eq!(payload.adapter_state.adapter_status, PlayerAdapterStatus::NotStarted);
    assert!(payload.current_channel.is_none());
}

#[tokio::test]
async fn playback_current_warns_when_player_dependency_is_missing() {
    let app = build_app(playback_test_state(StaticPlayerAdapterFixtures {
        preflight_error: Some(PlayerAdapterError::new(
            "player_dependency_missing",
            "mpv executable is not available; install mpv or set HDHR_BACKEND_MPV_BIN to a valid executable path",
            false,
        )),
        ..StaticPlayerAdapterFixtures::default()
    }));

    let response = app
        .oneshot(Request::builder().uri("/api/playback/current").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCurrentResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Idle);
    assert_eq!(payload.adapter_state.adapter_status, PlayerAdapterStatus::NotStarted);
    assert_eq!(payload.warnings.len(), 1);
    assert!(payload.warnings[0].contains("mpv executable is not available"));
}

#[tokio::test]
async fn playback_start_returns_playing_state_and_updates_current_endpoint() {
    let state = playback_test_state(StaticPlayerAdapterFixtures::default());
    let app = build_app(state.clone());

    let request_body = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCommandResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Playing);
    assert_eq!(payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("5.1"));
    assert!(payload.failure.is_none());

    let current_response = app
        .oneshot(Request::builder().uri("/api/playback/current").body(Body::empty()).unwrap())
        .await
        .expect("response");
    let current_body = current_response.into_body().collect().await.expect("body").to_bytes();
    let current_payload: PlaybackCurrentResponse = serde_json::from_slice(&current_body).expect("json");

    assert_eq!(current_payload.session_state.status, PlaybackSessionStatus::Playing);
    assert_eq!(current_payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("5.1"));
    assert_eq!(
        state
            .state_store()
            .load_context()
            .expect("load state")
            .and_then(|context| context.channel_ref),
        Some("5.1".to_string())
    );
}

#[tokio::test]
async fn playback_switch_reuses_existing_session_and_adapter_process() {
    let app = build_app(playback_test_state(StaticPlayerAdapterFixtures::default()));

    let start_request = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");
    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(start_request))
                .unwrap(),
        )
        .await
        .expect("response");
    let start_body = start_response.into_body().collect().await.expect("body").to_bytes();
    let start_payload: PlaybackCommandResponse = serde_json::from_slice(&start_body).expect("json");

    let switch_request = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "7.2".to_string(),
    })
    .expect("serialize request");
    let switch_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/switch")
                .header("content-type", "application/json")
                .body(Body::from(switch_request))
                .unwrap(),
        )
        .await
        .expect("response");
    let switch_body = switch_response.into_body().collect().await.expect("body").to_bytes();
    let switch_payload: PlaybackCommandResponse = serde_json::from_slice(&switch_body).expect("json");

    assert_eq!(switch_payload.session_state.status, PlaybackSessionStatus::Playing);
    assert_eq!(switch_payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("7.2"));
    assert_eq!(switch_payload.session_state.session_id, start_payload.session_state.session_id);
    assert_eq!(switch_payload.adapter_state.process_id, start_payload.adapter_state.process_id);
}

#[tokio::test]
async fn playback_stop_keeps_last_channel_context_and_returns_stopped_state() {
    let app = build_app(playback_test_state(StaticPlayerAdapterFixtures::default()));

    let start_request = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");
    app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(start_request))
                .unwrap(),
        )
        .await
        .expect("response");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/stop")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("response");
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCommandResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Stopped);
    assert_eq!(payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("5.1"));
    assert_eq!(payload.adapter_state.adapter_status, PlayerAdapterStatus::AdapterReady);
}

#[tokio::test]
async fn playback_start_uses_one_bounded_automatic_retry_for_retryable_failure() {
    let fixtures = StaticPlayerAdapterFixtures {
        initial_state: PlayerAdapterState::not_started(),
        preflight_error: None,
        ensure_ready_results: Vec::new(),
        load_stream_results: vec![
            Err(PlayerAdapterError::new(
                "stream_load_failed",
                "first load attempt timed out",
                true,
            )),
            Ok(PlayerAdapterState {
                adapter_status: PlayerAdapterStatus::AdapterStreaming,
                process_id: Some(4101),
                last_command: Some("load_stream".to_string()),
                last_error: None,
                updated_at: "4101".to_string(),
            }),
        ],
        stop_stream_results: Vec::new(),
        rebuild_results: Vec::new(),
    };
    let app = build_app(playback_test_state(fixtures));

    let request_body = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("response");
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCommandResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Playing);
    assert!(payload.used_automatic_retry);
    assert_eq!(payload.session_state.retry_count, 1);
    assert_eq!(payload.adapter_state.process_id, Some(4101));
    assert!(payload.failure.is_none());
}

#[tokio::test]
async fn playback_start_fails_fast_when_player_dependency_is_missing() {
    let app = build_app(playback_test_state(StaticPlayerAdapterFixtures {
        preflight_error: Some(PlayerAdapterError::new(
            "player_dependency_missing",
            "mpv executable is not available; install mpv or set HDHR_BACKEND_MPV_BIN to a valid executable path",
            false,
        )),
        ..StaticPlayerAdapterFixtures::default()
    }));

    let request_body = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: PlaybackCommandResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.session_state.status, PlaybackSessionStatus::Failed);
    assert!(!payload.used_automatic_retry);
    assert_eq!(payload.adapter_state.process_id, None);
    assert_eq!(payload.failure.as_ref().map(|failure| failure.code.as_str()), Some("player_dependency_missing"));
    assert_eq!(payload.failure.as_ref().map(|failure| failure.retryable), Some(false));
    assert_eq!(payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("5.1"));
    assert_eq!(payload.warnings.len(), 1);
    assert!(payload.warnings[0].contains("mpv executable is not available"));
}

#[tokio::test]
async fn playback_retry_replays_the_last_failed_channel() {
    let fixtures = StaticPlayerAdapterFixtures {
        initial_state: PlayerAdapterState::not_started(),
        preflight_error: None,
        ensure_ready_results: Vec::new(),
        load_stream_results: vec![
            Err(PlayerAdapterError::new(
                "stream_load_failed",
                "initial playback failed",
                false,
            )),
            Ok(PlayerAdapterState {
                adapter_status: PlayerAdapterStatus::AdapterStreaming,
                process_id: Some(4102),
                last_command: Some("load_stream".to_string()),
                last_error: None,
                updated_at: "4102".to_string(),
            }),
        ],
        stop_stream_results: Vec::new(),
        rebuild_results: Vec::new(),
    };
    let app = build_app(playback_test_state(fixtures));

    let start_request = serde_json::to_vec(&PlaybackCommandRequest {
        device_ref: Some("hdhr-1234abcd".to_string()),
        channel_ref: "5.1".to_string(),
    })
    .expect("serialize request");
    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/start")
                .header("content-type", "application/json")
                .body(Body::from(start_request))
                .unwrap(),
        )
        .await
        .expect("response");
    let start_body = start_response.into_body().collect().await.expect("body").to_bytes();
    let start_payload: PlaybackCommandResponse = serde_json::from_slice(&start_body).expect("json");

    assert_eq!(start_payload.session_state.status, PlaybackSessionStatus::Failed);
    assert!(start_payload.failure.is_some());

    let retry_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/playback/retry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("response");
    let retry_body = retry_response.into_body().collect().await.expect("body").to_bytes();
    let retry_payload: PlaybackCommandResponse = serde_json::from_slice(&retry_body).expect("json");

    assert_eq!(retry_payload.session_state.status, PlaybackSessionStatus::Playing);
    assert_eq!(retry_payload.current_channel.as_ref().map(|channel| channel.guide_number.as_str()), Some("5.1"));
    assert!(retry_payload.failure.is_none());
    assert_eq!(retry_payload.adapter_state.process_id, Some(4102));
}