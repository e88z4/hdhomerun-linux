use std::collections::HashMap;

use axum::body::Body;
use axum::http::Request;
use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::device::DiscoveredDevice;
use hdhomerun_backend::models::{
    ContractEndpointStatus, RememberedContext, TunerDiagnostic, TunerDiagnosticsResponse,
    TunerDiagnosticsState,
};
use http_body_util::BodyExt;
use tempfile::tempdir;
use tower::util::ServiceExt;

#[tokio::test]
async fn tuners_endpoint_returns_partial_results_when_one_tuner_fails() {
    let temp = tempdir().expect("tempdir");
    let mut diagnostics = HashMap::new();
    diagnostics.insert(
        "hdhr-1234abcd".to_string(),
        vec![
            Ok(TunerDiagnostic {
                tuner_index: 0,
                is_active_context: true,
                channel: Some("auto:5".to_string()),
                virtual_channel: Some("5.1".to_string()),
                program_name: Some("News".to_string()),
                lock_state: Some("8vsb".to_string()),
                signal_present: true,
                signal_strength: Some(95),
                signal_to_noise_quality: Some(100),
                symbol_error_quality: Some(100),
                bits_per_second: Some(19300000),
                packets_per_second: Some(2100),
                availability: "available".to_string(),
                warning: None,
            }),
            Err("failed to read status for tuner 1".to_string()),
        ],
    );
    let state = AppState::for_tests_with_fixtures(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 2,
            is_legacy: false,
        }],
        HashMap::new(),
        diagnostics,
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
        .oneshot(Request::builder().uri("/api/tuners").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: TunerDiagnosticsResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.status, ContractEndpointStatus::Available);
    assert_eq!(payload.state, TunerDiagnosticsState::Partial);
    assert_eq!(payload.tuners.len(), 2);
    assert_eq!(payload.tuners[0].virtual_channel.as_deref(), Some("5.1"));
    assert_eq!(payload.tuners[1].availability, "unavailable");
}

#[tokio::test]
async fn tuners_endpoint_requires_selected_device() {
    let temp = tempdir().expect("tempdir");
    let state = AppState::for_tests_with_fixtures(
        temp.path().to_path_buf(),
        vec![DiscoveredDevice {
            device_ref: "hdhr-1234abcd".to_string(),
            device_id: "1234ABCD".to_string(),
            friendly_name: "HDHomeRun 1234ABCD".to_string(),
            base_url: "http://192.168.1.10".to_string(),
            lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
            tuner_count: 2,
            is_legacy: false,
        }],
        HashMap::new(),
        HashMap::new(),
    );

    let app = build_app(state);
    let response = app
        .oneshot(Request::builder().uri("/api/tuners").body(Body::empty()).unwrap())
        .await
        .expect("response");

    let body = response.into_body().collect().await.expect("body").to_bytes();
    let payload: TunerDiagnosticsResponse = serde_json::from_slice(&body).expect("json");

    assert_eq!(payload.state, TunerDiagnosticsState::SelectionRequired);
    assert!(payload.tuners.is_empty());
}