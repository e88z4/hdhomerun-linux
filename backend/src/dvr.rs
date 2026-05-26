use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde::Deserialize;

use crate::device::DiscoveredDevice;
use crate::error::AppError;
use crate::models::{
    CreateOneTimeRecordingRuleRequest, CreateSeriesRecordingRuleRequest, DvrReadinessCondition,
    DvrReadinessConditionCode, DvrReadinessConditionSeverity, DvrReadinessState,
    DvrRecordingDeleteOutcome, DvrRecordingSummary, DvrRecordingsState,
    DvrRecordingRule, DvrRuleDeleteOutcome, DvrRuleKind, DvrRuleMutationOutcome, DvrRuleOptions,
    DvrScheduleProjectionEntry, DvrScheduleProjectionSource, DvrScheduleProjectionState,
    DvrUpcomingRecording, DvrUpcomingState, PlaybackRecordingSummary,
};

const RECORDING_RULES_API_URL: &str = "https://api.hdhomerun.com/api/recording_rules";
const UPCOMING_RECORDINGS_API_URL: &str = "https://api.hdhomerun.com/api/upcoming_recordings";
const DVR_HTTP_TIMEOUT: Duration = Duration::from_secs(3);

pub type SharedDvrProvider = Arc<dyn DvrProvider>;

pub trait DvrProvider: Send + Sync {
    fn readiness_for(&self, device: &DiscoveredDevice) -> Result<DvrReadinessSnapshot, AppError>;

    fn list_rules(&self, device: &DiscoveredDevice) -> Result<Vec<DvrRecordingRule>, AppError>;

    fn recordings_for(
        &self,
        device: &DiscoveredDevice,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingsSnapshot, AppError>;

    fn playback_target_for(
        &self,
        device: &DiscoveredDevice,
        recording_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingPlaybackTarget, AppError>;

    fn delete_recording(
        &self,
        device: &DiscoveredDevice,
        recording_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingDeleteSnapshot, AppError>;

    fn create_series_rule(
        &self,
        device: &DiscoveredDevice,
        request: &CreateSeriesRecordingRuleRequest,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError>;

    fn create_one_time_rule(
        &self,
        device: &DiscoveredDevice,
        request: &CreateOneTimeRecordingRuleRequest,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError>;

    fn delete_rule(
        &self,
        device: &DiscoveredDevice,
        recording_rule_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleDeleteSnapshot, AppError>;

    fn upcoming_for(&self, device: &DiscoveredDevice) -> Result<DvrUpcomingSnapshot, AppError>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrReadinessSnapshot {
    pub state: DvrReadinessState,
    pub usable: bool,
    pub conditions: Vec<DvrReadinessCondition>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrUpcomingSnapshot {
    pub state: DvrUpcomingState,
    pub entries: Vec<DvrUpcomingRecording>,
    pub schedule_projection: Vec<DvrScheduleProjectionEntry>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrRecordingsSnapshot {
    pub state: DvrRecordingsState,
    pub recordings: Vec<DvrRecordingSummary>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrRecordingPlaybackTarget {
    pub recording: PlaybackRecordingSummary,
    pub playback_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrRecordingDeleteSnapshot {
    pub outcome: DvrRecordingDeleteOutcome,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrRuleMutationSnapshot {
    pub outcome: DvrRuleMutationOutcome,
    pub rules: Vec<DvrRecordingRule>,
    pub schedule_projection: Vec<DvrScheduleProjectionEntry>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DvrRuleDeleteSnapshot {
    pub outcome: DvrRuleDeleteOutcome,
    pub warnings: Vec<String>,
}

pub struct NativeDvrProvider;

impl NativeDvrProvider {
    pub fn shared() -> SharedDvrProvider {
        Arc::new(Self)
    }

    fn client() -> Result<Client, AppError> {
        Client::builder()
            .timeout(DVR_HTTP_TIMEOUT)
            .build()
            .map_err(|error| AppError::internal(format!("failed to build DVR client: {error}")))
    }

    fn device_auth(device: &DiscoveredDevice) -> Option<&str> {
        device
            .device_auth
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
    }

    fn validate_rule_options(options: &DvrRuleOptions, allow_multi_channel: bool) -> Result<(), AppError> {
        if !options.unsupported_options.is_empty() {
            return Err(AppError::Validation(format!(
                "unsupported DVR rule options were provided: {}",
                options.unsupported_options.join(", ")
            )));
        }

        if let Some(start_padding) = options.start_padding {
            if start_padding > 3600 {
                return Err(AppError::Validation(
                    "startPadding must be 3600 seconds or less".to_string(),
                ));
            }
        }

        if let Some(end_padding) = options.end_padding {
            if end_padding > 10_800 {
                return Err(AppError::Validation(
                    "endPadding must be 10800 seconds or less".to_string(),
                ));
            }
        }

        if !allow_multi_channel && options.channel_only.len() > 1 {
            return Err(AppError::Validation(
                "one-time DVR rules must target exactly one channel".to_string(),
            ));
        }

        if !allow_multi_channel
            && options
                .channel_only
                .iter()
                .any(|channel| channel.contains('|') || channel.trim().is_empty())
        {
            return Err(AppError::Validation(
                "one-time DVR rule channelOnly must be a single virtual channel number".to_string(),
            ));
        }

        Ok(())
    }

    pub(crate) fn validate_series_request(
        request: &CreateSeriesRecordingRuleRequest,
    ) -> Result<(), AppError> {
        if request.series_id.trim().is_empty() {
            return Err(AppError::Validation(
                "seriesId must not be empty for a series recording rule".to_string(),
            ));
        }

        Self::validate_rule_options(&request.options, true)
    }

    pub(crate) fn validate_one_time_request(
        request: &CreateOneTimeRecordingRuleRequest,
    ) -> Result<(), AppError> {
        if request.series_id.trim().is_empty() {
            return Err(AppError::Validation(
                "seriesId must not be empty for a one-time recording rule".to_string(),
            ));
        }

        if request.channel_number.trim().is_empty() {
            return Err(AppError::Validation(
                "channelNumber must not be empty for a one-time recording rule".to_string(),
            ));
        }

        Self::validate_rule_options(&request.options, false)
    }

    fn fetch_recording_rules(&self, device: &DiscoveredDevice) -> Result<Vec<DvrRecordingRule>, AppError> {
        let device_auth = Self::device_auth(device)
            .ok_or_else(|| AppError::internal("selected device did not expose a DeviceAuth value for DVR rule lookup"))?;

        let rules = Self::client()?
            .get(RECORDING_RULES_API_URL)
            .query(&[("DeviceAuth", device_auth)])
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("DVR recording-rules request failed: {error}")))?
            .json::<Option<Vec<RecordingRuleWire>>>()
            .map_err(|error| AppError::internal(format!("DVR recording-rules parsing failed: {error}")))?
            .unwrap_or_default();

        Ok(rules.into_iter().map(DvrRecordingRule::from).collect())
    }

    fn fetch_upcoming_recordings(&self, device: &DiscoveredDevice) -> Result<Vec<DvrUpcomingRecording>, AppError> {
        let device_auth = Self::device_auth(device)
            .ok_or_else(|| AppError::internal("selected device did not expose a DeviceAuth value for DVR upcoming lookup"))?;

        let entries = Self::client()?
            .get(UPCOMING_RECORDINGS_API_URL)
            .query(&[("DeviceAuth", device_auth)])
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("DVR upcoming-recordings request failed: {error}")))?
            .json::<Option<Vec<UpcomingRecordingWire>>>()
            .map_err(|error| AppError::internal(format!("DVR upcoming-recordings parsing failed: {error}")))?
            .unwrap_or_default();

        Ok(entries.into_iter().map(DvrUpcomingRecording::from).collect())
    }

    fn sync_recording_events(&self, discovered_devices: &[DiscoveredDevice]) -> Result<(), AppError> {
        let client = Self::client()?;

        for storage_url in discovered_devices
            .iter()
            .filter_map(|device| device.storage_url.as_deref())
            .filter_map(storage_sync_url)
        {
            client
                .post(storage_url)
                .send()
                .and_then(|response| response.error_for_status())
                .map_err(|error| AppError::internal(format!("DVR recording-events sync failed: {error}")))?;
        }

        Ok(())
    }

    fn submit_rule_form(&self, form: &[(String, String)]) -> Result<Vec<DvrRecordingRule>, AppError> {
        Self::client()?
            .post(RECORDING_RULES_API_URL)
            .form(form)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("DVR rule mutation request failed: {error}")))?
            .json::<Option<Vec<RecordingRuleWire>>>()
            .map_err(|error| AppError::internal(format!("DVR rule mutation parsing failed: {error}")))
            .map(|rules| rules.unwrap_or_default().into_iter().map(DvrRecordingRule::from).collect())
    }

    fn map_delete_response_status(status: StatusCode) -> AppError {
        if status == StatusCode::BAD_REQUEST {
            AppError::Validation(
                "the record engine rejected the delete command for this recording; the recording may no longer be deletable from this device"
                    .to_string(),
            )
        } else {
            AppError::internal(format!("DVR delete request failed with status {status}"))
        }
    }

    fn delete_command_url(cmd_url: &str) -> Result<reqwest::Url, AppError> {
        let url = reqwest::Url::parse(cmd_url)
            .map_err(|error| AppError::Validation(format!("the selected recording exposed an invalid delete target: {error}")))?;

        let existing_pairs = url
            .query_pairs()
            .filter(|(key, _)| key != "cmd" && key != "rerecord")
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect::<Vec<_>>();

        let mut rebuilt = url;
        rebuilt.set_query(None);
        {
            let mut query = rebuilt.query_pairs_mut();
            for (key, value) in existing_pairs {
                query.append_pair(&key, &value);
            }
            query.append_pair("cmd", "delete");
            query.append_pair("rerecord", "0");
        }

        Ok(rebuilt)
    }

    fn fetch_recording_entries(&self, url: &str) -> Result<Vec<RecordedFileWire>, AppError> {
        Self::client()?
            .get(url)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("DVR recorded-files request failed: {error}")))?
            .json::<Option<Vec<RecordedFileWire>>>()
            .map_err(|error| AppError::internal(format!("DVR recorded-files parsing failed: {error}")))
            .map(|entries| entries.unwrap_or_default())
    }

    fn fetch_recordings_from_source(&self, source: &RecordingSource) -> Result<Vec<RecordedCandidate>, AppError> {
        let entries = self.fetch_recording_entries(&source.storage_url)?;
        let entries = flatten_recording_entries(entries, |episodes_url| {
            self.fetch_recording_entries(episodes_url).ok()
        });

        Ok(entries
            .into_iter()
            .map(|entry| normalize_recorded_candidate(source, entry))
            .collect())
    }

    fn build_recordings_catalog(
        &self,
        device: &DiscoveredDevice,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingsCatalog, AppError> {
        let sources = recording_sources(device, discovered_devices);
        if sources.is_empty() {
            return Ok(DvrRecordingsCatalog {
                state: DvrRecordingsState::Unavailable,
                recordings: Vec::new(),
                warnings: vec!["No DVR storage sources are currently available for the selected device.".to_string()],
            });
        }

        let mut warnings = Vec::new();
        let mut candidates = Vec::new();
        let mut success_count = 0usize;
        let mut failure_count = 0usize;

        for source in &sources {
            match self.fetch_recordings_from_source(source) {
                Ok(mut source_candidates) => {
                    success_count += 1;
                    candidates.append(&mut source_candidates);
                }
                Err(error) => {
                    failure_count += 1;
                    warnings.push(format!(
                        "recordings from source {} are currently unavailable: {}",
                        source.source_ref, error
                    ));
                }
            }
        }

        let mut groups = Vec::<ResolvedRecording>::new();
        let mut positions = HashMap::<String, usize>::new();
        for candidate in candidates {
            if let Some(index) = positions.get(&candidate.recording_id).copied() {
                groups[index].variants.push(candidate);
            } else {
                positions.insert(candidate.recording_id.clone(), groups.len());
                groups.push(ResolvedRecording {
                    recording_id: candidate.recording_id.clone(),
                    variants: vec![candidate],
                });
            }
        }

        let mut recordings = groups
            .into_iter()
            .map(|group| group.into_summary())
            .collect::<Vec<_>>();
        recordings.sort_by(|left, right| {
            right
                .summary
                .record_start_time
                .cmp(&left.summary.record_start_time)
                .then_with(|| left.summary.title.cmp(&right.summary.title))
        });

        let state = match (success_count > 0, failure_count > 0) {
            (true, true) => DvrRecordingsState::Degraded,
            (true, false) => DvrRecordingsState::Ready,
            (false, _) => DvrRecordingsState::Unavailable,
        };

        Ok(DvrRecordingsCatalog {
            state,
            recordings,
            warnings,
        })
    }
}

impl DvrProvider for NativeDvrProvider {
    fn readiness_for(&self, device: &DiscoveredDevice) -> Result<DvrReadinessSnapshot, AppError> {
        let mut conditions = Vec::new();
        let mut warnings = Vec::new();

        if Self::device_auth(device).is_none() {
            conditions.push(DvrReadinessCondition {
                code: DvrReadinessConditionCode::MissingDeviceAuth,
                severity: DvrReadinessConditionSeverity::Blocking,
                message: "This tuner does not currently expose DeviceAuth for DVR guide APIs.".to_string(),
                recoverable: true,
            });
        }

        let storage_url = device.storage_url.as_deref().map(str::trim).filter(|value| !value.is_empty());
        if storage_url.is_none() {
            conditions.push(DvrReadinessCondition {
                code: DvrReadinessConditionCode::MissingStorage,
                severity: DvrReadinessConditionSeverity::Blocking,
                message: "No DVR storage endpoint is currently available for the selected device.".to_string(),
                recoverable: true,
            });
        }

        if let Some(storage_url) = storage_url {
            let probe_result = Self::client()?
                .get(storage_url)
                .send()
                .and_then(|response| response.error_for_status());

            if probe_result.is_err() {
                conditions.push(DvrReadinessCondition {
                    code: DvrReadinessConditionCode::RecordEngineUnavailable,
                    severity: DvrReadinessConditionSeverity::Blocking,
                    message: "The DVR record engine could not be reached from the selected storage endpoint.".to_string(),
                    recoverable: true,
                });
                warnings.push("DVR storage probe failed while evaluating readiness.".to_string());
            }
        }

        let usable = !conditions
            .iter()
            .any(|condition| matches!(condition.severity, DvrReadinessConditionSeverity::Blocking));

        Ok(DvrReadinessSnapshot {
            state: if usable {
                DvrReadinessState::Ready
            } else {
                DvrReadinessState::NotReady
            },
            usable,
            conditions,
            warnings,
        })
    }

    fn list_rules(&self, device: &DiscoveredDevice) -> Result<Vec<DvrRecordingRule>, AppError> {
        self.fetch_recording_rules(device)
    }

    fn recordings_for(
        &self,
        device: &DiscoveredDevice,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingsSnapshot, AppError> {
        let catalog = self.build_recordings_catalog(device, discovered_devices)?;
        Ok(DvrRecordingsSnapshot {
            state: catalog.state,
            recordings: catalog.recordings.iter().map(|recording| recording.summary.clone()).collect(),
            warnings: catalog.warnings,
        })
    }

    fn playback_target_for(
        &self,
        device: &DiscoveredDevice,
        recording_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingPlaybackTarget, AppError> {
        let catalog = self.build_recordings_catalog(device, discovered_devices)?;
        let Some(recording) = catalog.recordings.into_iter().find(|recording| recording.recording_id == recording_id) else {
            return Err(AppError::Validation(
                "requested recording is no longer available; refresh recordings and try again".to_string(),
            ));
        };

        let playback_url = recording.playback_url.clone().ok_or_else(|| {
            AppError::Validation(
                "the selected recording does not currently expose a playable target; refresh recordings and try again"
                    .to_string(),
            )
        })?;

        Ok(DvrRecordingPlaybackTarget {
            recording: PlaybackRecordingSummary {
                recording_id: recording.summary.recording_id.clone(),
                title: recording.summary.title.clone(),
                episode_title: recording.summary.episode_title.clone(),
                image_url: recording.summary.image_url.clone(),
                channel_name: recording.summary.channel_name.clone(),
                record_start_time: recording.summary.record_start_time,
                record_end_time: recording.summary.record_end_time,
                resume_position: recording.summary.resume_position,
                watched: recording.summary.watched,
            },
            playback_url,
        })
    }

    fn delete_recording(
        &self,
        device: &DiscoveredDevice,
        recording_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingDeleteSnapshot, AppError> {
        let catalog = self.build_recordings_catalog(device, discovered_devices)?;
        let Some(recording) = catalog.recordings.into_iter().find(|recording| recording.recording_id == recording_id) else {
            return Ok(DvrRecordingDeleteSnapshot {
                outcome: DvrRecordingDeleteOutcome::MissingRecording,
                warnings: vec!["The selected recording is no longer available; refresh recordings and try again.".to_string()],
            });
        };

        let cmd_url = recording.cmd_url.clone().ok_or_else(|| {
            AppError::Validation(
                "the selected recording does not currently expose a trusted delete target; refresh recordings and try again"
                    .to_string(),
            )
        })?;

        let delete_url = Self::delete_command_url(&cmd_url)?;

        let response = Self::client()?
            .post(delete_url)
            .send()
            .map_err(|error| AppError::internal(format!("DVR delete request failed: {error}")))?;

        if let Err(error) = response.error_for_status_ref() {
            let status = error.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            return Err(Self::map_delete_response_status(status));
        }

        self.sync_recording_events(discovered_devices)?;

        Ok(DvrRecordingDeleteSnapshot {
            outcome: DvrRecordingDeleteOutcome::Confirmed,
            warnings: Vec::new(),
        })
    }

    fn create_series_rule(
        &self,
        device: &DiscoveredDevice,
        request: &CreateSeriesRecordingRuleRequest,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError> {
        Self::validate_series_request(request)?;

        let device_auth = Self::device_auth(device)
            .ok_or_else(|| AppError::Validation("selected device cannot create DVR rules without DeviceAuth".to_string()))?;

        let mut form = vec![
            ("DeviceAuth".to_string(), device_auth.to_string()),
            ("Cmd".to_string(), "add".to_string()),
            ("SeriesID".to_string(), request.series_id.trim().to_string()),
        ];
        append_rule_options(&mut form, &request.options, true)?;

        let rules = self.submit_rule_form(&form)?;
        self.sync_recording_events(discovered_devices)?;
        let upcoming = self.fetch_upcoming_recordings(device)?;

        Ok(DvrRuleMutationSnapshot {
            outcome: DvrRuleMutationOutcome::Confirmed,
            schedule_projection: build_schedule_projection(&rules, &upcoming),
            rules,
            warnings: Vec::new(),
        })
    }

    fn create_one_time_rule(
        &self,
        device: &DiscoveredDevice,
        request: &CreateOneTimeRecordingRuleRequest,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError> {
        Self::validate_one_time_request(request)?;

        if request.start_time <= unix_timestamp_now() {
            return Ok(DvrRuleMutationSnapshot {
                outcome: DvrRuleMutationOutcome::InvalidAiring,
                rules: self.fetch_recording_rules(device).unwrap_or_default(),
                schedule_projection: Vec::new(),
                warnings: vec!["The selected airing is no longer valid for a one-time recording rule.".to_string()],
            });
        }

        let device_auth = Self::device_auth(device)
            .ok_or_else(|| AppError::Validation("selected device cannot create DVR rules without DeviceAuth".to_string()))?;

        let mut form = vec![
            ("DeviceAuth".to_string(), device_auth.to_string()),
            ("Cmd".to_string(), "add".to_string()),
            ("SeriesID".to_string(), request.series_id.trim().to_string()),
            ("DateTimeOnly".to_string(), request.start_time.to_string()),
            (
                "ChannelOnly".to_string(),
                request.channel_number.trim().to_string(),
            ),
        ];
        append_rule_options(&mut form, &request.options, false)?;

        let rules = self.submit_rule_form(&form)?;
        self.sync_recording_events(discovered_devices)?;
        let upcoming = self.fetch_upcoming_recordings(device)?;

        Ok(DvrRuleMutationSnapshot {
            outcome: DvrRuleMutationOutcome::Confirmed,
            schedule_projection: build_schedule_projection(&rules, &upcoming),
            rules,
            warnings: Vec::new(),
        })
    }

    fn delete_rule(
        &self,
        device: &DiscoveredDevice,
        recording_rule_id: &str,
        discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleDeleteSnapshot, AppError> {
        let recording_rule_id = recording_rule_id.trim();
        if recording_rule_id.is_empty() {
            return Err(AppError::Validation(
                "recording rule id must not be empty for DVR rule deletion".to_string(),
            ));
        }

        let existing_rules = self.fetch_recording_rules(device)?;
        if !existing_rules
            .iter()
            .any(|rule| rule.recording_rule_id == recording_rule_id)
        {
            return Ok(DvrRuleDeleteSnapshot {
                outcome: DvrRuleDeleteOutcome::MissingRule,
                warnings: vec!["The selected recording rule is no longer available; refresh DVR state and try again.".to_string()],
            });
        }

        let device_auth = Self::device_auth(device)
            .ok_or_else(|| AppError::Validation("selected device cannot delete DVR rules without DeviceAuth".to_string()))?;

        let form = vec![
            ("DeviceAuth".to_string(), device_auth.to_string()),
            ("Cmd".to_string(), "delete".to_string()),
            ("RecordingRuleID".to_string(), recording_rule_id.to_string()),
        ];
        self.submit_rule_form(&form)?;
        self.sync_recording_events(discovered_devices)?;

        Ok(DvrRuleDeleteSnapshot {
            outcome: DvrRuleDeleteOutcome::Confirmed,
            warnings: Vec::new(),
        })
    }

    fn upcoming_for(&self, device: &DiscoveredDevice) -> Result<DvrUpcomingSnapshot, AppError> {
        let rules = self.fetch_recording_rules(device)?;
        let entries = self.fetch_upcoming_recordings(device)?;
        let schedule_projection = build_schedule_projection(&rules, &entries);

        Ok(DvrUpcomingSnapshot {
            state: if entries.is_empty() {
                DvrUpcomingState::Unavailable
            } else {
                DvrUpcomingState::Ready
            },
            entries,
            schedule_projection,
            warnings: Vec::new(),
        })
    }
}

#[derive(Clone, Default)]
pub struct StaticDvrFixtures {
    pub readiness: Option<DvrReadinessSnapshot>,
    pub rules: Vec<DvrRecordingRule>,
    pub upcoming: Vec<DvrUpcomingRecording>,
    pub recordings: Vec<StaticDvrRecordingFixture>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StaticDvrRecordingFixture {
    pub summary: DvrRecordingSummary,
    pub playback_url: Option<String>,
    pub cmd_url: Option<String>,
}

#[derive(Clone)]
pub struct StaticDvrProvider {
    fixtures: Arc<Mutex<StaticDvrFixtures>>,
}

impl StaticDvrProvider {
    pub fn shared(fixtures: StaticDvrFixtures) -> SharedDvrProvider {
        Arc::new(Self {
            fixtures: Arc::new(Mutex::new(fixtures)),
        })
    }
}

impl DvrProvider for StaticDvrProvider {
    fn readiness_for(&self, _device: &DiscoveredDevice) -> Result<DvrReadinessSnapshot, AppError> {
        Ok(self
            .fixtures
            .lock()
            .expect("static DVR fixtures lock")
            .readiness
            .clone()
            .unwrap_or(DvrReadinessSnapshot {
                state: DvrReadinessState::Ready,
                usable: true,
                conditions: Vec::new(),
                warnings: Vec::new(),
            }))
    }

    fn list_rules(&self, _device: &DiscoveredDevice) -> Result<Vec<DvrRecordingRule>, AppError> {
        Ok(self.fixtures.lock().expect("static DVR fixtures lock").rules.clone())
    }

    fn recordings_for(
        &self,
        _device: &DiscoveredDevice,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingsSnapshot, AppError> {
        let fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        Ok(DvrRecordingsSnapshot {
            state: DvrRecordingsState::Ready,
            recordings: fixtures.recordings.iter().map(|recording| recording.summary.clone()).collect(),
            warnings: Vec::new(),
        })
    }

    fn playback_target_for(
        &self,
        _device: &DiscoveredDevice,
        recording_id: &str,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingPlaybackTarget, AppError> {
        let fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        let Some(recording) = fixtures.recordings.iter().find(|recording| recording.summary.recording_id == recording_id) else {
            return Err(AppError::Validation(
                "requested recording is no longer available; refresh recordings and try again".to_string(),
            ));
        };
        let playback_url = recording.playback_url.clone().ok_or_else(|| {
            AppError::Validation(
                "the selected recording does not currently expose a playable target; refresh recordings and try again"
                    .to_string(),
            )
        })?;

        Ok(DvrRecordingPlaybackTarget {
            recording: PlaybackRecordingSummary {
                recording_id: recording.summary.recording_id.clone(),
                title: recording.summary.title.clone(),
                episode_title: recording.summary.episode_title.clone(),
                image_url: recording.summary.image_url.clone(),
                channel_name: recording.summary.channel_name.clone(),
                record_start_time: recording.summary.record_start_time,
                record_end_time: recording.summary.record_end_time,
                resume_position: recording.summary.resume_position,
                watched: recording.summary.watched,
            },
            playback_url,
        })
    }

    fn delete_recording(
        &self,
        _device: &DiscoveredDevice,
        recording_id: &str,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRecordingDeleteSnapshot, AppError> {
        let mut fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        let original_len = fixtures.recordings.len();
        fixtures.recordings.retain(|recording| recording.summary.recording_id != recording_id);

        Ok(DvrRecordingDeleteSnapshot {
            outcome: if fixtures.recordings.len() == original_len {
                DvrRecordingDeleteOutcome::MissingRecording
            } else {
                DvrRecordingDeleteOutcome::Confirmed
            },
            warnings: if fixtures.recordings.len() == original_len {
                vec!["The selected recording is no longer available; refresh recordings and try again.".to_string()]
            } else {
                Vec::new()
            },
        })
    }

    fn create_series_rule(
        &self,
        _device: &DiscoveredDevice,
        request: &CreateSeriesRecordingRuleRequest,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError> {
        NativeDvrProvider::validate_series_request(request)?;
        let fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        Ok(DvrRuleMutationSnapshot {
            outcome: DvrRuleMutationOutcome::Confirmed,
            rules: fixtures.rules.clone(),
            schedule_projection: build_schedule_projection(&fixtures.rules, &fixtures.upcoming),
            warnings: Vec::new(),
        })
    }

    fn create_one_time_rule(
        &self,
        _device: &DiscoveredDevice,
        request: &CreateOneTimeRecordingRuleRequest,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleMutationSnapshot, AppError> {
        NativeDvrProvider::validate_one_time_request(request)?;
        let fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        Ok(DvrRuleMutationSnapshot {
            outcome: if request.start_time <= unix_timestamp_now() {
                DvrRuleMutationOutcome::InvalidAiring
            } else {
                DvrRuleMutationOutcome::Confirmed
            },
            rules: fixtures.rules.clone(),
            schedule_projection: build_schedule_projection(&fixtures.rules, &fixtures.upcoming),
            warnings: Vec::new(),
        })
    }

    fn delete_rule(
        &self,
        _device: &DiscoveredDevice,
        recording_rule_id: &str,
        _discovered_devices: &[DiscoveredDevice],
    ) -> Result<DvrRuleDeleteSnapshot, AppError> {
        let mut fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        let original_rule_len = fixtures.rules.len();
        fixtures
            .rules
            .retain(|rule| rule.recording_rule_id != recording_rule_id);
        fixtures
            .upcoming
            .retain(|entry| entry.recording_rule_id != recording_rule_id);

        Ok(DvrRuleDeleteSnapshot {
            outcome: if fixtures.rules.len() == original_rule_len {
                DvrRuleDeleteOutcome::MissingRule
            } else {
                DvrRuleDeleteOutcome::Confirmed
            },
            warnings: if fixtures.rules.len() == original_rule_len {
                vec!["The selected recording rule is no longer available; refresh DVR state and try again.".to_string()]
            } else {
                Vec::new()
            },
        })
    }

    fn upcoming_for(&self, _device: &DiscoveredDevice) -> Result<DvrUpcomingSnapshot, AppError> {
        let fixtures = self.fixtures.lock().expect("static DVR fixtures lock");
        Ok(DvrUpcomingSnapshot {
            state: if fixtures.upcoming.is_empty() {
                DvrUpcomingState::Unavailable
            } else {
                DvrUpcomingState::Ready
            },
            entries: fixtures.upcoming.clone(),
            schedule_projection: build_schedule_projection(&fixtures.rules, &fixtures.upcoming),
            warnings: Vec::new(),
        })
    }
}

fn append_rule_options(
    form: &mut Vec<(String, String)>,
    options: &DvrRuleOptions,
    allow_multi_channel: bool,
) -> Result<(), AppError> {
    NativeDvrProvider::validate_rule_options(options, allow_multi_channel)?;

    if !options.channel_only.is_empty() {
        form.push(("ChannelOnly".to_string(), options.channel_only.join("|")));
    }
    if !options.team_only.is_empty() {
        form.push(("TeamOnly".to_string(), options.team_only.join("|")));
    }
    if options.recent_only {
        form.push(("RecentOnly".to_string(), "1".to_string()));
    }
    if let Some(value) = options.after_original_airdate_only {
        form.push(("AfterOriginalAirdateOnly".to_string(), value.to_string()));
    }
    if let Some(value) = options.start_padding {
        form.push(("StartPadding".to_string(), value.to_string()));
    }
    if let Some(value) = options.end_padding {
        form.push(("EndPadding".to_string(), value.to_string()));
    }

    Ok(())
}

fn storage_sync_url(storage_url: &str) -> Option<String> {
    let storage_url = storage_url.trim();
    if storage_url.is_empty() {
        return None;
    }

    let base = if let Some(prefix) = storage_url.strip_suffix("/recorded_files.json") {
        prefix
    } else if let Some((prefix, _)) = storage_url.rsplit_once('/') {
        prefix
    } else {
        storage_url
    };

    Some(format!("{}/recording_events.post?sync", base.trim_end_matches('/')))
}

pub fn build_schedule_projection(
    rules: &[DvrRecordingRule],
    upcoming: &[DvrUpcomingRecording],
) -> Vec<DvrScheduleProjectionEntry> {
    let mut projection = upcoming
        .iter()
        .map(|entry| DvrScheduleProjectionEntry {
            series_id: entry.series_id.clone(),
            program_id: Some(entry.program_id.clone()),
            title: entry.title.clone(),
            recording_rule_id: Some(entry.recording_rule_id.clone()),
            state: DvrScheduleProjectionState::Scheduled,
            reason: "Matched an explicit upcoming recording.".to_string(),
            source: DvrScheduleProjectionSource::ExplicitUpcoming,
        })
        .collect::<Vec<_>>();

    for rule in rules {
        let already_scheduled = projection.iter().any(|entry| {
            entry.recording_rule_id.as_deref() == Some(rule.recording_rule_id.as_str())
                || entry.series_id == rule.series_id
        });

        if !already_scheduled {
            projection.push(DvrScheduleProjectionEntry {
                series_id: rule.series_id.clone(),
                program_id: None,
                title: rule.title.clone(),
                recording_rule_id: Some(rule.recording_rule_id.clone()),
                state: DvrScheduleProjectionState::NotScheduled,
                reason: "A recording rule exists, but no explicit upcoming airing is currently scheduled.".to_string(),
                source: DvrScheduleProjectionSource::RuleContext,
            });
        }
    }

    projection
}

fn unix_timestamp_now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs() as i64)
        .unwrap_or(0)
}

#[derive(Debug, Deserialize)]
struct RecordingRuleWire {
    #[serde(rename = "RecordingRuleID")]
    recording_rule_id: String,
    #[serde(rename = "SeriesID")]
    series_id: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Synopsis")]
    synopsis: Option<String>,
    #[serde(rename = "ImageURL")]
    image_url: Option<String>,
    #[serde(rename = "ChannelOnly")]
    channel_only: Option<String>,
    #[serde(rename = "TeamOnly")]
    team_only: Option<String>,
    #[serde(rename = "RecentOnly")]
    recent_only: Option<bool>,
    #[serde(rename = "AfterOriginalAirdateOnly")]
    after_original_airdate_only: Option<i64>,
    #[serde(rename = "DateTimeOnly")]
    date_time_only: Option<i64>,
    #[serde(rename = "Priority")]
    priority: Option<u32>,
    #[serde(rename = "StartPadding")]
    start_padding: Option<u32>,
    #[serde(rename = "EndPadding")]
    end_padding: Option<u32>,
}

impl From<RecordingRuleWire> for DvrRecordingRule {
    fn from(value: RecordingRuleWire) -> Self {
        Self {
            recording_rule_id: value.recording_rule_id,
            series_id: value.series_id,
            title: value.title,
            synopsis: value.synopsis,
            image_url: value.image_url,
            kind: if value.date_time_only.is_some() {
                DvrRuleKind::OneTime
            } else {
                DvrRuleKind::Series
            },
            channel_only: split_pipe_value(value.channel_only),
            team_only: split_pipe_value(value.team_only),
            recent_only: value.recent_only.unwrap_or(false),
            after_original_airdate_only: value.after_original_airdate_only,
            date_time_only: value.date_time_only,
            priority: value.priority,
            start_padding: value.start_padding.unwrap_or(30),
            end_padding: value.end_padding.unwrap_or(30),
        }
    }
}

#[derive(Debug, Deserialize)]
struct UpcomingRecordingWire {
    #[serde(rename = "RecordingRuleID")]
    recording_rule_id: String,
    #[serde(rename = "SeriesID")]
    series_id: String,
    #[serde(rename = "ProgramID")]
    program_id: String,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "EpisodeNumber")]
    episode_number: Option<String>,
    #[serde(rename = "EpisodeTitle")]
    episode_title: Option<String>,
    #[serde(rename = "Synopsis")]
    synopsis: Option<String>,
    #[serde(rename = "ImageURL")]
    image_url: Option<String>,
    #[serde(rename = "StartTime")]
    start_time: i64,
    #[serde(rename = "EndTime")]
    end_time: i64,
    #[serde(rename = "RecordStartTime")]
    record_start_time: i64,
    #[serde(rename = "RecordEndTime")]
    record_end_time: i64,
    #[serde(rename = "ChannelNumber")]
    channel_number: String,
    #[serde(rename = "ChannelName")]
    channel_name: String,
    #[serde(rename = "ChannelImageURL")]
    channel_image_url: Option<String>,
    #[serde(rename = "RecordingRuleExt")]
    recording_rule_ext: Option<String>,
}

impl From<UpcomingRecordingWire> for DvrUpcomingRecording {
    fn from(value: UpcomingRecordingWire) -> Self {
        Self {
            recording_rule_id: value.recording_rule_id,
            series_id: value.series_id,
            program_id: value.program_id,
            title: value.title,
            episode_number: value.episode_number,
            episode_title: value.episode_title,
            synopsis: value.synopsis,
            image_url: value.image_url,
            start_time: value.start_time,
            end_time: value.end_time,
            record_start_time: value.record_start_time,
            record_end_time: value.record_end_time,
            channel_number: value.channel_number,
            channel_name: value.channel_name,
            channel_image_url: value.channel_image_url,
            recording_rule_ext: value.recording_rule_ext,
        }
    }
}

fn split_pipe_value(value: Option<String>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split('|')
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
        .map(ToString::to_string)
        .collect()
}

#[derive(Clone)]
struct RecordingSource {
    source_ref: String,
    storage_url: String,
    is_local: bool,
    priority_rank: usize,
}

struct DvrRecordingsCatalog {
    state: DvrRecordingsState,
    recordings: Vec<ResolvedRecordingSummary>,
    warnings: Vec<String>,
}

#[derive(Clone)]
struct RecordedCandidate {
    recording_id: String,
    title: String,
    episode_title: Option<String>,
    synopsis: Option<String>,
    image_url: Option<String>,
    channel_name: Option<String>,
    channel_number: Option<String>,
    record_start_time: i64,
    record_end_time: i64,
    resume_position: i64,
    watched: bool,
    playback_url: Option<String>,
    cmd_url: Option<String>,
    is_local: bool,
    priority_rank: usize,
}

struct ResolvedRecording {
    recording_id: String,
    variants: Vec<RecordedCandidate>,
}

#[derive(Clone)]
struct ResolvedRecordingSummary {
    recording_id: String,
    summary: DvrRecordingSummary,
    playback_url: Option<String>,
    cmd_url: Option<String>,
}

impl ResolvedRecording {
    fn into_summary(mut self) -> ResolvedRecordingSummary {
        self.variants.sort_by(|left, right| {
            left.priority_rank
                .cmp(&right.priority_rank)
                .then_with(|| right.record_start_time.cmp(&left.record_start_time))
        });
        let preferred = self.variants.remove(0);
        let source_count = (1 + self.variants.len()) as u32;

        ResolvedRecordingSummary {
            recording_id: self.recording_id,
            summary: DvrRecordingSummary {
                recording_id: preferred.recording_id.clone(),
                title: preferred.title.clone(),
                episode_title: preferred.episode_title.clone(),
                synopsis: preferred.synopsis.clone(),
                image_url: preferred.image_url.clone(),
                channel_name: preferred.channel_name.clone(),
                channel_number: preferred.channel_number.clone(),
                record_start_time: preferred.record_start_time,
                record_end_time: preferred.record_end_time,
                resume_position: preferred.resume_position,
                watched: preferred.watched,
                source_count,
                preferred_local: preferred.is_local,
            },
            playback_url: preferred.playback_url.clone(),
            cmd_url: preferred.cmd_url.clone(),
        }
    }
}

fn recording_sources(
    selected_device: &DiscoveredDevice,
    discovered_devices: &[DiscoveredDevice],
) -> Vec<RecordingSource> {
    let selected_host = url_host(&selected_device.base_url);
    let mut sources = discovered_devices
        .iter()
        .filter_map(|device| {
            let storage_url = device.storage_url.as_deref()?.trim();
            if storage_url.is_empty() {
                return None;
            }

            let is_local = device.device_ref == selected_device.device_ref
                || (selected_host.is_some() && selected_host == url_host(storage_url));

            Some(RecordingSource {
                source_ref: device.device_ref.clone(),
                storage_url: storage_url.to_string(),
                is_local,
                priority_rank: usize::from(!is_local),
            })
        })
        .collect::<Vec<_>>();

    sources.sort_by(|left, right| {
        left.priority_rank
            .cmp(&right.priority_rank)
            .then_with(|| left.source_ref.cmp(&right.source_ref))
    });
    sources
}

fn normalize_recorded_candidate(source: &RecordingSource, entry: RecordedFileWire) -> RecordedCandidate {
    let title = entry
        .title
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "Untitled recording".to_string());
    let record_start_time = entry.record_start_time.or(entry.start_time).unwrap_or(0);
    let record_end_time = entry.record_end_time.or(entry.end_time).unwrap_or(record_start_time);
    let resume_position = entry.resume_position.unwrap_or(0).max(0);
    let recording_id = build_recording_id(&logical_recording_key(&entry, &title, record_start_time));

    RecordedCandidate {
        recording_id,
        title,
        episode_title: entry.episode_title.filter(|value| !value.trim().is_empty()),
        synopsis: entry.synopsis.filter(|value| !value.trim().is_empty()),
        image_url: entry.image_url.filter(|value| !value.trim().is_empty()),
        channel_name: entry.channel_name.filter(|value| !value.trim().is_empty()),
        channel_number: entry.channel_number.filter(|value| !value.trim().is_empty()),
        record_start_time,
        record_end_time,
        resume_position,
        watched: is_recording_watched(resume_position, record_start_time, entry.end_time.unwrap_or(record_end_time)),
        playback_url: coalesce_optional_url(entry.playback_url),
        cmd_url: coalesce_optional_url(entry.cmd_url),
        is_local: source.is_local,
        priority_rank: source.priority_rank,
    }
}

fn flatten_recording_entries<F>(entries: Vec<RecordedFileWire>, mut fetch_nested: F) -> Vec<RecordedFileWire>
where
    F: FnMut(&str) -> Option<Vec<RecordedFileWire>>,
{
    let mut flattened = Vec::new();

    for entry in entries {
        let episodes_url = entry
            .episodes_url
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty());

        if entry_has_direct_targets(&entry) || episodes_url.is_none() {
            flattened.push(entry);
            continue;
        }

        if let Some(mut nested_entries) = fetch_nested(episodes_url.expect("episodes url checked above")) {
            if !nested_entries.is_empty() {
                flattened.append(&mut nested_entries);
                continue;
            }
        }

        flattened.push(entry);
    }

    flattened
}

fn entry_has_direct_targets(entry: &RecordedFileWire) -> bool {
    optional_wire_value(&entry.playback_url).is_some() || optional_wire_value(&entry.cmd_url).is_some()
}

fn optional_wire_value(value: &Option<String>) -> Option<String> {
    value.as_deref().map(str::trim).filter(|value| !value.is_empty()).map(ToString::to_string)
}

fn logical_recording_key(entry: &RecordedFileWire, title: &str, record_start_time: i64) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        entry.series_id.as_deref().unwrap_or_default(),
        entry.program_id.as_deref().unwrap_or_default(),
        title,
        entry.episode_title.as_deref().unwrap_or_default(),
        record_start_time
    )
}

fn build_recording_id(logical_key: &str) -> String {
    let mut id = String::with_capacity(logical_key.len() * 2);
    for byte in logical_key.as_bytes() {
        use std::fmt::Write as _;
        let _ = write!(&mut id, "{:02x}", byte);
    }
    id
}

fn coalesce_optional_url(value: Option<String>) -> Option<String> {
    value.map(|value| value.trim().to_string()).filter(|value| !value.is_empty())
}

fn is_recording_watched(resume_position: i64, record_start_time: i64, end_time: i64) -> bool {
    resume_position >= end_time.saturating_sub(record_start_time).saturating_sub(180)
}

fn url_host(value: &str) -> Option<String> {
    reqwest::Url::parse(value).ok()?.host_str().map(ToString::to_string)
}

#[derive(Clone, Debug, Deserialize)]
struct RecordedFileWire {
    #[serde(rename = "SeriesID")]
    series_id: Option<String>,
    #[serde(rename = "ProgramID")]
    program_id: Option<String>,
    #[serde(rename = "Title")]
    title: Option<String>,
    #[serde(rename = "EpisodeTitle", alias = "Subtitle")]
    episode_title: Option<String>,
    #[serde(rename = "Synopsis")]
    synopsis: Option<String>,
    #[serde(rename = "ImageURL", alias = "EpisodeImageURL")]
    image_url: Option<String>,
    #[serde(rename = "StartTime")]
    start_time: Option<i64>,
    #[serde(rename = "EndTime")]
    end_time: Option<i64>,
    #[serde(rename = "RecordStartTime")]
    record_start_time: Option<i64>,
    #[serde(rename = "RecordEndTime")]
    record_end_time: Option<i64>,
    #[serde(rename = "ChannelName")]
    channel_name: Option<String>,
    #[serde(rename = "ChannelNumber")]
    channel_number: Option<String>,
    #[serde(rename = "ResumePosition")]
    resume_position: Option<i64>,
    #[serde(rename = "EpisodesURL")]
    episodes_url: Option<String>,
    #[serde(rename = "PlayURL", alias = "PlaybackURL", alias = "URL")]
    playback_url: Option<String>,
    #[serde(rename = "CmdURL")]
    cmd_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        NativeDvrProvider, RecordedFileWire, RecordingSource, build_schedule_projection,
        flatten_recording_entries, normalize_recorded_candidate, storage_sync_url,
    };
    use crate::models::{
        CreateOneTimeRecordingRuleRequest, CreateSeriesRecordingRuleRequest, DvrReadinessCondition,
        DvrReadinessConditionCode, DvrReadinessConditionSeverity, DvrReadinessState,
        DvrRecordingRule, DvrRuleDeleteOutcome, DvrRuleKind, DvrRuleOptions, DvrScheduleProjectionSource,
        DvrScheduleProjectionState, DvrUpcomingRecording,
    };
    use crate::error::AppError;
    use reqwest::StatusCode;

    #[test]
    fn schedule_projection_prefers_explicit_upcoming_entries() {
        let rules = vec![DvrRecordingRule {
            recording_rule_id: "rule-1".to_string(),
            series_id: "series-1".to_string(),
            title: "Example Show".to_string(),
            synopsis: None,
            image_url: None,
            kind: DvrRuleKind::Series,
            channel_only: Vec::new(),
            team_only: Vec::new(),
            recent_only: false,
            after_original_airdate_only: None,
            date_time_only: None,
            priority: Some(1),
            start_padding: 30,
            end_padding: 30,
        }];
        let upcoming = vec![DvrUpcomingRecording {
            recording_rule_id: "rule-1".to_string(),
            series_id: "series-1".to_string(),
            program_id: "program-1".to_string(),
            title: "Example Show".to_string(),
            episode_number: None,
            episode_title: None,
            synopsis: None,
            image_url: None,
            start_time: 2_000_000_000,
            end_time: 2_000_000_300,
            record_start_time: 1_999_999_970,
            record_end_time: 2_000_000_330,
            channel_number: "5.1".to_string(),
            channel_name: "News".to_string(),
            channel_image_url: None,
            recording_rule_ext: Some("RecordIfNotRecorded".to_string()),
        }];

        let projection = build_schedule_projection(&rules, &upcoming);
        assert_eq!(projection.len(), 1);
        assert_eq!(projection[0].state, DvrScheduleProjectionState::Scheduled);
        assert_eq!(projection[0].source, DvrScheduleProjectionSource::ExplicitUpcoming);
    }

    #[test]
    fn schedule_projection_keeps_rule_without_guessing_schedule() {
        let rules = vec![DvrRecordingRule {
            recording_rule_id: "rule-1".to_string(),
            series_id: "series-1".to_string(),
            title: "Example Show".to_string(),
            synopsis: None,
            image_url: None,
            kind: DvrRuleKind::Series,
            channel_only: Vec::new(),
            team_only: Vec::new(),
            recent_only: false,
            after_original_airdate_only: None,
            date_time_only: None,
            priority: Some(1),
            start_padding: 30,
            end_padding: 30,
        }];

        let projection = build_schedule_projection(&rules, &[]);
        assert_eq!(projection.len(), 1);
        assert_eq!(projection[0].state, DvrScheduleProjectionState::NotScheduled);
        assert_eq!(projection[0].source, DvrScheduleProjectionSource::RuleContext);
    }

    #[test]
    fn series_request_rejects_unsupported_options() {
        let request = CreateSeriesRecordingRuleRequest {
            series_id: "series-1".to_string(),
            title: None,
            options: DvrRuleOptions {
                unsupported_options: vec!["mysteryOption".to_string()],
                ..DvrRuleOptions::default()
            },
        };

        let error = NativeDvrProvider::validate_series_request(&request).expect_err("validation error");
        match error {
            crate::error::AppError::Validation(message) => {
                assert!(message.contains("unsupported DVR rule options"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn one_time_request_rejects_multi_channel_requests() {
        let request = CreateOneTimeRecordingRuleRequest {
            series_id: "series-1".to_string(),
            title: None,
            start_time: i64::MAX,
            channel_number: "5.1".to_string(),
            options: DvrRuleOptions {
                channel_only: vec!["5.1".to_string(), "7.2".to_string()],
                ..DvrRuleOptions::default()
            },
        };

        let error = NativeDvrProvider::validate_one_time_request(&request).expect_err("validation error");
        match error {
            crate::error::AppError::Validation(message) => {
                assert!(message.contains("exactly one channel"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn storage_sync_url_reuses_storage_base() {
        let sync_url = storage_sync_url("http://10.0.0.5:4999/recorded_files.json").expect("sync url");
        assert_eq!(sync_url, "http://10.0.0.5:4999/recording_events.post?sync");
    }

    #[test]
    fn readiness_condition_shape_is_blocking_for_missing_storage() {
        let condition = DvrReadinessCondition {
            code: DvrReadinessConditionCode::MissingStorage,
            severity: DvrReadinessConditionSeverity::Blocking,
            message: "No storage".to_string(),
            recoverable: true,
        };

        assert_eq!(condition.code, DvrReadinessConditionCode::MissingStorage);
        assert_eq!(condition.severity, DvrReadinessConditionSeverity::Blocking);
        assert_eq!(DvrReadinessState::NotReady, DvrReadinessState::NotReady);
    }

    #[test]
    fn normalize_recorded_candidate_marks_watched_from_resume_position() {
        let candidate = normalize_recorded_candidate(
            &RecordingSource {
                source_ref: "hdhr-local".to_string(),
                storage_url: "http://10.0.0.5:4999/recorded_files.json".to_string(),
                is_local: true,
                priority_rank: 0,
            },
            RecordedFileWire {
                series_id: Some("series-1".to_string()),
                program_id: Some("program-1".to_string()),
                title: Some("Example Show".to_string()),
                episode_title: Some("Pilot".to_string()),
                synopsis: None,
                image_url: None,
                start_time: Some(1_000),
                end_time: Some(1_600),
                record_start_time: Some(970),
                record_end_time: Some(1_630),
                channel_name: Some("News".to_string()),
                channel_number: Some("5.1".to_string()),
                resume_position: Some(500),
                episodes_url: None,
                playback_url: Some("http://10.0.0.5:4999/play/abc".to_string()),
                cmd_url: Some("http://10.0.0.5:4999/cmd/abc".to_string()),
            },
        );

        assert!(candidate.watched);
        assert!(candidate.is_local);
    }

    #[test]
    fn flatten_recording_entries_replaces_series_aggregate_with_episode_entries() {
        let flattened = flatten_recording_entries(
            vec![RecordedFileWire {
                series_id: Some("series-1".to_string()),
                program_id: None,
                title: Some("Example Show".to_string()),
                episode_title: None,
                synopsis: None,
                image_url: None,
                start_time: Some(100),
                end_time: Some(200),
                record_start_time: None,
                record_end_time: None,
                channel_name: None,
                channel_number: None,
                resume_position: None,
                episodes_url: Some("http://10.0.0.5/recorded_files.json?SeriesID=series-1".to_string()),
                playback_url: None,
                cmd_url: None,
            }],
            |episodes_url| {
                assert_eq!(episodes_url, "http://10.0.0.5/recorded_files.json?SeriesID=series-1");
                Some(vec![RecordedFileWire {
                    series_id: Some("series-1".to_string()),
                    program_id: Some("program-1".to_string()),
                    title: Some("Example Show".to_string()),
                    episode_title: Some("Pilot".to_string()),
                    synopsis: None,
                    image_url: None,
                    start_time: Some(100),
                    end_time: Some(200),
                    record_start_time: Some(101),
                    record_end_time: Some(205),
                    channel_name: Some("WXYZ".to_string()),
                    channel_number: Some("5.1".to_string()),
                    resume_position: Some(0),
                    episodes_url: None,
                    playback_url: Some("http://10.0.0.5/recorded/play?id=123".to_string()),
                    cmd_url: Some("http://10.0.0.5/recorded/cmd?id=123".to_string()),
                }])
            },
        );

        assert_eq!(flattened.len(), 1);
        assert_eq!(flattened[0].episode_title.as_deref(), Some("Pilot"));
        assert_eq!(flattened[0].playback_url.as_deref(), Some("http://10.0.0.5/recorded/play?id=123"));
        assert_eq!(flattened[0].cmd_url.as_deref(), Some("http://10.0.0.5/recorded/cmd?id=123"));
    }

    #[test]
    fn delete_bad_request_maps_to_validation_error() {
        let error = NativeDvrProvider::map_delete_response_status(StatusCode::BAD_REQUEST);

        assert!(matches!(error, AppError::Validation(_)));
        assert_eq!(
            error.to_string(),
            "validation failure: the record engine rejected the delete command for this recording; the recording may no longer be deletable from this device"
        );
    }

    #[test]
    fn delete_command_url_preserves_existing_query_and_appends_delete_parameters() {
        let url = NativeDvrProvider::delete_command_url("http://10.0.0.5/recorded/cmd?id=abc123&token=xyz")
            .expect("delete url");

        assert_eq!(
            url.as_str(),
            "http://10.0.0.5/recorded/cmd?id=abc123&token=xyz&cmd=delete&rerecord=0"
        );
    }

    #[test]
    fn static_rule_delete_removes_rule_and_upcoming_entries() {
        use super::{StaticDvrFixtures, StaticDvrProvider};
        use crate::device::DiscoveredDevice;

        let provider = StaticDvrProvider::shared(StaticDvrFixtures {
            readiness: None,
            rules: vec![DvrRecordingRule {
                recording_rule_id: "rule-1".to_string(),
                series_id: "series-1".to_string(),
                title: "Example Show".to_string(),
                synopsis: None,
                image_url: None,
                kind: DvrRuleKind::Series,
                channel_only: Vec::new(),
                team_only: Vec::new(),
                recent_only: false,
                after_original_airdate_only: None,
                date_time_only: None,
                priority: Some(1),
                start_padding: 30,
                end_padding: 30,
            }],
            upcoming: vec![DvrUpcomingRecording {
                recording_rule_id: "rule-1".to_string(),
                series_id: "series-1".to_string(),
                program_id: "program-1".to_string(),
                title: "Example Show".to_string(),
                episode_number: None,
                episode_title: Some("Pilot".to_string()),
                synopsis: None,
                image_url: None,
                start_time: 2_000_000_000,
                end_time: 2_000_000_300,
                record_start_time: 1_999_999_970,
                record_end_time: 2_000_000_330,
                channel_number: "5.1".to_string(),
                channel_name: "News".to_string(),
                channel_image_url: None,
                recording_rule_ext: Some("RecordIfNotRecorded".to_string()),
            }],
            recordings: Vec::new(),
        });

        let device = DiscoveredDevice {
            device_ref: "hdhr-test".to_string(),
            device_id: "device-1".to_string(),
            friendly_name: "Test Device".to_string(),
            base_url: "http://127.0.0.1".to_string(),
            device_auth: Some("auth".to_string()),
            storage_url: None,
            lineup_url: None,
            tuner_count: 2,
            is_legacy: false,
        };

        let snapshot = provider
            .delete_rule(&device, "rule-1", &[])
            .expect("delete snapshot");
        assert_eq!(snapshot.outcome, DvrRuleDeleteOutcome::Confirmed);

        let rules = provider.list_rules(&device).expect("rules after delete");
        assert!(rules.is_empty());

        let upcoming = provider.upcoming_for(&device).expect("upcoming after delete");
        assert!(upcoming.entries.is_empty());
    }
}