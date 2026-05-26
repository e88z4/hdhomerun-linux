use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use serde_json::{Value, json};

use crate::models::{
    ContractEndpointStatus, LineupChannel, PlaybackCommandResponse, PlaybackCurrentResponse,
    PlaybackMode, PlaybackRecordingSummary, PlaybackSessionState, PlaybackSessionStatus,
    PlayerAdapterState, PlayerAdapterStatus, RetryablePlaybackFailure,
};

const MPV_START_TIMEOUT: Duration = Duration::from_secs(2);
const MPV_COMMAND_TIMEOUT: Duration = Duration::from_secs(3);
const MPV_RETRY_LIMIT: u8 = 1;
const MPV_SOCKET_FILE: &str = "playback-mpv.sock";
const MPV_BIN_ENV: &str = "HDHR_BACKEND_MPV_BIN";
const PLAYER_MODE_ENV: &str = "HDHR_BACKEND_PLAYER_MODE";

pub type SharedPlaybackService = Arc<PlaybackService>;
pub type SharedPlayerAdapter = Arc<dyn PlayerAdapter>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlayerAdapterError {
    pub code: String,
    pub message: String,
    pub retryable: bool,
}

impl PlayerAdapterError {
    pub fn new(code: impl Into<String>, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            retryable,
        }
    }
}

pub trait PlayerAdapter: Send + Sync {
    fn state(&self) -> PlayerAdapterState;
    fn preflight_check(&self) -> Result<(), PlayerAdapterError> {
        Ok(())
    }
    fn ensure_ready(&self) -> Result<PlayerAdapterState, PlayerAdapterError>;
    fn load_stream(&self, playback_url: &str) -> Result<PlayerAdapterState, PlayerAdapterError>;
    fn stop_stream(&self) -> Result<PlayerAdapterState, PlayerAdapterError>;
    fn rebuild(&self) -> Result<PlayerAdapterState, PlayerAdapterError>;
}

pub struct PlaybackService {
    adapter: SharedPlayerAdapter,
    runtime: Mutex<PlaybackRuntime>,
}

#[derive(Clone)]
struct PlaybackRuntime {
    session_state: PlaybackSessionState,
    current_channel: Option<LineupChannel>,
    current_recording: Option<PlaybackRecordingSummary>,
    warnings: Vec<String>,
    failure: Option<RetryablePlaybackFailure>,
}

impl Default for PlaybackRuntime {
    fn default() -> Self {
        Self {
            session_state: PlaybackSessionState::idle(),
            current_channel: None,
            current_recording: None,
            warnings: Vec::new(),
            failure: None,
        }
    }
}

impl PlaybackService {
    pub fn shared_default(state_dir: PathBuf) -> SharedPlaybackService {
        match env::var(PLAYER_MODE_ENV)
            .ok()
            .map(|value| value.trim().to_ascii_lowercase())
            .as_deref()
        {
            Some("client") | Some("client_managed") | Some("embedded") => {
                Self::shared_with_adapter(ClientManagedAdapter::shared())
            }
            _ => Self::shared_native(state_dir),
        }
    }

    pub fn shared_native(state_dir: PathBuf) -> SharedPlaybackService {
        Self::shared_with_adapter(NativeMpvAdapter::shared(state_dir))
    }

    pub fn shared_with_adapter(adapter: SharedPlayerAdapter) -> SharedPlaybackService {
        Arc::new(Self {
            adapter,
            runtime: Mutex::new(PlaybackRuntime::default()),
        })
    }

    pub fn current(&self) -> PlaybackCurrentResponse {
        let runtime = self.runtime.lock().expect("playback runtime lock");
        let mut warnings = runtime.warnings.clone();
        if runtime.failure.is_none() {
            if let Err(error) = self.adapter.preflight_check() {
                push_warning(&mut warnings, error.message);
            }
        }

        PlaybackCurrentResponse {
            status: ContractEndpointStatus::Available,
            session_state: runtime.session_state.clone(),
            adapter_state: self.adapter.state(),
            current_channel: runtime.current_channel.clone(),
            current_recording: runtime.current_recording.clone(),
            selected_device_ref: runtime.session_state.selected_device_ref.clone(),
            warnings,
            failure: runtime.failure.clone(),
        }
    }

    pub fn start(&self, device_ref: String, channel: LineupChannel) -> PlaybackCommandResponse {
        self.run_live_playback_command(device_ref, channel, false)
    }

    pub fn switch_channel(&self, device_ref: String, channel: LineupChannel) -> PlaybackCommandResponse {
        self.run_live_playback_command(device_ref, channel, true)
    }

    pub fn start_recording(
        &self,
        device_ref: String,
        recording: PlaybackRecordingSummary,
        playback_url: String,
    ) -> PlaybackCommandResponse {
        self.run_source_command(device_ref, None, Some(recording), playback_url, PlaybackMode::Recorded, false)
    }

    pub fn stop(&self) -> PlaybackCommandResponse {
        let mut runtime = self.runtime.lock().expect("playback runtime lock");
        let was_recorded = runtime.session_state.playback_mode == PlaybackMode::Recorded;

        let adapter_state = match self.adapter.stop_stream() {
            Ok(state) => state,
            Err(error) => {
                runtime.session_state.status = PlaybackSessionStatus::Failed;
                runtime.session_state.warning = Some(error.message.clone());
                runtime.session_state.updated_at = timestamp_now();
                runtime.failure = Some(build_failure(
                    &error,
                    false,
                    runtime.session_state.selected_device_ref.clone(),
                    runtime.session_state.channel_ref.clone(),
                ));
                runtime.warnings = vec![error.message.clone()];

                return build_command_response(&runtime, self.adapter.state(), false);
            }
        };

        runtime.session_state.status = PlaybackSessionStatus::Stopped;
        runtime.session_state.playback_mode = PlaybackMode::Idle;
        runtime.session_state.playback_url = None;
        runtime.session_state.warning = None;
        runtime.session_state.updated_at = timestamp_now();
        runtime.failure = None;
        runtime.warnings.clear();
        if was_recorded {
            runtime.current_recording = None;
        }

        build_command_response(&runtime, adapter_state, false)
    }

    fn run_live_playback_command(
        &self,
        device_ref: String,
        channel: LineupChannel,
        is_switch: bool,
    ) -> PlaybackCommandResponse {
        let playback_url = channel.playback_url.clone().unwrap_or_default();
        self.run_source_command(
            device_ref,
            Some(channel),
            None,
            playback_url,
            PlaybackMode::Live,
            is_switch,
        )
    }

    fn run_source_command(
        &self,
        device_ref: String,
        channel: Option<LineupChannel>,
        recording: Option<PlaybackRecordingSummary>,
        playback_url: String,
        playback_mode: PlaybackMode,
        is_switch: bool,
    ) -> PlaybackCommandResponse {
        let mut runtime = self.runtime.lock().expect("playback runtime lock");
        let has_existing_session = runtime.session_state.session_id.is_some()
            && matches!(
                runtime.session_state.status,
                PlaybackSessionStatus::Playing
                    | PlaybackSessionStatus::Switching
                    | PlaybackSessionStatus::Starting
                    | PlaybackSessionStatus::RetryingStart
            );

        runtime.session_state.session_id = if is_switch && has_existing_session {
            runtime.session_state.session_id.clone()
        } else {
            Some(new_session_id())
        };
        runtime.session_state.status = if is_switch && has_existing_session {
            PlaybackSessionStatus::Switching
        } else {
            PlaybackSessionStatus::Starting
        };
        runtime.session_state.playback_mode = playback_mode;
        runtime.session_state.selected_device_ref = Some(device_ref.clone());
        runtime.session_state.channel_ref = channel
            .as_ref()
            .map(|channel| channel.channel_ref.clone())
            .or_else(|| recording.as_ref().map(|recording| recording.recording_id.clone()));
        runtime.session_state.playback_url = Some(playback_url.clone());
        runtime.session_state.retry_count = 0;
        runtime.session_state.warning = None;
        runtime.session_state.updated_at = timestamp_now();
        runtime.current_channel = channel.clone();
        runtime.current_recording = recording.clone();
        runtime.warnings.clear();
        runtime.failure = None;
        let mut used_automatic_retry = false;

        let adapter_state = match self
            .adapter
            .preflight_check()
            .and_then(|_| self.adapter.ensure_ready())
            .and_then(|_| self.adapter.load_stream(&playback_url))
        {
            Ok(state) => state,
            Err(error) if error.retryable => {
                used_automatic_retry = true;
                runtime.session_state.status = PlaybackSessionStatus::RetryingStart;
                runtime.session_state.retry_count = MPV_RETRY_LIMIT;
                runtime.session_state.warning = Some("playback succeeded after one bounded automatic retry".to_string());
                runtime.session_state.updated_at = timestamp_now();

                match self.adapter.rebuild().and_then(|_| self.adapter.load_stream(&playback_url)) {
                    Ok(state) => state,
                    Err(retry_error) => {
                        runtime.session_state.status = PlaybackSessionStatus::Failed;
                        runtime.session_state.warning = Some(retry_error.message.clone());
                        runtime.session_state.updated_at = timestamp_now();
                        runtime.failure = Some(build_failure(
                            &retry_error,
                            true,
                            Some(device_ref.clone()),
                            channel
                                .as_ref()
                                .map(|channel| channel.channel_ref.clone())
                                .or_else(|| recording.as_ref().map(|recording| recording.recording_id.clone())),
                        ));
                        runtime.warnings = vec![retry_error.message.clone()];

                        return build_command_response(&runtime, self.adapter.state(), true);
                    }
                }
            }
            Err(error) => {
                runtime.session_state.status = PlaybackSessionStatus::Failed;
                runtime.session_state.warning = Some(error.message.clone());
                runtime.session_state.updated_at = timestamp_now();
                runtime.failure = Some(build_failure(
                    &error,
                    false,
                    Some(device_ref.clone()),
                    channel
                        .as_ref()
                        .map(|channel| channel.channel_ref.clone())
                        .or_else(|| recording.as_ref().map(|recording| recording.recording_id.clone())),
                ));
                runtime.warnings = vec![error.message.clone()];

                return build_command_response(&runtime, self.adapter.state(), false);
            }
        };

        runtime.session_state.status = PlaybackSessionStatus::Playing;
        runtime.session_state.retry_count = u8::from(used_automatic_retry);
        runtime.session_state.warning = if used_automatic_retry {
            Some("playback succeeded after one bounded automatic retry".to_string())
        } else {
            None
        };
        runtime.session_state.updated_at = timestamp_now();
        runtime.failure = None;
        runtime.warnings = if used_automatic_retry {
            vec!["playback succeeded after one bounded automatic retry".to_string()]
        } else {
            Vec::new()
        };

        build_command_response(&runtime, adapter_state, used_automatic_retry)
    }
}

fn build_command_response(
    runtime: &PlaybackRuntime,
    adapter_state: PlayerAdapterState,
    used_automatic_retry: bool,
) -> PlaybackCommandResponse {
    PlaybackCommandResponse {
        status: ContractEndpointStatus::Available,
        session_state: runtime.session_state.clone(),
        adapter_state,
        current_channel: runtime.current_channel.clone(),
            current_recording: runtime.current_recording.clone(),
        selected_device_ref: runtime.session_state.selected_device_ref.clone(),
        used_automatic_retry,
        warnings: runtime.warnings.clone(),
        failure: runtime.failure.clone(),
    }
}

fn build_failure(
    error: &PlayerAdapterError,
    retry_consumed: bool,
    device_ref: Option<String>,
    channel_ref: Option<String>,
) -> RetryablePlaybackFailure {
    RetryablePlaybackFailure {
        code: error.code.clone(),
        message: error.message.clone(),
        retryable: error.retryable,
        retry_consumed,
        channel_ref,
        device_ref,
    }
}

fn new_session_id() -> String {
    format!("session-{}", timestamp_now())
}

fn timestamp_now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis().to_string())
        .unwrap_or_else(|_| "0".to_string())
}

pub struct NativeMpvAdapter {
    program: String,
    socket_path: PathBuf,
    inner: Mutex<NativeMpvAdapterInner>,
}

pub struct ClientManagedAdapter {
    state: Mutex<PlayerAdapterState>,
}

impl ClientManagedAdapter {
    pub fn shared() -> SharedPlayerAdapter {
        Arc::new(Self {
            state: Mutex::new(PlayerAdapterState {
                adapter_status: PlayerAdapterStatus::AdapterReady,
                process_id: None,
                last_command: Some("client_managed_ready".to_string()),
                last_error: None,
                updated_at: timestamp_now(),
            }),
        })
    }
}

impl PlayerAdapter for ClientManagedAdapter {
    fn state(&self) -> PlayerAdapterState {
        self.state.lock().expect("client managed adapter lock").clone()
    }

    fn ensure_ready(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut state = self.state.lock().expect("client managed adapter lock");
        *state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: None,
            last_command: Some("client_managed_ready".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(state.clone())
    }

    fn load_stream(&self, _playback_url: &str) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut state = self.state.lock().expect("client managed adapter lock");
        *state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterStreaming,
            process_id: None,
            last_command: Some("client_managed_load".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(state.clone())
    }

    fn stop_stream(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut state = self.state.lock().expect("client managed adapter lock");
        *state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: None,
            last_command: Some("client_managed_stop".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(state.clone())
    }

    fn rebuild(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut state = self.state.lock().expect("client managed adapter lock");
        *state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: None,
            last_command: Some("client_managed_rebuild".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(state.clone())
    }
}

struct NativeMpvAdapterInner {
    child: Option<Child>,
    state: PlayerAdapterState,
}

impl NativeMpvAdapter {
    pub fn shared(state_dir: PathBuf) -> SharedPlayerAdapter {
        let socket_dir = state_dir.join("runtime");
        Arc::new(Self {
            program: resolve_mpv_program(),
            socket_path: socket_dir.join(MPV_SOCKET_FILE),
            inner: Mutex::new(NativeMpvAdapterInner {
                child: None,
                state: PlayerAdapterState::not_started(),
            }),
        })
    }

    fn spawn_process(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        if let Some(parent) = self.socket_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                PlayerAdapterError::new(
                    "adapter_start_failed",
                    format!("failed to create playback runtime directory: {error}"),
                    false,
                )
            })?;
        }

        let _ = fs::remove_file(&self.socket_path);

        let child = Command::new(&self.program)
            .arg("--idle=yes")
            .arg("--force-window=yes")
            .arg("--keep-open=no")
            .arg("--terminal=no")
            .arg(format!("--input-ipc-server={}", self.socket_path.display()))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|error| {
                PlayerAdapterError::new(
                    "adapter_start_failed",
                    format!("failed to start {}: {error}", self.program),
                    false,
                )
            })?;

        let process_id = child.id();
        {
            let mut inner = self.inner.lock().expect("native mpv adapter lock");
            inner.child = Some(child);
            inner.state = PlayerAdapterState {
                adapter_status: PlayerAdapterStatus::ProcessStarting,
                process_id: Some(process_id),
                last_command: None,
                last_error: None,
                updated_at: timestamp_now(),
            };
        }

        self.wait_for_socket(process_id)
    }

    fn wait_for_socket(&self, process_id: u32) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let deadline = std::time::Instant::now() + MPV_START_TIMEOUT;

        while std::time::Instant::now() < deadline {
            if self.socket_path.exists() && self.send_command(json!({"command": ["get_property", "idle-active"]}), None).is_ok() {
                let state = PlayerAdapterState {
                    adapter_status: PlayerAdapterStatus::AdapterReady,
                    process_id: Some(process_id),
                    last_command: Some("get_property idle-active".to_string()),
                    last_error: None,
                    updated_at: timestamp_now(),
                };
                let mut inner = self.inner.lock().expect("native mpv adapter lock");
                inner.state = state.clone();
                return Ok(state);
            }

            if let Some(state) = self.check_process_exit() {
                return Err(PlayerAdapterError::new(
                    "adapter_start_failed",
                    state.last_error.unwrap_or_else(|| "mpv exited before it became ready".to_string()),
                    true,
                ));
            }

            thread::sleep(Duration::from_millis(50));
        }

        Err(PlayerAdapterError::new(
            "adapter_start_timeout",
            "mpv did not expose its IPC socket before the startup timeout".to_string(),
            true,
        ))
    }

    fn send_command(&self, command: Value, expected_event: Option<&str>) -> Result<(), PlayerAdapterError> {
        let mut stream = UnixStream::connect(&self.socket_path).map_err(|error| {
            PlayerAdapterError::new(
                "adapter_ipc_unavailable",
                format!("failed to connect to mpv IPC socket: {error}"),
                true,
            )
        })?;
        stream
            .set_read_timeout(Some(MPV_COMMAND_TIMEOUT))
            .map_err(|error| PlayerAdapterError::new("adapter_ipc_unavailable", format!("failed to set IPC read timeout: {error}"), true))?;
        stream
            .set_write_timeout(Some(MPV_COMMAND_TIMEOUT))
            .map_err(|error| PlayerAdapterError::new("adapter_ipc_unavailable", format!("failed to set IPC write timeout: {error}"), true))?;

        let payload = serde_json::to_vec(&command).map_err(|error| {
            PlayerAdapterError::new(
                "adapter_command_invalid",
                format!("failed to serialize mpv IPC command: {error}"),
                false,
            )
        })?;
        stream.write_all(&payload).map_err(|error| {
            PlayerAdapterError::new(
                "adapter_ipc_write_failed",
                format!("failed to write mpv IPC command: {error}"),
                true,
            )
        })?;
        stream.write_all(b"\n").map_err(|error| {
            PlayerAdapterError::new(
                "adapter_ipc_write_failed",
                format!("failed to terminate mpv IPC command: {error}"),
                true,
            )
        })?;
        stream.flush().map_err(|error| {
            PlayerAdapterError::new(
                "adapter_ipc_write_failed",
                format!("failed to flush mpv IPC command: {error}"),
                true,
            )
        })?;

        let mut reader = BufReader::new(stream);
        let mut saw_success_reply = false;

        loop {
            let mut line = String::new();
            let bytes = reader.read_line(&mut line).map_err(|error| {
                PlayerAdapterError::new(
                    "adapter_ipc_read_failed",
                    format!("failed to read mpv IPC response: {error}"),
                    true,
                )
            })?;
            if bytes == 0 {
                return Err(PlayerAdapterError::new(
                    "adapter_ipc_closed",
                    "mpv IPC connection closed unexpectedly".to_string(),
                    true,
                ));
            }

            let value: Value = serde_json::from_str(line.trim()).map_err(|error| {
                PlayerAdapterError::new(
                    "adapter_ipc_invalid_response",
                    format!("failed to parse mpv IPC response: {error}"),
                    true,
                )
            })?;

            if let Some(event_name) = value.get("event").and_then(Value::as_str) {
                if expected_event.is_some_and(|expected| expected == event_name) {
                    return Ok(());
                }
                continue;
            }

            if let Some(error_name) = value.get("error").and_then(Value::as_str) {
                if error_name != "success" {
                    return Err(PlayerAdapterError::new(
                        "adapter_command_failed",
                        format!("mpv rejected the playback command: {error_name}"),
                        true,
                    ));
                }

                saw_success_reply = true;
                if expected_event.is_none() {
                    return Ok(());
                }
            }

            if saw_success_reply && expected_event.is_none() {
                return Ok(());
            }
        }
    }

    fn shutdown_process(&self) {
        let mut inner = self.inner.lock().expect("native mpv adapter lock");
        if let Some(mut child) = inner.child.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
        let _ = fs::remove_file(&self.socket_path);
        inner.state = PlayerAdapterState::not_started();
    }

    fn check_process_exit(&self) -> Option<PlayerAdapterState> {
        let mut inner = self.inner.lock().expect("native mpv adapter lock");
        let child = inner.child.as_mut()?;
        match child.try_wait() {
            Ok(Some(status)) => {
                let state = PlayerAdapterState {
                    adapter_status: PlayerAdapterStatus::AdapterError,
                    process_id: Some(child.id()),
                    last_command: inner.state.last_command.clone(),
                    last_error: Some(format!("mpv exited unexpectedly with status {status}")),
                    updated_at: timestamp_now(),
                };
                inner.child = None;
                inner.state = state.clone();
                Some(state)
            }
            Ok(None) | Err(_) => None,
        }
    }
}

impl PlayerAdapter for NativeMpvAdapter {
    fn state(&self) -> PlayerAdapterState {
        if let Some(state) = self.check_process_exit() {
            return state;
        }

        let inner = self.inner.lock().expect("native mpv adapter lock");
        inner.state.clone()
    }

    fn preflight_check(&self) -> Result<(), PlayerAdapterError> {
        let current_state = {
            let inner = self.inner.lock().expect("native mpv adapter lock");
            inner.state.clone()
        };

        if current_state.process_id.is_some() {
            return Ok(());
        }

        if find_program_on_path(&self.program).is_some() {
            return Ok(());
        }

        Err(PlayerAdapterError::new(
            "player_dependency_missing",
            format!(
                "{} executable is not available; install mpv or set {} to a valid executable path",
                self.program, MPV_BIN_ENV
            ),
            false,
        ))
    }

    fn ensure_ready(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        self.preflight_check()?;

        if let Some(state) = self.check_process_exit() {
            return Err(PlayerAdapterError::new(
                "adapter_exited",
                state.last_error.unwrap_or_else(|| "mpv exited unexpectedly".to_string()),
                true,
            ));
        }

        let current_state = {
            let inner = self.inner.lock().expect("native mpv adapter lock");
            inner.state.clone()
        };

        if current_state.process_id.is_some() && self.socket_path.exists() {
            self.send_command(json!({"command": ["get_property", "idle-active"]}), None)?;
            let mut inner = self.inner.lock().expect("native mpv adapter lock");
            inner.state.adapter_status = PlayerAdapterStatus::AdapterReady;
            inner.state.last_command = Some("get_property idle-active".to_string());
            inner.state.last_error = None;
            inner.state.updated_at = timestamp_now();
            return Ok(inner.state.clone());
        }

        self.spawn_process()
    }

    fn load_stream(&self, playback_url: &str) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let ready_state = self.ensure_ready()?;
        {
            let mut inner = self.inner.lock().expect("native mpv adapter lock");
            inner.state = PlayerAdapterState {
                adapter_status: PlayerAdapterStatus::AdapterLoadingStream,
                process_id: ready_state.process_id,
                last_command: Some("loadfile replace".to_string()),
                last_error: None,
                updated_at: timestamp_now(),
            };
        }

        self.send_command(json!({"command": ["loadfile", playback_url, "replace"]}), Some("file-loaded"))?;

        let mut inner = self.inner.lock().expect("native mpv adapter lock");
        inner.state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterStreaming,
            process_id: ready_state.process_id,
            last_command: Some("loadfile replace".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }

    fn stop_stream(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let current_state = self.state();
        if current_state.process_id.is_none() {
            return Ok(current_state);
        }

        self.send_command(json!({"command": ["stop"]}), None)?;
        let mut inner = self.inner.lock().expect("native mpv adapter lock");
        inner.state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: current_state.process_id,
            last_command: Some("stop".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }

    fn rebuild(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        self.shutdown_process();
        self.ensure_ready()
    }
}

#[derive(Clone, Debug)]
pub struct StaticPlayerAdapterFixtures {
    pub initial_state: PlayerAdapterState,
    pub preflight_error: Option<PlayerAdapterError>,
    pub ensure_ready_results: Vec<Result<PlayerAdapterState, PlayerAdapterError>>,
    pub load_stream_results: Vec<Result<PlayerAdapterState, PlayerAdapterError>>,
    pub stop_stream_results: Vec<Result<PlayerAdapterState, PlayerAdapterError>>,
    pub rebuild_results: Vec<Result<PlayerAdapterState, PlayerAdapterError>>,
}

impl Default for StaticPlayerAdapterFixtures {
    fn default() -> Self {
        Self {
            initial_state: PlayerAdapterState::not_started(),
            preflight_error: None,
            ensure_ready_results: Vec::new(),
            load_stream_results: Vec::new(),
            stop_stream_results: Vec::new(),
            rebuild_results: Vec::new(),
        }
    }
}

#[derive(Clone)]
pub struct StaticPlayerAdapter {
    inner: Arc<Mutex<StaticPlayerAdapterInner>>,
}

struct StaticPlayerAdapterInner {
    state: PlayerAdapterState,
    next_process_id: u32,
    preflight_error: Option<PlayerAdapterError>,
    ensure_ready_results: VecDeque<Result<PlayerAdapterState, PlayerAdapterError>>,
    load_stream_results: VecDeque<Result<PlayerAdapterState, PlayerAdapterError>>,
    stop_stream_results: VecDeque<Result<PlayerAdapterState, PlayerAdapterError>>,
    rebuild_results: VecDeque<Result<PlayerAdapterState, PlayerAdapterError>>,
}

impl StaticPlayerAdapter {
    pub fn shared(fixtures: StaticPlayerAdapterFixtures) -> SharedPlayerAdapter {
        Arc::new(Self {
            inner: Arc::new(Mutex::new(StaticPlayerAdapterInner {
                state: fixtures.initial_state,
                next_process_id: 4100,
                preflight_error: fixtures.preflight_error,
                ensure_ready_results: fixtures.ensure_ready_results.into(),
                load_stream_results: fixtures.load_stream_results.into(),
                stop_stream_results: fixtures.stop_stream_results.into(),
                rebuild_results: fixtures.rebuild_results.into(),
            })),
        })
    }

    fn pop_result(
        queue: &mut VecDeque<Result<PlayerAdapterState, PlayerAdapterError>>,
    ) -> Option<Result<PlayerAdapterState, PlayerAdapterError>> {
        queue.pop_front()
    }

    fn ensure_process_id(inner: &mut StaticPlayerAdapterInner) -> u32 {
        match inner.state.process_id {
            Some(process_id) => process_id,
            None => {
                let process_id = inner.next_process_id;
                inner.next_process_id += 1;
                process_id
            }
        }
    }
}

impl PlayerAdapter for StaticPlayerAdapter {
    fn state(&self) -> PlayerAdapterState {
        self.inner.lock().expect("static player adapter lock").state.clone()
    }

    fn preflight_check(&self) -> Result<(), PlayerAdapterError> {
        let inner = self.inner.lock().expect("static player adapter lock");
        inner.preflight_error.clone().map_or(Ok(()), Err)
    }

    fn ensure_ready(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut inner = self.inner.lock().expect("static player adapter lock");
        if let Some(result) = Self::pop_result(&mut inner.ensure_ready_results) {
            match result {
                Ok(state) => {
                    inner.state = state.clone();
                    return Ok(state);
                }
                Err(error) => {
                    inner.state.adapter_status = PlayerAdapterStatus::AdapterError;
                    inner.state.last_error = Some(error.message.clone());
                    inner.state.updated_at = timestamp_now();
                    return Err(error);
                }
            }
        }

        let process_id = Self::ensure_process_id(&mut inner);
        inner.state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: Some(process_id),
            last_command: Some("ensure_ready".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }

    fn load_stream(&self, _playback_url: &str) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut inner = self.inner.lock().expect("static player adapter lock");
        if let Some(result) = Self::pop_result(&mut inner.load_stream_results) {
            match result {
                Ok(state) => {
                    inner.state = state.clone();
                    return Ok(state);
                }
                Err(error) => {
                    inner.state.adapter_status = PlayerAdapterStatus::AdapterError;
                    inner.state.last_error = Some(error.message.clone());
                    inner.state.updated_at = timestamp_now();
                    return Err(error);
                }
            }
        }

        let process_id = Self::ensure_process_id(&mut inner);
        inner.state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterStreaming,
            process_id: Some(process_id),
            last_command: Some("load_stream".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }

    fn stop_stream(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut inner = self.inner.lock().expect("static player adapter lock");
        if let Some(result) = Self::pop_result(&mut inner.stop_stream_results) {
            match result {
                Ok(state) => {
                    inner.state = state.clone();
                    return Ok(state);
                }
                Err(error) => {
                    inner.state.adapter_status = PlayerAdapterStatus::AdapterError;
                    inner.state.last_error = Some(error.message.clone());
                    inner.state.updated_at = timestamp_now();
                    return Err(error);
                }
            }
        }

        inner.state = PlayerAdapterState {
            adapter_status: if inner.state.process_id.is_some() {
                PlayerAdapterStatus::AdapterReady
            } else {
                PlayerAdapterStatus::NotStarted
            },
            process_id: inner.state.process_id,
            last_command: Some("stop".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }

    fn rebuild(&self) -> Result<PlayerAdapterState, PlayerAdapterError> {
        let mut inner = self.inner.lock().expect("static player adapter lock");
        if let Some(result) = Self::pop_result(&mut inner.rebuild_results) {
            match result {
                Ok(state) => {
                    inner.state = state.clone();
                    return Ok(state);
                }
                Err(error) => {
                    inner.state.adapter_status = PlayerAdapterStatus::AdapterError;
                    inner.state.last_error = Some(error.message.clone());
                    inner.state.updated_at = timestamp_now();
                    return Err(error);
                }
            }
        }

        let process_id = inner.next_process_id;
        inner.next_process_id += 1;
        inner.state = PlayerAdapterState {
            adapter_status: PlayerAdapterStatus::AdapterReady,
            process_id: Some(process_id),
            last_command: Some("rebuild".to_string()),
            last_error: None,
            updated_at: timestamp_now(),
        };
        Ok(inner.state.clone())
    }
}

fn push_warning(warnings: &mut Vec<String>, warning: String) {
    if !warnings.iter().any(|existing| existing == &warning) {
        warnings.push(warning);
    }
}

fn resolve_mpv_program() -> String {
    env::var(MPV_BIN_ENV)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "mpv".to_string())
}

fn find_program_on_path(program: &str) -> Option<PathBuf> {
    let program_path = Path::new(program);
    if program_path.components().count() > 1 {
        return is_executable_file(program_path).then(|| program_path.to_path_buf());
    }

    env::var_os("PATH").and_then(|path_value| {
        env::split_paths(&path_value)
            .map(|path| path.join(program))
            .find(|candidate| is_executable_file(candidate))
    })
}

fn is_executable_file(path: &Path) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file() && metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn sample_channel() -> LineupChannel {
        LineupChannel {
            channel_ref: "channel:5.1".to_string(),
            guide_number: "5.1".to_string(),
            guide_name: "News".to_string(),
            current_program_title: None,
            image_url: None,
            tags: Vec::new(),
            playback_url: Some("http://127.0.0.1/auto/v5.1".to_string()),
            availability: crate::models::ChannelAvailability::Playable,
            restriction_reason: None,
        }
    }

    #[test]
    fn shared_default_uses_client_managed_adapter_when_env_requests_it() {
        let _guard = env_guard().lock().expect("env guard");
        let previous = env::var(PLAYER_MODE_ENV).ok();
        unsafe {
            env::set_var(PLAYER_MODE_ENV, "client");
        }

        let service = PlaybackService::shared_default(std::env::temp_dir());
        let current = service.current();

        match previous {
            Some(value) => unsafe {
                env::set_var(PLAYER_MODE_ENV, value);
            },
            None => unsafe {
                env::remove_var(PLAYER_MODE_ENV);
            },
        }

        assert_eq!(current.adapter_state.adapter_status, PlayerAdapterStatus::AdapterReady);
        assert_eq!(current.adapter_state.process_id, None);
        assert_eq!(current.adapter_state.last_command.as_deref(), Some("client_managed_ready"));
    }

    #[test]
    fn client_managed_adapter_streams_without_external_process() {
        let service = PlaybackService::shared_with_adapter(ClientManagedAdapter::shared());

        let response = service.start("hdhr-1234abcd".to_string(), sample_channel());

        assert_eq!(response.session_state.status, PlaybackSessionStatus::Playing);
        assert_eq!(response.adapter_state.adapter_status, PlayerAdapterStatus::AdapterStreaming);
        assert_eq!(response.adapter_state.process_id, None);
        assert_eq!(response.adapter_state.last_command.as_deref(), Some("client_managed_load"));
        assert!(response.failure.is_none());
    }
}