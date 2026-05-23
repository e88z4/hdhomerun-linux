use axum::extract::{Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use tokio::task;

use crate::app::AppState;
use crate::device::{build_devices_response, reconcile_remembered_context};
use crate::error::AppError;
use crate::models::{
    BootstrapMode, BootstrapResult, ContractEndpointDescriptor, ContractEndpointStatus,
    DeviceSelectionRequest, DevicesResponse, GuideResponse, GuideState, HealthStatus, LineupChannel,
    LineupResponse, LineupState,
    PlaybackCommandRequest, PlaybackCommandResponse, PlaybackCurrentResponse,
    RememberedContext, RuntimeStateResponse, TunerDiagnostic, TunerDiagnosticsResponse,
    TunerDiagnosticsState,
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route("/api/state", get(runtime_state))
        .route("/api/bootstrap", get(bootstrap))
        .route("/api/devices", get(devices))
        .route("/api/devices/select", post(select_device))
        .route("/api/lineup", get(lineup))
        .route("/api/guide", get(guide))
        .route("/api/tuners", get(tuners))
        .route("/api/playback/current", get(playback_current))
        .route("/api/playback/start", post(playback_start))
        .route("/api/playback/stop", post(playback_stop))
        .route("/api/playback/retry", post(playback_retry))
        .route("/api/playback/switch", post(playback_switch_channel))
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

async fn select_device(
    State(state): State<AppState>,
    Json(request): Json<DeviceSelectionRequest>,
) -> Result<Json<DevicesResponse>, AppError> {
    if request.device_ref.trim().is_empty() {
        return Err(AppError::Validation(
            "deviceRef must not be empty when selecting a device".to_string(),
        ));
    }

    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(&state).await?;

    let device_exists = discovered_devices
        .iter()
        .any(|device| device.device_ref == request.device_ref);
    if !device_exists {
        return Err(AppError::Validation(
            "requested device is not currently available".to_string(),
        ));
    }

    let previous_device_ref = remembered_context
        .as_ref()
        .and_then(|context| context.device_ref.as_deref());
    let channel_ref = if previous_device_ref == Some(request.device_ref.as_str()) {
        remembered_context.as_ref().and_then(|context| context.channel_ref.clone())
    } else {
        None
    };

    let new_context = RememberedContext {
        device_ref: Some(request.device_ref.clone()),
        channel_ref,
        auto_resume: remembered_context
            .as_ref()
            .map(|context| context.auto_resume)
            .unwrap_or(false),
        updated_at: playback_timestamp_now(),
    };
    state.state_store().save_context(&new_context)?;

    let mut response = build_devices_response(discovered_devices, Some(&new_context));
    if previous_device_ref.is_some() && previous_device_ref != Some(request.device_ref.as_str()) {
        response
            .warnings
            .push("selected device changed and the remembered channel was cleared".to_string());
    }

    Ok(Json(response))
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
            let (channels, guide_warning) =
                enrich_lineup_with_current_programs(&state, selected_device.clone(), channels).await;
            state
                .store_cached_lineup(selected_device.device_ref.clone(), channels.clone())
                .await;

            let mut warnings = Vec::new();
            if let Some(guide_warning) = guide_warning {
                warnings.push(guide_warning);
            }

            Ok(Json(LineupResponse {
                status: ContractEndpointStatus::Available,
                selected_device_ref: Some(selected_device.device_ref),
                state: LineupState::Ready,
                channels,
                warnings,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuideQuery {
    start: Option<i64>,
    duration_hours: Option<u8>,
}

async fn guide(
    State(state): State<AppState>,
    Query(query): Query<GuideQuery>,
) -> Result<Json<GuideResponse>, AppError> {
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

    let duration_hours = query.duration_hours.unwrap_or(4).clamp(1, 24);
    let window_start = query.start.unwrap_or_else(playback_timestamp_unix_now);

    let Some(selected_device) = selected_device else {
        return Ok(Json(GuideResponse {
            status: ContractEndpointStatus::Available,
            selected_device_ref: None,
            state: GuideState::SelectionRequired,
            window_start,
            duration_hours,
            channels: Vec::new(),
            warnings: vec!["select a discovered device before requesting guide data".to_string()],
        }));
    };

    let mut warnings = Vec::new();
    let channels = match fetch_lineup(&state, selected_device.clone()).await {
        Ok(channels) => {
            state
                .store_cached_lineup(selected_device.device_ref.clone(), channels.clone())
                .await;
            channels
        }
        Err(error) => match state.cached_lineup(&selected_device.device_ref).await {
            Some(channels) => {
                warnings.push(format!(
                    "lineup refresh failed and the last successful lineup is being reused for guide data: {error}"
                ));
                channels
            }
            None => {
                return Ok(Json(GuideResponse {
                    status: ContractEndpointStatus::Available,
                    selected_device_ref: Some(selected_device.device_ref),
                    state: GuideState::Unavailable,
                    window_start,
                    duration_hours,
                    channels: Vec::new(),
                    warnings: vec!["guide data is currently unavailable for the selected device".to_string()],
                }));
            }
        },
    };

    let guide_provider = state.guide_provider();
    let guide_channels = match task::spawn_blocking(move || {
        guide_provider.schedule_for(&selected_device, &channels, window_start, duration_hours)
    })
    .await
    {
        Ok(Ok(guide_channels)) => guide_channels,
        Ok(Err(error)) => {
            warnings.push(format!("guide data is unavailable: {error}"));
            Vec::new()
        }
        Err(error) => {
            warnings.push(format!("guide lookup task failed: {error}"));
            Vec::new()
        }
    };

    let state_value = if guide_channels.is_empty() {
        GuideState::Unavailable
    } else {
        GuideState::Ready
    };

    Ok(Json(GuideResponse {
        status: ContractEndpointStatus::Available,
        selected_device_ref: remembered_context.and_then(|context| context.device_ref),
        state: state_value,
        window_start,
        duration_hours,
        channels: guide_channels,
        warnings,
    }))
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

async fn playback_current(State(state): State<AppState>) -> Result<Json<PlaybackCurrentResponse>, AppError> {
    let playback = state.playback_service();
    let response = task::spawn_blocking(move || playback.current())
        .await
        .map_err(|error| AppError::internal(format!("playback current task failed: {error}")))?;

    Ok(Json(response))
}

async fn playback_start(
    State(state): State<AppState>,
    Json(request): Json<PlaybackCommandRequest>,
) -> Result<Json<PlaybackCommandResponse>, AppError> {
    let target = resolve_playback_target(&state, &request).await?;
    let playback = state.playback_service();
    let device_ref = target.device_ref.clone();
    let channel = target.channel.clone();
    let response = task::spawn_blocking(move || playback.start(device_ref, channel))
        .await
        .map_err(|error| AppError::internal(format!("playback start task failed: {error}")))?;

    persist_playback_context(&state, &response, target.auto_resume)?;
    Ok(Json(response))
}

async fn playback_stop(State(state): State<AppState>) -> Result<Json<PlaybackCommandResponse>, AppError> {
    let auto_resume = state
        .state_store()
        .load_context()?
        .as_ref()
        .map(|context| context.auto_resume)
        .unwrap_or(false);

    let playback = state.playback_service();
    let response = task::spawn_blocking(move || playback.stop())
        .await
        .map_err(|error| AppError::internal(format!("playback stop task failed: {error}")))?;

    persist_playback_context(&state, &response, auto_resume)?;
    Ok(Json(response))
}

async fn playback_switch_channel(
    State(state): State<AppState>,
    Json(request): Json<PlaybackCommandRequest>,
) -> Result<Json<PlaybackCommandResponse>, AppError> {
    let target = resolve_playback_target(&state, &request).await?;
    let playback = state.playback_service();
    let device_ref = target.device_ref.clone();
    let channel = target.channel.clone();
    let response = task::spawn_blocking(move || playback.switch_channel(device_ref, channel))
        .await
        .map_err(|error| AppError::internal(format!("playback switch task failed: {error}")))?;

    persist_playback_context(&state, &response, target.auto_resume)?;
    Ok(Json(response))
}

async fn playback_retry(State(state): State<AppState>) -> Result<Json<PlaybackCommandResponse>, AppError> {
    let playback = state.playback_service();
    let current = task::spawn_blocking(move || playback.current())
        .await
        .map_err(|error| AppError::internal(format!("playback retry preflight task failed: {error}")))?;

    let device_ref = current
        .selected_device_ref
        .clone()
        .ok_or_else(|| AppError::Validation("no retryable playback context is currently available".to_string()))?;
    let channel_ref = current
        .current_channel
        .as_ref()
        .map(|channel| channel.channel_ref.clone())
        .ok_or_else(|| AppError::Validation("no retryable playback context is currently available".to_string()))?;

    let target = resolve_playback_target(
        &state,
        &PlaybackCommandRequest {
            device_ref: Some(device_ref),
            channel_ref,
        },
    )
    .await?;

    let playback = state.playback_service();
    let response = task::spawn_blocking(move || playback.start(target.device_ref.clone(), target.channel.clone()))
        .await
        .map_err(|error| AppError::internal(format!("playback retry task failed: {error}")))?;

    persist_playback_context(&state, &response, target.auto_resume)?;
    Ok(Json(response))
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
            name: "deviceSelect".to_string(),
            path: "/api/devices/select".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "lineup".to_string(),
            path: "/api/lineup".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "guide".to_string(),
            path: "/api/guide".to_string(),
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
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "playbackStart".to_string(),
            path: "/api/playback/start".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "playbackStop".to_string(),
            path: "/api/playback/stop".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "playbackRetry".to_string(),
            path: "/api/playback/retry".to_string(),
            status: ContractEndpointStatus::Available,
        },
        ContractEndpointDescriptor {
            name: "playbackSwitch".to_string(),
            path: "/api/playback/switch".to_string(),
            status: ContractEndpointStatus::Available,
        },
    ]
}

fn playback_timestamp_unix_now() -> i64 {
    chrono::Utc::now().timestamp()
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

async fn enrich_lineup_with_current_programs(
    state: &AppState,
    device: crate::device::DiscoveredDevice,
    channels: Vec<LineupChannel>,
) -> (Vec<LineupChannel>, Option<String>) {
    let guide_provider = state.guide_provider();
    let channels_for_lookup = channels.clone();
    let window_start = playback_timestamp_unix_now();

    let guide_channels = match task::spawn_blocking(move || {
        guide_provider.schedule_for(&device, &channels_for_lookup, window_start, 4)
    }).await {
        Ok(Ok(guide_channels)) => guide_channels,
        Ok(Err(error)) => return (channels, Some(format!("guide data is unavailable: {error}"))),
        Err(error) => return (channels, Some(format!("guide lookup task failed: {error}"))),
    };

    let channels = channels
        .into_iter()
        .map(|mut channel| {
            if let Some(guide_channel) = guide_channels.iter().find(|guide_channel| guide_channel.channel_ref == channel.channel_ref) {
                channel.current_program_title = guide_channel.current_program_title.clone();
                channel.image_url = guide_channel.image_url.clone();
            }
            channel
        })
        .collect();

    (channels, None)
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

struct ResolvedPlaybackTarget {
    device_ref: String,
    channel: LineupChannel,
    auto_resume: bool,
}

async fn resolve_playback_target(
    state: &AppState,
    request: &PlaybackCommandRequest,
) -> Result<ResolvedPlaybackTarget, AppError> {
    if request.channel_ref.trim().is_empty() {
        return Err(AppError::Validation(
            "channelRef must not be empty for playback commands".to_string(),
        ));
    }

    let remembered_context = state.state_store().load_context()?;
    let discovered_devices = discover_devices(state).await?;
    let (remembered_context, cleared_stale_device) =
        reconcile_remembered_context(remembered_context, &discovered_devices);

    if cleared_stale_device {
        state.state_store().clear_context()?;
    }

    let target_device_ref = request
        .device_ref
        .clone()
        .or_else(|| remembered_context.as_ref().and_then(|context| context.device_ref.clone()))
        .ok_or_else(|| AppError::Validation("select a discovered device before starting playback".to_string()))?;

    let device = discovered_devices
        .iter()
        .find(|device| device.device_ref == target_device_ref)
        .cloned()
        .ok_or_else(|| AppError::Validation("requested playback device is not currently available".to_string()))?;

    let channels = match fetch_lineup(state, device.clone()).await {
        Ok(channels) => {
            state
                .store_cached_lineup(device.device_ref.clone(), channels.clone())
                .await;
            channels
        }
        Err(error) => state
            .cached_lineup(&device.device_ref)
            .await
            .ok_or(error)?,
    };

    let channel = channels
        .into_iter()
        .find(|channel| channel_matches(channel, &request.channel_ref))
        .ok_or_else(|| AppError::Validation("requested channel was not present in the selected device lineup".to_string()))?;

    match channel.availability {
        crate::models::ChannelAvailability::Playable => {}
        crate::models::ChannelAvailability::Restricted => {
            return Err(AppError::Validation(
                channel
                    .restriction_reason
                    .clone()
                    .unwrap_or_else(|| "requested channel is not playable".to_string()),
            ));
        }
        crate::models::ChannelAvailability::Unavailable => {
            return Err(AppError::Validation(
                channel
                    .restriction_reason
                    .clone()
                    .unwrap_or_else(|| "requested channel does not have a usable playback URL".to_string()),
            ));
        }
    }

    Ok(ResolvedPlaybackTarget {
        device_ref: device.device_ref,
        channel,
        auto_resume: remembered_context
            .as_ref()
            .map(|context| context.auto_resume)
            .unwrap_or(false),
    })
}

fn channel_matches(channel: &LineupChannel, requested_channel_ref: &str) -> bool {
    channel.channel_ref == requested_channel_ref
        || channel.guide_number == requested_channel_ref
        || channel
            .channel_ref
            .strip_prefix("channel:")
            .is_some_and(|channel_ref| channel_ref == requested_channel_ref)
}

fn persist_playback_context(
    state: &AppState,
    response: &PlaybackCommandResponse,
    auto_resume: bool,
) -> Result<(), AppError> {
    if response.failure.is_some() {
        return Ok(());
    }

    let Some(device_ref) = response.selected_device_ref.clone() else {
        return Ok(());
    };
    let Some(channel) = response.current_channel.as_ref() else {
        return Ok(());
    };

    state.state_store().save_context(&RememberedContext {
        device_ref: Some(device_ref),
        channel_ref: Some(channel.guide_number.clone()),
        auto_resume,
        updated_at: playback_timestamp_now(),
    })
}

fn playback_timestamp_now() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string())
}