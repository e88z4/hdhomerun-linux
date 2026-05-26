use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::models::{BootstrapMode, BootstrapResult, HealthStatus, RememberedContext};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn health_endpoint_reports_ready_backend() {
    let temp = tempdir().expect("tempdir");
    let app = build_app(AppState::for_tests(temp.path().to_path_buf()));

    let response = app
        .oneshot(Request::builder().uri("/api/health").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: HealthStatus = serde_json::from_slice(&body).expect("json");
    assert!(payload.ready);
}

#[tokio::test]
async fn bootstrap_without_state_requires_selection() {
    let temp = tempdir().expect("tempdir");
    let app = build_app(AppState::for_tests(temp.path().to_path_buf()));

    let response = app
        .oneshot(Request::builder().uri("/api/bootstrap").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: BootstrapResult = serde_json::from_slice(&body).expect("json");
    assert_eq!(payload.mode, BootstrapMode::SelectionRequired);
    assert!(payload.remembered_context.is_none());
}

#[tokio::test]
async fn bootstrap_with_auto_resume_returns_resume_requested() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_devices(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-device-1".to_string(),
            device_id: "DEVICE001".to_string(),
            friendly_name: "HDHomeRun DEVICE001".to_string(),
            base_url: "http://192.168.1.50".to_string(),
            device_auth: None,
            storage_url: None,
            lineup_url: Some("http://192.168.1.50/lineup.json".to_string()),
            tuner_count: 4,
            is_legacy: false,
        }],
    );
    state
        .state_store()
        .save_context(&RememberedContext {
            device_ref: Some("hdhr-device-1".to_string()),
            channel_ref: Some("5.1".to_string()),
            auto_resume: true,
            updated_at: "2026-05-20T23:32:16Z".to_string(),
        })
        .expect("save state");

    let app = build_app(state);

    let response = app
        .oneshot(Request::builder().uri("/api/bootstrap").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: BootstrapResult = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.mode, BootstrapMode::ResumeRequested);
    assert!(payload.remembered_context.is_some());
}

#[tokio::test]
async fn bootstrap_clears_missing_remembered_device() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_devices(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-device-2".to_string(),
            device_id: "DEVICE002".to_string(),
            friendly_name: "HDHomeRun DEVICE002".to_string(),
            base_url: "http://192.168.1.51".to_string(),
            device_auth: None,
            storage_url: None,
            lineup_url: Some("http://192.168.1.51/lineup.json".to_string()),
            tuner_count: 2,
            is_legacy: false,
        }],
    );
    state
        .state_store()
        .save_context(&RememberedContext {
            device_ref: Some("hdhr-missing".to_string()),
            channel_ref: Some("5.1".to_string()),
            auto_resume: true,
            updated_at: "2026-05-20T23:32:16Z".to_string(),
        })
        .expect("save state");

    let app = build_app(state.clone());
    let response = app
        .oneshot(Request::builder().uri("/api/bootstrap").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: BootstrapResult = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.mode, BootstrapMode::SelectionRequired);
    assert!(payload.remembered_context.is_none());
    assert!(state.state_store().load_context().expect("load state").is_none());
}