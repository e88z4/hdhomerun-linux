use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::models::{
    ContractEndpointStatus, DeviceSelectionRequest, DevicesResponse, RememberedContext,
};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn devices_endpoint_returns_discovered_devices_and_selected_context() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_devices(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 4,
            is_legacy: false,
        }],
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
        .oneshot(Request::builder().uri("/api/devices").body(Body::empty()).unwrap())
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: DevicesResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.status, ContractEndpointStatus::Available);
    assert_eq!(payload.devices.len(), 1);
    assert_eq!(payload.selected_device_ref.as_deref(), Some("hdhr-1234abcd"));
    assert!(!payload.selection_required);
    assert!(payload.devices[0].is_selected);
}

#[tokio::test]
async fn devices_endpoint_requires_selection_when_remembered_device_is_missing() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_devices(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 4,
            is_legacy: false,
        }],
    );
    state
        .state_store()
        .save_context(&RememberedContext {
            device_ref: Some("hdhr-deadbeef".to_string()),
            channel_ref: Some("7.2".to_string()),
            auto_resume: true,
            updated_at: "2026-05-20T23:32:16Z".to_string(),
        })
        .expect("save state");

    let app = build_app(state);
    let response = app
        .oneshot(Request::builder().uri("/api/devices").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: DevicesResponse = serde_json::from_slice(&body).expect("json");

    assert!(payload.selection_required);
    assert!(payload.selected_device_ref.is_none());
    assert_eq!(payload.warnings.len(), 1);
}

#[tokio::test]
async fn device_selection_endpoint_updates_selected_device_and_clears_stale_channel() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_devices(
        temp.path().to_path_buf(),
        vec![
            DiscoveredDevice {
                device_ref: "hdhr-1234abcd".to_string(),
                device_id: "1234ABCD".to_string(),
                friendly_name: "HDHomeRun 1234ABCD".to_string(),
                base_url: "http://192.168.1.10".to_string(),
                lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
                tuner_count: 4,
                is_legacy: false,
            },
            DiscoveredDevice {
                device_ref: "hdhr-5678ef01".to_string(),
                device_id: "5678EF01".to_string(),
                friendly_name: "HDHomeRun 5678EF01".to_string(),
                base_url: "http://192.168.1.11".to_string(),
                lineup_url: Some("http://192.168.1.11/lineup.json".to_string()),
                tuner_count: 2,
                is_legacy: false,
            },
        ],
    );
    state
        .state_store()
        .save_context(&RememberedContext {
            device_ref: Some("hdhr-1234abcd".to_string()),
            channel_ref: Some("5.1".to_string()),
            auto_resume: true,
            updated_at: "2026-05-20T23:32:16Z".to_string(),
        })
        .expect("save state");

    let app = build_app(state.clone());
    let request_body = serde_json::to_vec(&DeviceSelectionRequest {
        device_ref: "hdhr-5678ef01".to_string(),
    })
    .expect("serialize request");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/devices/select")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("response");

    assert!(response.status().is_success());
    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: DevicesResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.selected_device_ref.as_deref(), Some("hdhr-5678ef01"));
    assert!(payload.devices.iter().any(|device| device.device_ref == "hdhr-5678ef01" && device.is_selected));
    assert_eq!(payload.warnings.len(), 1);

    let context = state.state_store().load_context().expect("load state").expect("context");
    assert_eq!(context.device_ref.as_deref(), Some("hdhr-5678ef01"));
    assert!(context.channel_ref.is_none());
    assert!(context.auto_resume);
}