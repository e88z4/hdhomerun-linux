use std::ffi::c_char;
use std::ffi::CString;
use std::sync::Arc;
use std::time::Duration;

use crate::error::AppError;
use crate::models::{
    ChannelAvailability, ContractEndpointStatus, DeviceSummary, DevicesResponse, LineupChannel,
    RememberedContext, TunerDiagnostic,
};

const HDHOMERUN_DEVICE_TYPE_TUNER: u32 = 0x0000_0001;
const HDHOMERUN_DEVICE_ID_WILDCARD: u32 = 0xFFFF_FFFF;
const MAX_DISCOVERY_RESULTS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DiscoveredDevice {
    pub device_ref: String,
    pub device_id: String,
    pub friendly_name: String,
    pub base_url: String,
    pub lineup_url: Option<String>,
    pub tuner_count: u8,
    pub is_legacy: bool,
}

pub trait DeviceDiscovery: Send + Sync {
    fn discover(&self) -> Result<Vec<DiscoveredDevice>, AppError>;
}

pub type SharedDeviceDiscovery = Arc<dyn DeviceDiscovery>;

pub trait LineupProvider: Send + Sync {
    fn lineup_for(&self, device: &DiscoveredDevice) -> Result<Vec<LineupChannel>, AppError>;
}

pub type SharedLineupProvider = Arc<dyn LineupProvider>;

pub trait TunerDiagnosticsProvider: Send + Sync {
    fn diagnostics_for(
        &self,
        device: &DiscoveredDevice,
        remembered_context: Option<&RememberedContext>,
    ) -> Result<Vec<Result<TunerDiagnostic, String>>, AppError>;
}

pub type SharedTunerDiagnosticsProvider = Arc<dyn TunerDiagnosticsProvider>;

pub struct NativeDeviceDiscovery;

impl NativeDeviceDiscovery {
    pub fn shared() -> SharedDeviceDiscovery {
        Arc::new(Self)
    }
}

impl DeviceDiscovery for NativeDeviceDiscovery {
    fn discover(&self) -> Result<Vec<DiscoveredDevice>, AppError> {
        let mut results = vec![ffi::HdhomerunDiscoverDeviceV3::default(); MAX_DISCOVERY_RESULTS];
        let count = unsafe {
            ffi::hdhomerun_discover_find_devices_custom_v3(
                0,
                HDHOMERUN_DEVICE_TYPE_TUNER,
                HDHOMERUN_DEVICE_ID_WILDCARD,
                results.as_mut_ptr(),
                results.len() as i32,
            )
        };

        if count < 0 {
            return Err(AppError::internal("HDHomeRun discovery failed."));
        }

        Ok(normalize_discovery_results(&results[..count as usize]))
    }
}

pub struct NativeLineupProvider;

impl NativeLineupProvider {
    pub fn shared() -> SharedLineupProvider {
        Arc::new(Self)
    }
}

impl LineupProvider for NativeLineupProvider {
    fn lineup_for(&self, device: &DiscoveredDevice) -> Result<Vec<LineupChannel>, AppError> {
        let lineup_url = device.lineup_url.as_ref().ok_or_else(|| {
            AppError::internal("Selected device did not expose a lineup URL.")
        })?;

        let response = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(2))
            .build()
            .map_err(|error| AppError::internal(format!("failed to build lineup client: {error}")))?
            .get(lineup_url)
            .send()
            .and_then(|response| response.error_for_status())
            .map_err(|error| AppError::internal(format!("lineup request failed: {error}")))?;

        let entries = response
            .json::<Vec<LineupEntryWire>>()
            .map_err(|error| AppError::internal(format!("lineup parsing failed: {error}")))?;

        Ok(normalize_lineup_entries(entries))
    }
}

pub struct NativeTunerDiagnosticsProvider;

impl NativeTunerDiagnosticsProvider {
    pub fn shared() -> SharedTunerDiagnosticsProvider {
        Arc::new(Self)
    }
}

impl TunerDiagnosticsProvider for NativeTunerDiagnosticsProvider {
    fn diagnostics_for(
        &self,
        device: &DiscoveredDevice,
        remembered_context: Option<&RememberedContext>,
    ) -> Result<Vec<Result<TunerDiagnostic, String>>, AppError> {
        let device_name = CString::new(device.device_id.clone())
            .map_err(|_| AppError::internal("device id contained an unexpected NUL byte"))?;

        let mut diagnostics = Vec::new();
        unsafe {
            let hd = ffi::hdhomerun_device_create_from_str(device_name.as_ptr(), std::ptr::null_mut());
            if hd.is_null() {
                return Err(AppError::internal("failed to create HDHomeRun device handle"));
            }

            for tuner_index in 0..device.tuner_count {
                if ffi::hdhomerun_device_set_tuner(hd, tuner_index as u32) <= 0 {
                    diagnostics.push(Err(format!("failed to select tuner {tuner_index}")));
                    continue;
                }

                let mut status = ffi::HdhomerunTunerStatus::default();
                let status_result = ffi::hdhomerun_device_get_tuner_status(
                    hd,
                    std::ptr::null_mut(),
                    &mut status,
                );
                if status_result <= 0 {
                    diagnostics.push(Err(format!("failed to read status for tuner {tuner_index}")));
                    continue;
                }

                let mut vstatus = ffi::HdhomerunTunerVStatus::default();
                let vstatus_result = ffi::hdhomerun_device_get_tuner_vstatus(
                    hd,
                    std::ptr::null_mut(),
                    &mut vstatus,
                );

                let virtual_channel = (vstatus_result > 0)
                    .then(|| ffi::char_array_to_string(&vstatus.vchannel))
                    .filter(|value| !value.is_empty());
                let program_name = (vstatus_result > 0)
                    .then(|| ffi::char_array_to_string(&vstatus.name))
                    .filter(|value| !value.is_empty());

                let is_active_context = remembered_context
                    .and_then(|context| context.channel_ref.as_deref())
                    .and_then(|channel_ref| virtual_channel.as_deref().map(|value| value == channel_ref))
                    .unwrap_or(false);

                diagnostics.push(Ok(TunerDiagnostic {
                    tuner_index,
                    is_active_context,
                    channel: Some(ffi::char_array_to_string(&status.channel)).filter(|value| !value.is_empty()),
                    virtual_channel,
                    program_name,
                    lock_state: Some(ffi::char_array_to_string(&status.lock_str)).filter(|value| !value.is_empty()),
                    signal_present: status.signal_present,
                    signal_strength: Some(status.signal_strength),
                    signal_to_noise_quality: Some(status.signal_to_noise_quality),
                    symbol_error_quality: Some(status.symbol_error_quality),
                    bits_per_second: Some(status.raw_bits_per_second),
                    packets_per_second: Some(status.packets_per_second),
                    availability: if status.signal_present {
                        "available".to_string()
                    } else {
                        "no_signal".to_string()
                    },
                    warning: if vstatus_result > 0 {
                        None
                    } else {
                        Some("virtual channel details were unavailable".to_string())
                    },
                }));
            }

            ffi::hdhomerun_device_destroy(hd);
        }

        Ok(diagnostics)
    }
}

#[derive(Clone)]
pub struct StaticDeviceDiscovery {
    devices: Vec<DiscoveredDevice>,
}

impl StaticDeviceDiscovery {
    pub fn shared(devices: Vec<DiscoveredDevice>) -> SharedDeviceDiscovery {
        Arc::new(Self { devices })
    }
}

impl DeviceDiscovery for StaticDeviceDiscovery {
    fn discover(&self) -> Result<Vec<DiscoveredDevice>, AppError> {
        Ok(self.devices.clone())
    }
}

#[derive(Clone)]
pub struct StaticLineupProvider {
    lineups: std::collections::HashMap<String, Result<Vec<LineupChannel>, String>>,
}

impl StaticLineupProvider {
    pub fn shared(
        lineups: std::collections::HashMap<String, Result<Vec<LineupChannel>, String>>,
    ) -> SharedLineupProvider {
        Arc::new(Self { lineups })
    }
}

impl LineupProvider for StaticLineupProvider {
    fn lineup_for(&self, device: &DiscoveredDevice) -> Result<Vec<LineupChannel>, AppError> {
        match self.lineups.get(&device.device_ref) {
            Some(Ok(channels)) => Ok(channels.clone()),
            Some(Err(message)) => Err(AppError::internal(message.clone())),
            None => Err(AppError::internal("no lineup fixture configured for selected device")),
        }
    }
}

#[derive(Clone)]
pub struct StaticTunerDiagnosticsProvider {
    diagnostics: std::collections::HashMap<String, Vec<Result<TunerDiagnostic, String>>>,
}

impl StaticTunerDiagnosticsProvider {
    pub fn shared(
        diagnostics: std::collections::HashMap<String, Vec<Result<TunerDiagnostic, String>>>,
    ) -> SharedTunerDiagnosticsProvider {
        Arc::new(Self { diagnostics })
    }
}

impl TunerDiagnosticsProvider for StaticTunerDiagnosticsProvider {
    fn diagnostics_for(
        &self,
        device: &DiscoveredDevice,
        _remembered_context: Option<&RememberedContext>,
    ) -> Result<Vec<Result<TunerDiagnostic, String>>, AppError> {
        self.diagnostics
            .get(&device.device_ref)
            .cloned()
            .ok_or_else(|| AppError::internal("no tuner diagnostics fixture configured for selected device"))
    }
}

pub fn build_devices_response(
    devices: Vec<DiscoveredDevice>,
    remembered_context: Option<&RememberedContext>,
) -> DevicesResponse {
    let remembered_device_ref = remembered_context.and_then(|context| context.device_ref.as_deref());
    let selected_device_ref = remembered_device_ref
        .and_then(|device_ref| devices.iter().any(|device| device.device_ref == device_ref).then_some(device_ref.to_string()));

    let mut warnings = Vec::new();
    if devices.is_empty() {
        warnings.push("no HDHomeRun tuner devices are currently reachable on the local network".to_string());
    }

    if remembered_device_ref.is_some() && selected_device_ref.is_none() {
        warnings.push("remembered device is not currently available and selection is required".to_string());
    }

    let devices = devices
        .into_iter()
        .map(|device| DeviceSummary {
            device_ref: device.device_ref.clone(),
            device_id: device.device_id,
            name: device.friendly_name,
            base_url: device.base_url,
            lineup_url: device.lineup_url,
            tuner_count: device.tuner_count,
            is_legacy: device.is_legacy,
            is_selected: selected_device_ref
                .as_ref()
                .is_some_and(|selected| selected == &device.device_ref),
        })
        .collect();

    DevicesResponse {
        status: ContractEndpointStatus::Available,
        devices,
        selected_device_ref: selected_device_ref.clone(),
        selection_required: selected_device_ref.is_none(),
        warnings,
    }
}

pub fn normalize_discovery_results(results: &[ffi::HdhomerunDiscoverDeviceV3]) -> Vec<DiscoveredDevice> {
    results
        .iter()
        .map(|result| {
            let device_id = format!("{:08X}", result.device_id);
            let base_url = ffi::char_array_to_string(&result.base_url);
            let lineup_url = ffi::char_array_to_string(&result.lineup_url);

            DiscoveredDevice {
                device_ref: format!("hdhr-{}", device_id.to_lowercase()),
                friendly_name: format!("HDHomeRun {}", device_id),
                device_id,
                base_url,
                lineup_url: (!lineup_url.is_empty()).then_some(lineup_url),
                tuner_count: result.tuner_count,
                is_legacy: result.is_legacy,
            }
        })
        .collect()
}

pub fn reconcile_remembered_context(
    remembered_context: Option<RememberedContext>,
    devices: &[DiscoveredDevice],
) -> (Option<RememberedContext>, bool) {
    match remembered_context {
        Some(context)
            if context
                .device_ref
                .as_deref()
                .is_some_and(|device_ref| devices.iter().any(|device| device.device_ref == device_ref)) =>
        {
            (Some(context), false)
        }
        Some(_) => (None, true),
        None => (None, false),
    }
}

pub fn normalize_lineup_entries(entries: Vec<LineupEntryWire>) -> Vec<LineupChannel> {
    entries
        .into_iter()
        .map(|entry| {
            let tags = entry
                .tags
                .unwrap_or_default()
                .split(',')
                .map(str::trim)
                .filter(|tag| !tag.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            let playback_url = entry.url.filter(|url| !url.is_empty());
            let (availability, restriction_reason) = if tags.iter().any(|tag| tag == "drm") {
                (
                    ChannelAvailability::Restricted,
                    Some("content protection required".to_string()),
                )
            } else if playback_url.is_none() {
                (
                    ChannelAvailability::Unavailable,
                    Some("device did not provide a playback URL".to_string()),
                )
            } else {
                (ChannelAvailability::Playable, None)
            };

            LineupChannel {
                channel_ref: format!("channel:{}", entry.guide_number),
                guide_number: entry.guide_number,
                guide_name: entry.guide_name,
                tags,
                playback_url,
                availability,
                restriction_reason,
            }
        })
        .collect()
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct LineupEntryWire {
    #[serde(rename = "GuideNumber")]
    pub guide_number: String,
    #[serde(rename = "GuideName")]
    pub guide_name: String,
    #[serde(rename = "Tags")]
    pub tags: Option<String>,
    #[serde(rename = "URL")]
    pub url: Option<String>,
}

mod ffi {
    use super::c_char;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct HdhomerunDiscoverDeviceV3 {
        pub ip_addr: u32,
        pub device_type: u32,
        pub device_id: u32,
        pub tuner_count: u8,
        pub is_legacy: bool,
        pub device_auth: [c_char; 25],
        pub base_url: [c_char; 29],
        pub storage_id: [c_char; 37],
        pub lineup_url: [c_char; 128],
        pub storage_url: [c_char; 128],
    }

    impl Default for HdhomerunDiscoverDeviceV3 {
        fn default() -> Self {
            Self {
                ip_addr: 0,
                device_type: 0,
                device_id: 0,
                tuner_count: 0,
                is_legacy: false,
                device_auth: [0; 25],
                base_url: [0; 29],
                storage_id: [0; 37],
                lineup_url: [0; 128],
                storage_url: [0; 128],
            }
        }
    }

    unsafe extern "C" {
        pub fn hdhomerun_discover_find_devices_custom_v3(
            target_ip: u32,
            device_type_match: u32,
            device_id_match: u32,
            result_list: *mut HdhomerunDiscoverDeviceV3,
            max_count: i32,
        ) -> i32;

        pub fn hdhomerun_device_create_from_str(
            device_str: *const c_char,
            dbg: *mut std::ffi::c_void,
        ) -> *mut std::ffi::c_void;

        pub fn hdhomerun_device_destroy(hd: *mut std::ffi::c_void);

        pub fn hdhomerun_device_set_tuner(hd: *mut std::ffi::c_void, tuner: u32) -> i32;

        pub fn hdhomerun_device_get_tuner_status(
            hd: *mut std::ffi::c_void,
            pstatus_str: *mut *mut c_char,
            status: *mut HdhomerunTunerStatus,
        ) -> i32;

        pub fn hdhomerun_device_get_tuner_vstatus(
            hd: *mut std::ffi::c_void,
            pvstatus_str: *mut *mut c_char,
            vstatus: *mut HdhomerunTunerVStatus,
        ) -> i32;
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct HdhomerunTunerStatus {
        pub channel: [c_char; 32],
        pub lock_str: [c_char; 32],
        pub signal_present: bool,
        pub lock_supported: bool,
        pub lock_unsupported: bool,
        pub signal_strength: u32,
        pub signal_to_noise_quality: u32,
        pub symbol_error_quality: u32,
        pub raw_bits_per_second: u32,
        pub packets_per_second: u32,
    }

    impl Default for HdhomerunTunerStatus {
        fn default() -> Self {
            Self {
                channel: [0; 32],
                lock_str: [0; 32],
                signal_present: false,
                lock_supported: false,
                lock_unsupported: false,
                signal_strength: 0,
                signal_to_noise_quality: 0,
                symbol_error_quality: 0,
                raw_bits_per_second: 0,
                packets_per_second: 0,
            }
        }
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct HdhomerunTunerVStatus {
        pub vchannel: [c_char; 32],
        pub name: [c_char; 32],
        pub auth: [c_char; 32],
        pub cci: [c_char; 32],
        pub cgms: [c_char; 32],
        pub not_subscribed: bool,
        pub not_available: bool,
        pub copy_protected: bool,
    }

    impl Default for HdhomerunTunerVStatus {
        fn default() -> Self {
            Self {
                vchannel: [0; 32],
                name: [0; 32],
                auth: [0; 32],
                cci: [0; 32],
                cgms: [0; 32],
                not_subscribed: false,
                not_available: false,
                copy_protected: false,
            }
        }
    }

    pub fn char_array_to_string(value: &[c_char]) -> String {
        let len = value.iter().position(|&byte| byte == 0).unwrap_or(value.len());
        let bytes = value[..len].iter().map(|&byte| byte as u8).collect::<Vec<_>>();
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::TunerDiagnosticsProvider;
    use super::ffi::HdhomerunDiscoverDeviceV3;
    use super::{
        LineupEntryWire, build_devices_response, normalize_discovery_results,
        normalize_lineup_entries, reconcile_remembered_context,
    };
    use crate::models::{ChannelAvailability, RememberedContext, TunerDiagnostic};

    fn char_buf<const N: usize>(value: &str) -> [i8; N] {
        let mut result = [0; N];
        for (index, byte) in value.as_bytes().iter().enumerate() {
            result[index] = *byte as i8;
        }
        result
    }

    #[test]
    fn normalize_discovery_results_maps_vendor_records() {
        let results = [HdhomerunDiscoverDeviceV3 {
            device_id: 0x1234ABCD,
            tuner_count: 4,
            base_url: char_buf("http://192.168.1.10"),
            lineup_url: char_buf("http://192.168.1.10/lineup.json"),
            ..HdhomerunDiscoverDeviceV3::default()
        }];

        let normalized = normalize_discovery_results(&results);
        assert_eq!(normalized.len(), 1);
        assert_eq!(normalized[0].device_ref, "hdhr-1234abcd");
        assert_eq!(normalized[0].device_id, "1234ABCD");
        assert_eq!(normalized[0].lineup_url.as_deref(), Some("http://192.168.1.10/lineup.json"));
    }

    #[test]
    fn devices_response_marks_selection_required_when_remembered_device_is_missing() {
        let response = build_devices_response(
            vec![super::DiscoveredDevice {
                device_ref: "hdhr-1234abcd".to_string(),
                device_id: "1234ABCD".to_string(),
                friendly_name: "HDHomeRun 1234ABCD".to_string(),
                base_url: "http://192.168.1.10".to_string(),
                lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
                tuner_count: 4,
                is_legacy: false,
            }],
            Some(&RememberedContext {
                device_ref: Some("hdhr-deadbeef".to_string()),
                channel_ref: Some("5.1".to_string()),
                auto_resume: true,
                updated_at: "2026-05-20T23:32:16Z".to_string(),
            }),
        );

        assert!(response.selection_required);
        assert!(response.selected_device_ref.is_none());
        assert_eq!(response.warnings.len(), 1);
    }

    #[test]
    fn reconcile_remembered_context_clears_stale_device() {
        let (context, cleared) = reconcile_remembered_context(
            Some(RememberedContext {
                device_ref: Some("hdhr-deadbeef".to_string()),
                channel_ref: Some("5.1".to_string()),
                auto_resume: true,
                updated_at: "2026-05-20T23:32:16Z".to_string(),
            }),
            &[super::DiscoveredDevice {
                device_ref: "hdhr-1234abcd".to_string(),
                device_id: "1234ABCD".to_string(),
                friendly_name: "HDHomeRun 1234ABCD".to_string(),
                base_url: "http://192.168.1.10".to_string(),
                lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
                tuner_count: 4,
                is_legacy: false,
            }],
        );

        assert!(cleared);
        assert!(context.is_none());
    }

    #[test]
    fn normalize_lineup_entries_marks_drm_and_missing_urls() {
        let channels = normalize_lineup_entries(vec![
            LineupEntryWire {
                guide_number: "5.1".to_string(),
                guide_name: "News".to_string(),
                tags: Some("favorite,drm".to_string()),
                url: Some("http://192.168.1.10/auto/v5.1".to_string()),
            },
            LineupEntryWire {
                guide_number: "7.2".to_string(),
                guide_name: "Weather".to_string(),
                tags: None,
                url: None,
            },
        ]);

        assert_eq!(channels[0].availability, ChannelAvailability::Restricted);
        assert_eq!(channels[1].availability, ChannelAvailability::Unavailable);
    }

    #[test]
    fn static_tuner_diagnostics_provider_keeps_fixture_shape() {
        let mut fixtures = std::collections::HashMap::new();
        fixtures.insert(
            "hdhr-1234abcd".to_string(),
            vec![Ok(TunerDiagnostic {
                tuner_index: 0,
                is_active_context: true,
                channel: Some("auto:5".to_string()),
                virtual_channel: Some("5.1".to_string()),
                program_name: Some("News".to_string()),
                lock_state: Some("8vsb".to_string()),
                signal_present: true,
                signal_strength: Some(90),
                signal_to_noise_quality: Some(100),
                symbol_error_quality: Some(100),
                bits_per_second: Some(19000000),
                packets_per_second: Some(2000),
                availability: "available".to_string(),
                warning: None,
            })],
        );

        let provider = super::StaticTunerDiagnosticsProvider { diagnostics: fixtures };
        let results = provider
            .diagnostics_for(
                &super::DiscoveredDevice {
                    device_ref: "hdhr-1234abcd".to_string(),
                    device_id: "1234ABCD".to_string(),
                    friendly_name: "HDHomeRun 1234ABCD".to_string(),
                    base_url: "http://192.168.1.10".to_string(),
                    lineup_url: Some("http://192.168.1.10/lineup.json".to_string()),
                    tuner_count: 4,
                    is_legacy: false,
                },
                None,
            )
            .expect("diagnostics");

        assert_eq!(results.len(), 1);
        assert!(results[0].is_ok());
    }
}