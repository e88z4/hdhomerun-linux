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
    pub current_program_title: Option<String>,
    pub image_url: Option<String>,
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
pub enum GuideState {
    Ready,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GuideEntry {
    pub start_time: i64,
    pub end_time: i64,
    pub title: String,
    pub episode_title: Option<String>,
    pub synopsis: Option<String>,
    pub image_url: Option<String>,
    pub series_id: Option<String>,
    pub is_current: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GuideChannel {
    pub channel_ref: String,
    pub guide_number: String,
    pub guide_name: String,
    pub current_program_title: Option<String>,
    pub image_url: Option<String>,
    pub entries: Vec<GuideEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GuideResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: GuideState,
    pub window_start: i64,
    pub duration_hours: u8,
    pub channels: Vec<GuideChannel>,
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
pub enum PlaybackMode {
    Idle,
    Live,
    Recorded,
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
    pub playback_mode: PlaybackMode,
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
            playback_mode: PlaybackMode::Idle,
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
    pub current_recording: Option<PlaybackRecordingSummary>,
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
            current_recording: None,
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
    pub current_recording: Option<PlaybackRecordingSummary>,
    pub selected_device_ref: Option<String>,
    pub used_automatic_retry: bool,
    pub warnings: Vec<String>,
    pub failure: Option<RetryablePlaybackFailure>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlaybackRecordingSummary {
    pub recording_id: String,
    pub title: String,
    pub episode_title: Option<String>,
    pub image_url: Option<String>,
    pub channel_name: Option<String>,
    pub record_start_time: i64,
    pub record_end_time: i64,
    pub resume_position: i64,
    pub watched: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrReadinessState {
    Ready,
    Degraded,
    NotReady,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrReadinessConditionSeverity {
    Info,
    Warning,
    Blocking,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrReadinessConditionCode {
    MissingDeviceAuth,
    MissingStorage,
    RecordEngineUnavailable,
    UpstreamError,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrReadinessCondition {
    pub code: DvrReadinessConditionCode,
    pub severity: DvrReadinessConditionSeverity,
    pub message: String,
    pub recoverable: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrReadinessResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: DvrReadinessState,
    pub usable: bool,
    pub conditions: Vec<DvrReadinessCondition>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRuleKind {
    Series,
    OneTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct DvrRuleOptions {
    #[serde(default)]
    pub channel_only: Vec<String>,
    #[serde(default)]
    pub team_only: Vec<String>,
    #[serde(default)]
    pub recent_only: bool,
    pub after_original_airdate_only: Option<i64>,
    pub start_padding: Option<u32>,
    pub end_padding: Option<u32>,
    #[serde(default)]
    pub unsupported_options: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateSeriesRecordingRuleRequest {
    pub series_id: String,
    pub title: Option<String>,
    #[serde(default)]
    pub options: DvrRuleOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CreateOneTimeRecordingRuleRequest {
    pub series_id: String,
    pub title: Option<String>,
    pub start_time: i64,
    pub channel_number: String,
    #[serde(default)]
    pub options: DvrRuleOptions,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRecordingRule {
    pub recording_rule_id: String,
    pub series_id: String,
    pub title: String,
    pub synopsis: Option<String>,
    pub image_url: Option<String>,
    pub kind: DvrRuleKind,
    pub channel_only: Vec<String>,
    pub team_only: Vec<String>,
    pub recent_only: bool,
    pub after_original_airdate_only: Option<i64>,
    pub date_time_only: Option<i64>,
    pub priority: Option<u32>,
    pub start_padding: u32,
    pub end_padding: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRulesState {
    Ready,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRulesResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: DvrRulesState,
    pub rules: Vec<DvrRecordingRule>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrUpcomingState {
    Ready,
    Degraded,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrUpcomingRecording {
    pub recording_rule_id: String,
    pub series_id: String,
    pub program_id: String,
    pub title: String,
    pub episode_number: Option<String>,
    pub episode_title: Option<String>,
    pub synopsis: Option<String>,
    pub image_url: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub record_start_time: i64,
    pub record_end_time: i64,
    pub channel_number: String,
    pub channel_name: String,
    pub channel_image_url: Option<String>,
    pub recording_rule_ext: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrScheduleProjectionState {
    Scheduled,
    NotScheduled,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrScheduleProjectionSource {
    ExplicitUpcoming,
    RuleContext,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrScheduleProjectionEntry {
    pub series_id: String,
    pub program_id: Option<String>,
    pub title: String,
    pub recording_rule_id: Option<String>,
    pub state: DvrScheduleProjectionState,
    pub reason: String,
    pub source: DvrScheduleProjectionSource,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrUpcomingResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: DvrUpcomingState,
    pub entries: Vec<DvrUpcomingRecording>,
    pub schedule_projection: Vec<DvrScheduleProjectionEntry>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRecordingsState {
    Ready,
    Degraded,
    Unavailable,
    SelectionRequired,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRecordingSummary {
    pub recording_id: String,
    pub title: String,
    pub episode_title: Option<String>,
    pub synopsis: Option<String>,
    pub image_url: Option<String>,
    pub channel_name: Option<String>,
    pub channel_number: Option<String>,
    pub record_start_time: i64,
    pub record_end_time: i64,
    pub resume_position: i64,
    pub watched: bool,
    pub source_count: u32,
    pub preferred_local: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRecordingsResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub state: DvrRecordingsState,
    pub recordings: Vec<DvrRecordingSummary>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRecordingDeleteOutcome {
    Confirmed,
    MissingRecording,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRecordingDeleteResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub outcome: DvrRecordingDeleteOutcome,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRuleDeleteOutcome {
    Confirmed,
    MissingRule,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRuleDeleteResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub outcome: DvrRuleDeleteOutcome,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DvrRuleMutationOutcome {
    Confirmed,
    InvalidAiring,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DvrRuleMutationResponse {
    pub status: ContractEndpointStatus,
    pub selected_device_ref: Option<String>,
    pub outcome: DvrRuleMutationOutcome,
    pub rules: Vec<DvrRecordingRule>,
    pub schedule_projection: Vec<DvrScheduleProjectionEntry>,
    pub warnings: Vec<String>,
}