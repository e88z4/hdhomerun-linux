use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackendRuntimeStatus {
    Stopped,
    Starting,
    Ready,
    Unhealthy,
    FailedToStart,
}

impl BackendRuntimeStatus {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LaunchMode {
    BundledAutoStart,
    ManagedService,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BackendRuntimeState {
    pub status: BackendRuntimeStatus,
    pub started_at: Option<String>,
    pub last_health_check_at: Option<String>,
    pub launch_mode: LaunchMode,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RememberedContext {
    pub device_ref: Option<String>,
    pub channel_ref: Option<String>,
    pub auto_resume: bool,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BootstrapMode {
    SelectionRequired,
    RestoredContext,
    ResumeRequested,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContractEndpointStatus {
    Available,
    Provisional,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContractEndpointDescriptor {
    pub name: String,
    pub path: String,
    pub status: ContractEndpointStatus,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapResult {
    pub mode: BootstrapMode,
    pub remembered_context: Option<RememberedContext>,
    pub available_contract_endpoints: Vec<ContractEndpointDescriptor>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatus {
    pub ready: bool,
    pub status: BackendRuntimeStatus,
    pub service_version: String,
    pub api_version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeStateResponse {
    pub runtime: BackendRuntimeState,
    pub remembered_context: Option<RememberedContext>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
    pub retry_hint: String,
    pub details: Option<String>,
}

impl ApiErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>, retry_hint: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retry_hint: retry_hint.into(),
            details: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSummary {
    pub device_ref: String,
    pub device_id: String,
    pub name: String,
    pub base_url: String,
    pub lineup_url: Option<String>,
    pub tuner_count: u8,
    pub is_legacy: bool,
    pub is_selected: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DevicesResponse {
    pub status: ContractEndpointStatus,
    pub devices: Vec<DeviceSummary>,
    pub selected_device_ref: Option<String>,
    pub selection_required: bool,
    pub warnings: Vec<String>,
}

impl DevicesResponse {
    pub fn provisional() -> Self {
        Self {
            status: ContractEndpointStatus::Provisional,
            devices: Vec::new(),
            selected_device_ref: None,
            selection_required: true,
            warnings: vec!["device discovery is not implemented in Unit 1".to_string()],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceSelectionRequest {
    pub device_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelAvailability {
    Playable,
    Restricted,
    Unavailable,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LineupChannel {
    pub channel_ref: String,
    pub guide_number: String,
    pub guide_name: String,
    pub tags: Vec<String>,
    pub playback_url: Option<String>,
    pub availability: ChannelAvailability,
    pub restriction_reason: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum LineupState {
    Ready,
    Stale,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LineupResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: LineupState,
    pub channels: Vec<LineupChannel>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TunerDiagnosticsState {
    Ready,
    Partial,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TunerDiagnostic {
    pub tuner_index: u8,
    pub is_active_context: bool,
    pub channel: Option<String>,
    pub virtual_channel: Option<String>,
    pub program_name: Option<String>,
    pub lock_state: Option<String>,
    pub signal_present: bool,
    pub signal_strength: Option<u32>,
    pub signal_to_noise_quality: Option<u32>,
    pub symbol_error_quality: Option<u32>,
    pub bits_per_second: Option<u32>,
    pub packets_per_second: Option<u32>,
    pub availability: String,
    pub warning: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TunerDiagnosticsResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: TunerDiagnosticsState,
    pub tuners: Vec<TunerDiagnostic>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackSessionStatus {
    Idle,
    Starting,
    RetryingStart,
    Playing,
    Switching,
    Stopped,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PlayerAdapterStatus {
    NotStarted,
    ProcessStarting,
    AdapterReady,
    AdapterLoadingStream,
    AdapterStreaming,
    AdapterError,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackSessionState {
    pub session_id: Option<String>,
    pub status: PlaybackSessionStatus,
    pub selected_device_ref: Option<String>,
    pub channel_ref: Option<String>,
    pub playback_url: Option<String>,
    pub retry_count: u8,
    pub warning: Option<String>,
    pub updated_at: String,
}

impl PlaybackSessionState {
    pub fn idle() -> Self {
        Self {
            session_id: None,
            status: PlaybackSessionStatus::Idle,
            selected_device_ref: None,
            channel_ref: None,
            playback_url: None,
            retry_count: 0,
            warning: None,
            updated_at: "boot".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAdapterState {
    pub adapter_status: PlayerAdapterStatus,
    pub process_id: Option<u32>,
    pub last_command: Option<String>,
    pub last_error: Option<String>,
    pub updated_at: String,
}

impl PlayerAdapterState {
    pub fn not_started() -> Self {
        Self {
            adapter_status: PlayerAdapterStatus::NotStarted,
            process_id: None,
            last_command: None,
            last_error: None,
            updated_at: "boot".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RetryablePlaybackFailure {
    pub code: String,
    pub message: String,
    pub retryable: bool,
    pub retry_consumed: bool,
    pub channel_ref: Option<String>,
    pub device_ref: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackCommandRequest {
    pub device_ref: Option<String>,
    pub channel_ref: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackCurrentResponse {
    pub status: ContractEndpointStatus,
    pub session_state: PlaybackSessionState,
    pub adapter_state: PlayerAdapterState,
    pub current_channel: Option<LineupChannel>,
    pub selected_device_ref: Option<String>,
    pub warnings: Vec<String>,
    pub failure: Option<RetryablePlaybackFailure>,
}

impl PlaybackCurrentResponse {
    pub fn idle() -> Self {
        Self {
            status: ContractEndpointStatus::Available,
            session_state: PlaybackSessionState::idle(),
            adapter_state: PlayerAdapterState::not_started(),
            current_channel: None,
            selected_device_ref: None,
            warnings: Vec::new(),
            failure: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackCommandResponse {
    pub status: ContractEndpointStatus,
    pub session_state: PlaybackSessionState,
    pub adapter_state: PlayerAdapterState,
    pub current_channel: Option<LineupChannel>,
    pub selected_device_ref: Option<String>,
    pub used_automatic_retry: bool,
    pub warnings: Vec<String>,
    pub failure: Option<RetryablePlaybackFailure>,
}