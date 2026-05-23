use std::collections::HashMap;

use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::models::{
    ChannelAvailability, ContractEndpointStatus, LineupChannel, LineupResponse, LineupState,
    RememberedContext,
};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn lineup_endpoint_returns_ready_channels_for_selected_device() {
    let temp = tempdir().expect("tempdir");
    let mut lineups = HashMap::new();
    lineups.insert(
        "hdhr-1234abcd".to_string(),
        Ok(vec![LineupChannel {
            channel_ref: "channel:5.1".to_string(),
            guide_number: "5.1".to_string(),
            guide_name: "News".to_string(),
            current_program_title: None,
            image_url: None,
            tags: vec!["favorite".to_string()],
            playback_url: Some("http://192.168.1.10/auto/v5.1".to_string()),
            availability: ChannelAvailability::Playable,
            restriction_reason: None,
        }]),
    );
    let state = AppState::for_tests_with_fixtures(
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

    let app = build_app(state);
    let response = app
        .oneshot(Request::builder().uri("/api/lineup").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: LineupResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.status, ContractEndpointStatus::Available);
    assert_eq!(payload.state, LineupState::Ready);
    assert_eq!(payload.channels.len(), 1);
    assert_eq!(payload.channels[0].availability, ChannelAvailability::Playable);
}

#[tokio::test]
async fn lineup_endpoint_requires_selected_device() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_fixtures(
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
        HashMap::new(),
        HashMap::new(),
    );

    let app = build_app(state);
    let response = app
        .oneshot(Request::builder().uri("/api/lineup").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: LineupResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.state, LineupState::SelectionRequired);
    assert!(payload.channels.is_empty());
}

#[tokio::test]
async fn lineup_endpoint_includes_current_program_titles_when_guide_data_exists() {
    let temp = tempdir().expect("tempdir");
    let mut lineups = HashMap::new();
    lineups.insert(
        "hdhr-1234abcd".to_string(),
        Ok(vec![LineupChannel {
            channel_ref: "channel:5.1".to_string(),
            guide_number: "5.1".to_string(),
            guide_name: "News".to_string(),
            current_program_title: None,
            image_url: None,
            tags: vec!["favorite".to_string()],
            playback_url: Some("http://192.168.1.10/auto/v5.1".to_string()),
            availability: ChannelAvailability::Playable,
            restriction_reason: None,
        }]),
    );

    let mut guide_programs = HashMap::new();
    guide_programs.insert("channel:5.1".to_string(), "Evening News".to_string());

    let state = AppState::for_tests_with_guide_fixtures(
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
        guide_programs,
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

    let app = build_app(state);
    let response = app
        .oneshot(Request::builder().uri("/api/lineup").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: LineupResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.channels[0].current_program_title.as_deref(), Some("Evening News"));
}