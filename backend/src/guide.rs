use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use chrono::Utc;
use serde::Deserialize;

use crate::device::DiscoveredDevice;
use crate::error::AppError;
use crate::models::{GuideChannel, GuideEntry, LineupChannel};

const GUIDE_API_URL: &str = "https://api.hdhomerun.com/api/guide";
const GUIDE_DURATION_HOURS: u8 = 4;

pub trait GuideProvider: Send + Sync {
    fn schedule_for(
        &self,
        device: &DiscoveredDevice,
        channels: &[LineupChannel],
        start: i64,
        duration_hours: u8,
    ) -> Result<Vec<GuideChannel>, AppError>;

    fn current_programs_for(
        &self,
        device: &DiscoveredDevice,
        channels: &[LineupChannel],
    ) -> Result<HashMap<String, String>, AppError>;
}

pub type SharedGuideProvider = Arc<dyn GuideProvider>;

pub struct NativeGuideProvider;

impl NativeGuideProvider {
    pub fn shared() -> SharedGuideProvider {
        Arc::new(Self)
    }

    fn fetch_vendor_guide(
        device: &DiscoveredDevice,
        start: Option<i64>,
        duration_hours: u8,
    ) -> Result<Vec<GuideChannelWire>, AppError> {
        let device_auth = device
            .device_auth
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                AppError::internal("selected device did not expose a DeviceAuth value for guide lookup")
            })?;
        let duration = duration_hours.to_string();

        let mut request = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(4))
            .build()
            .map_err(|error| AppError::internal(format!("failed to build guide client: {error}")))?
            .get(GUIDE_API_URL)
            .query(&[("DeviceAuth", device_auth), ("Duration", duration.as_str())]);

        if let Some(start) = start {
            request = request.query(&[("Start", start)]);
        }

        request
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("guide request failed: {error}")))?
            .json::<Vec<GuideChannelWire>>()
            .map_err(|error| AppError::internal(format!("guide response parsing failed: {error}")))
    }
}

impl GuideProvider for NativeGuideProvider {
    fn schedule_for(
        &self,
        device: &DiscoveredDevice,
        channels: &[LineupChannel],
        start: i64,
        duration_hours: u8,
    ) -> Result<Vec<GuideChannel>, AppError> {
        if channels.is_empty() {
            return Ok(Vec::new());
        }

        let guide = Self::fetch_vendor_guide(device, Some(start), duration_hours)?;
        Ok(resolve_schedule(channels, &guide, start, duration_hours))
    }

    fn current_programs_for(
        &self,
        device: &DiscoveredDevice,
        channels: &[LineupChannel],
    ) -> Result<HashMap<String, String>, AppError> {
        if channels.is_empty() {
            return Ok(HashMap::new());
        }

        let guide = Self::fetch_vendor_guide(device, None, GUIDE_DURATION_HOURS)?;

        resolve_current_programs(channels, &guide, Utc::now().timestamp())
    }
}

#[derive(Clone)]
pub struct StaticGuideProvider {
    programs: HashMap<String, String>,
}

impl StaticGuideProvider {
    pub fn shared(programs: HashMap<String, String>) -> SharedGuideProvider {
        Arc::new(Self { programs })
    }
}

impl GuideProvider for StaticGuideProvider {
    fn schedule_for(
        &self,
        _device: &DiscoveredDevice,
        channels: &[LineupChannel],
        start: i64,
        duration_hours: u8,
    ) -> Result<Vec<GuideChannel>, AppError> {
        let end = start + (i64::from(duration_hours) * 3600);
        Ok(channels
            .iter()
            .map(|channel| GuideChannel {
                channel_ref: channel.channel_ref.clone(),
                guide_number: channel.guide_number.clone(),
                guide_name: channel.guide_name.clone(),
                current_program_title: self.programs.get(&channel.channel_ref).cloned(),
                image_url: None,
                entries: self
                    .programs
                    .get(&channel.channel_ref)
                    .map(|title| {
                        vec![GuideEntry {
                            start_time: start,
                            end_time: end,
                            title: title.clone(),
                            episode_title: None,
                            synopsis: None,
                            image_url: None,
                            is_current: true,
                        }]
                    })
                    .unwrap_or_default(),
            })
            .collect())
    }

    fn current_programs_for(
        &self,
        _device: &DiscoveredDevice,
        _channels: &[LineupChannel],
    ) -> Result<HashMap<String, String>, AppError> {
        Ok(self.programs.clone())
    }
}

pub fn resolve_current_programs(
    channels: &[LineupChannel],
    guide: &[GuideChannelWire],
    now: i64,
) -> Result<HashMap<String, String>, AppError> {
    let schedule = resolve_schedule(channels, guide, now, GUIDE_DURATION_HOURS);
    let mut programs = HashMap::new();

    for channel in schedule {
        if let Some(entry) = channel.entries.iter().find(|entry| entry.is_current) {
            programs.insert(channel.channel_ref, entry.title.clone());
        }
    }

    Ok(programs)
}

pub fn resolve_schedule(
    channels: &[LineupChannel],
    guide: &[GuideChannelWire],
    now: i64,
    duration_hours: u8,
) -> Vec<GuideChannel> {
    let window_end = now + (i64::from(duration_hours) * 3600);

    channels
        .iter()
        .map(|channel| {
            let matched = guide.iter().find(|candidate| guide_matches_channel(candidate, channel));
            let entries = matched
                .map(|matched| {
                    matched
                        .guide
                        .iter()
                        .filter(|entry| entry.end_time > now && entry.start_time < window_end)
                        .map(|entry| GuideEntry {
                            start_time: entry.start_time,
                            end_time: entry.end_time,
                            title: entry.title.trim().to_string(),
                            episode_title: entry.episode_title.clone(),
                            synopsis: entry.synopsis.clone(),
                            image_url: entry.image_url.clone(),
                            is_current: now >= entry.start_time && now < entry.end_time,
                        })
                        .filter(|entry| !entry.title.is_empty())
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            let current_program_title = entries.iter().find(|entry| entry.is_current).map(|entry| entry.title.clone());

            GuideChannel {
                channel_ref: channel.channel_ref.clone(),
                guide_number: channel.guide_number.clone(),
                guide_name: channel.guide_name.clone(),
                current_program_title,
                image_url: matched.and_then(|entry| entry.image_url.clone()),
                entries,
            }
        })
        .collect()
}

fn normalize_guide_value(value: &str) -> String {
    value.trim().to_lowercase()
}

fn guide_matches_channel(guide_channel: &GuideChannelWire, channel: &LineupChannel) -> bool {
    normalize_guide_value(&guide_channel.guide_number) == normalize_guide_value(&channel.guide_number)
        || normalize_guide_value(&guide_channel.guide_name) == normalize_guide_value(&channel.guide_name)
}

#[derive(Debug, Deserialize)]
pub struct GuideChannelWire {
    #[serde(rename = "GuideNumber")]
    pub guide_number: String,
    #[serde(rename = "GuideName")]
    pub guide_name: String,
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "Guide", default)]
    pub guide: Vec<GuideEntryWire>,
}

#[derive(Debug, Deserialize)]
pub struct GuideEntryWire {
    #[serde(rename = "StartTime")]
    pub start_time: i64,
    #[serde(rename = "EndTime")]
    pub end_time: i64,
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "EpisodeTitle")]
    pub episode_title: Option<String>,
    #[serde(rename = "Synopsis")]
    pub synopsis: Option<String>,
    #[serde(rename = "ImageURL")]
    pub image_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{GuideChannelWire, GuideEntryWire, resolve_current_programs};
    use crate::models::{ChannelAvailability, LineupChannel};

    fn sample_channel(guide_number: &str, guide_name: &str) -> LineupChannel {
        LineupChannel {
            channel_ref: format!("channel:{guide_number}"),
            guide_number: guide_number.to_string(),
            guide_name: guide_name.to_string(),
            current_program_title: None,
            image_url: None,
            tags: Vec::new(),
            playback_url: Some(format!("http://192.168.1.10/auto/v{guide_number}")),
            availability: ChannelAvailability::Playable,
            restriction_reason: None,
        }
    }

    #[test]
        fn resolves_current_programs_from_vendor_guide_entries() {
        let channels = vec![sample_channel("5.1", "WRCB-HD"), sample_channel("7.2", "Weather")];
                let guide = vec![
                        GuideChannelWire {
                                guide_number: "5.1".to_string(),
                                guide_name: "WRCB-HD".to_string(),
                            image_url: None,
                                guide: vec![GuideEntryWire {
                                        start_time: 1_779_111_200,
                                        end_time: 1_779_114_800,
                                        title: "Evening News".to_string(),
                                episode_title: None,
                                synopsis: None,
                                image_url: None,
                                }],
                        },
                        GuideChannelWire {
                                guide_number: "7.2".to_string(),
                                guide_name: "Weather".to_string(),
                            image_url: None,
                                guide: vec![GuideEntryWire {
                                        start_time: 1_779_111_200,
                                        end_time: 1_779_114_800,
                                        title: "Storm Center".to_string(),
                                episode_title: None,
                                synopsis: None,
                                image_url: None,
                                }],
                        },
                ];

                let programs = resolve_current_programs(&channels, &guide, 1_779_113_400).expect("programs");

        assert_eq!(programs.get("channel:5.1"), Some(&"Evening News".to_string()));
        assert_eq!(programs.get("channel:7.2"), Some(&"Storm Center".to_string()));
    }

    #[test]
        fn ignores_airings_outside_current_time_window() {
        let channels = vec![sample_channel("5.1", "WRCB-HD")];
                let guide = vec![GuideChannelWire {
                        guide_number: "5.1".to_string(),
                        guide_name: "WRCB-HD".to_string(),
                    image_url: None,
                        guide: vec![GuideEntryWire {
                                start_time: 1_779_104_000,
                                end_time: 1_779_107_600,
                                title: "Earlier Show".to_string(),
                        episode_title: None,
                        synopsis: None,
                        image_url: None,
                        }],
                }];

                let programs = resolve_current_programs(&channels, &guide, 1_779_113_400).expect("programs");

        assert!(programs.is_empty());
    }
}