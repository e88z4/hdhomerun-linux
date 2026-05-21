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
#[serde(rename_all = "camelCase")]
pub struct PlaybackSessionSummary {
    pub channel_ref: Option<String>,
    pub state: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackCurrentResponse {
    pub status: ContractEndpointStatus,
    pub session: Option<PlaybackSessionSummary>,
    pub warnings: Vec<String>,
}

impl PlaybackCurrentResponse {
    pub fn provisional() -> Self {
        Self {
            status: ContractEndpointStatus::Provisional,
            session: None,
            warnings: vec!["playback session reporting is not implemented in Unit 1".to_string()],
        }
    }
}