use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use tokio::task;

use crate::app::AppState;
use crate::device::{build_devices_response, reconcile_remembered_context};
use crate::error::AppError;
use crate::models::{
    BootstrapMode, BootstrapResult, ContractEndpointDescriptor, ContractEndpointStatus,
    DevicesResponse, HealthStatus, LineupResponse, LineupState, PlaybackCurrentResponse,
    RuntimeStateResponse, TunerDiagnostic, TunerDiagnosticsResponse, TunerDiagnosticsState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/state", get(runtime_state))
        .route("/api/bootstrap", get(bootstrap))
        .route("/api/devices", get(devices))
        .route("/api/lineup", get(lineup))
        .route("/api/tuners", get(tuners))
        .route("/api/playback/current", get(playback_current))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<HealthStatus> {
    let runtime = state.runtime_state().await;
    Json(HealthStatus {
        ready: runtime.status.is_ready(),
        status: runtime.status,
        service_version: state.service_version().to_string(),
        api_version: state.api_version().to_string(),
    })
}

async fn runtime_state(State(state): State<AppState>) -> Result<Json<RuntimeStateResponse>, AppError> {
    let runtime = state.runtime_state().await;
    let remembered_context = state.state_store().load_context()?;

    Ok(Json(RuntimeStateResponse {
        runtime,
        remembered_context,
    }))
}

async fn bootstrap(State(state): State<AppState>) -> Result<Json<BootstrapResult>, AppError> {
    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(&state).await?;
    let (remembered_context, cleared_stale_device) =
        reconcile_remembered_context(remembered_context, &discovered_devices);

    if cleared_stale_device {
        state.state_store().clear_context()?;
    }

    let mode = match remembered_context.as_ref() {
        Some(context) if context.auto_resume => BootstrapMode::ResumeRequested,
        Some(_) => BootstrapMode::RestoredContext,
        None => BootstrapMode::SelectionRequired,
    };

    let mut warnings = Vec::new();
    if cleared_stale_device {
        warnings.push("remembered device was not available and the saved device context was cleared".to_string());
    }
    if discovered_devices.is_empty() {
        warnings.push("no HDHomeRun tuner devices are currently reachable on the local network".to_string());
    }

    Ok(Json(BootstrapResult {
        mode,
        remembered_context,
        available_contract_endpoints: available_contract_endpoints(),
        warnings,
    }))
}

async fn devices(State(state): State<AppState>) -> Result<Json<DevicesResponse>, AppError> {
    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(&state).await?;

    Ok(Json(build_devices_response(
        discovered_devices,
        remembered_context.as_ref(),
    )))
}

async fn lineup(State(state): State<AppState>) -> Result<Json<LineupResponse>, AppError> {
    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(&state).await?;
    let (remembered_context, cleared_stale_device) =
        reconcile_remembered_context(remembered_context, &discovered_devices);

    if cleared_stale_device {
        state.state_store().clear_context()?;
    }

    let selected_device = remembered_context.as_ref().and_then(|context| {
        context
            .device_ref
            .as_deref()
            .and_then(|device_ref| discovered_devices.iter().find(|device| device.device_ref == device_ref))
            .cloned()
    });

    let Some(selected_device) = selected_device else {
        return Ok(Json(LineupResponse {
            status: ContractEndpointStatus::Available,
            selected_device_ref: None,
            state: LineupState::SelectionRequired,
            channels: Vec::new(),
            warnings: vec!["select a discovered device before requesting a lineup".to_string()],
        }));
    };

    match fetch_lineup(&state, selected_device.clone()).await {
        Ok(channels) => {
            state
                .store_cached_lineup(selected_device.device_ref.clone(), channels.clone())
                .await;

            Ok(Json(LineupResponse {
                status: ContractEndpointStatus::Available,
                selected_device_ref: Some(selected_device.device_ref),
                state: LineupState::Ready,
                channels,
                warnings: Vec::new(),
            }))
        }
        Err(error) => {
            if let Some(channels) = state.cached_lineup(&selected_device.device_ref).await {
                return Ok(Json(LineupResponse {
                    status: ContractEndpointStatus::Available,
                    selected_device_ref: Some(selected_device.device_ref),
                    state: LineupState::Stale,
                    channels,
                    warnings: vec![format!("lineup refresh failed and the last successful lineup is being reused: {error}")],
                }));
            }

            Ok(Json(LineupResponse {
                status: ContractEndpointStatus::Available,
                selected_device_ref: Some(selected_device.device_ref),
                state: LineupState::Unavailable,
                channels: Vec::new(),
                warnings: vec!["lineup is currently unavailable for the selected device".to_string()],
            }))
        }
    }
}

async fn tuners(State(state): State<AppState>) -> Result<Json<TunerDiagnosticsResponse>, AppError> {
    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(&state).await?;
    let (remembered_context, cleared_stale_device) =
        reconcile_remembered_context(remembered_context, &discovered_devices);

    if cleared_stale_device {
        state.state_store().clear_context()?;
    }

    let selected_device = remembered_context.as_ref().and_then(|context| {
        context
            .device_ref
            .as_deref()
            .and_then(|device_ref| discovered_devices.iter().find(|device| device.device_ref == device_ref))
            .cloned()
    });

    let Some(selected_device) = selected_device else {
        return Ok(Json(TunerDiagnosticsResponse {
            status: ContractEndpointStatus::Available,
            selected_device_ref: None,
            state: TunerDiagnosticsState::SelectionRequired,
            tuners: Vec::new(),
            warnings: vec!["select a discovered device before requesting tuner diagnostics".to_string()],
        }));
    };

    let results = fetch_tuner_diagnostics(&state, selected_device.clone(), remembered_context.as_ref()).await?;
    let mut tuners = Vec::new();
    let mut warnings = Vec::new();
    let mut had_success = false;
    let mut had_failure = false;

    for (index, result) in results.into_iter().enumerate() {
        match result {
            Ok(mut tuner) => {
                if !tuner.is_active_context {
                    tuner.is_active_context = remembered_context
                        .as_ref()
                        .and_then(|context| context.channel_ref.as_deref())
                        .and_then(|channel_ref| tuner.virtual_channel.as_deref().map(|value| value == channel_ref))
                        .unwrap_or(false);
                }
                had_success = true;
                tuners.push(tuner);
            }
            Err(message) => {
                had_failure = true;
                warnings.push(message.clone());
                tuners.push(TunerDiagnostic {
                    tuner_index: index as u8,
                    is_active_context: false,
                    channel: None,
                    virtual_channel: None,
                    program_name: None,
                    lock_state: None,
                    signal_present: false,
                    signal_strength: None,
                    signal_to_noise_quality: None,
                    symbol_error_quality: None,
                    bits_per_second: None,
                    packets_per_second: None,
                    availability: "unavailable".to_string(),
                    warning: Some(message),
                });
            }
        }
    }

    let state_value = match (had_success, had_failure) {
        (true, false) => TunerDiagnosticsState::Ready,
        (true, true) => TunerDiagnosticsState::Partial,
        (false, true) | (false, false) => TunerDiagnosticsState::Unavailable,
    };

    Ok(Json(TunerDiagnosticsResponse {
        status: ContractEndpointStatus::Available,
        selected_device_ref: Some(selected_device.device_ref),
        state: state_value,
        tuners,
        warnings,
    }))
}

async fn playback_current() -> Json<PlaybackCurrentResponse> {
    Json(PlaybackCurrentResponse::provisional())
}

fn available_contract_endpoints() -> Vec<ContractEndpointDescriptor> {
    vec![
        ContractEndpointDescriptor {
            name: "health".to_string(),
            path: "/api/health".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "state".to_string(),
            path: "/api/state".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "bootstrap".to_string(),
            path: "/api/bootstrap".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "devices".to_string(),
            path: "/api/devices".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "lineup".to_string(),
            path: "/api/lineup".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "tuners".to_string(),
            path: "/api/tuners".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "playbackCurrent".to_string(),
            path: "/api/playback/current".to_string(),
            status: ContractEndpointStatus::Provisional,
        },
    ]
}

async fn discover_devices(state: &AppState) -> Result<Vec<crate::device::DiscoveredDevice>, AppError> {
    let discovery = state.device_discovery();
    task::spawn_blocking(move || discovery.discover())
        .await
        .map_err(|error| AppError::internal(format!("device discovery task failed: {error}")))?
}

async fn fetch_lineup(
    state: &AppState,
    device: crate::device::DiscoveredDevice,
) -> Result<Vec<crate::models::LineupChannel>, AppError> {
    let provider = state.lineup_provider();
    task::spawn_blocking(move || provider.lineup_for(&device))
        .await
        .map_err(|error| AppError::internal(format!("lineup task failed: {error}")))?
}

async fn fetch_tuner_diagnostics(
    state: &AppState,
    device: crate::device::DiscoveredDevice,
    remembered_context: Option<&crate::models::RememberedContext>,
) -> Result<Vec<Result<TunerDiagnostic, String>>, AppError> {
    let provider = state.tuner_diagnostics_provider();
    let remembered_context = remembered_context.cloned();
    task::spawn_blocking(move || provider.diagnostics_for(&device, remembered_context.as_ref()))
        .await
        .map_err(|error| AppError::internal(format!("tuner diagnostics task failed: {error}")))?
}