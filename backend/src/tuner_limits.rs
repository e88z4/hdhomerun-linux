use std::collections::HashMap;
use std::sync::{Arc, Mutex};

const RECORDING_RESERVE_ENV: &str = "HDHR_BACKEND_TUNER_RESERVE_RECORDING";
const FRONTEND_MIN_FREE_ENV: &str = "HDHR_BACKEND_FRONTEND_MIN_FREE";
const HEADLESS_MAX_ENV: &str = "HDHR_BACKEND_HEADLESS_MAX_PER_DEVICE";

#[derive(Clone)]
pub struct TunerPermitManager {
    inner: Arc<TunerPermitManagerInner>,
}

struct TunerPermitManagerInner {
    policy: TunerPermitPolicy,
    state: Mutex<HashMap<String, DeviceUsage>>,
}

#[derive(Clone, Copy)]
struct TunerPermitPolicy {
    recording_reserve: u8,
    frontend_min_free: u8,
    headless_max_per_device: Option<u8>,
}

#[derive(Default, Clone, Copy)]
struct DeviceUsage {
    tuner_count: u8,
    frontend_live: u8,
    headless: u8,
}

#[derive(Clone, Copy)]
enum PermitKind {
    FrontendLive,
    Headless,
}

pub struct TunerPermit {
    manager: Arc<TunerPermitManagerInner>,
    device_ref: String,
    kind: PermitKind,
    released: bool,
}

impl TunerPermitManager {
    pub fn from_env() -> Self {
        let recording_reserve = env_u8(RECORDING_RESERVE_ENV, 0);
        let frontend_min_free = env_u8(FRONTEND_MIN_FREE_ENV, 1);
        let headless_max_per_device = std::env::var(HEADLESS_MAX_ENV)
            .ok()
            .and_then(|value| value.trim().parse::<u8>().ok())
            .filter(|value| *value > 0);

        Self {
            inner: Arc::new(TunerPermitManagerInner {
                policy: TunerPermitPolicy {
                    recording_reserve,
                    frontend_min_free,
                    headless_max_per_device,
                },
                state: Mutex::new(HashMap::new()),
            }),
        }
    }

    pub fn try_acquire_frontend(&self, device_ref: &str, tuner_count: u8) -> Result<TunerPermit, String> {
        self.try_acquire(device_ref, tuner_count, PermitKind::FrontendLive)
    }

    pub fn try_acquire_headless(&self, device_ref: &str, tuner_count: u8) -> Result<TunerPermit, String> {
        self.try_acquire(device_ref, tuner_count, PermitKind::Headless)
    }

    fn try_acquire(&self, device_ref: &str, tuner_count: u8, kind: PermitKind) -> Result<TunerPermit, String> {
        if tuner_count == 0 {
            return Err("device reported zero tuners and cannot start live streams".to_string());
        }

        let mut state = self
            .inner
            .state
            .lock()
            .expect("tuner permit manager state lock");
        let usage = state.entry(device_ref.to_string()).or_insert_with(|| DeviceUsage {
            tuner_count,
            ..DeviceUsage::default()
        });

        usage.tuner_count = usage.tuner_count.max(tuner_count);
        let tuner_count = usage.tuner_count;

        let recording_reserve = self
            .inner
            .policy
            .recording_reserve
            .min(tuner_count.saturating_sub(1));
        let usable = tuner_count.saturating_sub(recording_reserve);

        if usable == 0 {
            return Err("all tuners are reserved for recording priority".to_string());
        }

        let used = usage.frontend_live.saturating_add(usage.headless);
        if used >= usable {
            return Err(format!(
                "all tuners are busy (usable={}, in_use={})",
                usable, used
            ));
        }

        match kind {
            PermitKind::FrontendLive => {
                if usage.frontend_live > 0 {
                    return Err("frontend live playback already has an active tuner permit".to_string());
                }
                usage.frontend_live = usage.frontend_live.saturating_add(1);
            }
            PermitKind::Headless => {
                if let Some(max_headless) = self.inner.policy.headless_max_per_device {
                    if usage.headless >= max_headless {
                        return Err(format!(
                            "headless stream limit reached for device (max={})",
                            max_headless
                        ));
                    }
                }

                if usage.frontend_live == 0 {
                    let min_free_for_frontend = self
                        .inner
                        .policy
                        .frontend_min_free
                        .min(usable.saturating_sub(1));
                    let remaining_after = usable.saturating_sub(used.saturating_add(1));
                    if remaining_after < min_free_for_frontend {
                        return Err(format!(
                            "headless stream denied to keep {} tuner(s) available for frontend playback",
                            min_free_for_frontend
                        ));
                    }
                }

                usage.headless = usage.headless.saturating_add(1);
            }
        }

        Ok(TunerPermit {
            manager: Arc::clone(&self.inner),
            device_ref: device_ref.to_string(),
            kind,
            released: false,
        })
    }
}

impl TunerPermit {
    pub fn device_ref(&self) -> &str {
        &self.device_ref
    }
}

impl Drop for TunerPermit {
    fn drop(&mut self) {
        if self.released {
            return;
        }

        let mut state = self
            .manager
            .state
            .lock()
            .expect("tuner permit manager state lock");
        if let Some(usage) = state.get_mut(&self.device_ref) {
            match self.kind {
                PermitKind::FrontendLive => {
                    usage.frontend_live = usage.frontend_live.saturating_sub(1);
                }
                PermitKind::Headless => {
                    usage.headless = usage.headless.saturating_sub(1);
                }
            }

            if usage.frontend_live == 0 && usage.headless == 0 {
                state.remove(&self.device_ref);
            }
        }

        self.released = true;
    }
}

fn env_u8(name: &str, default: u8) -> u8 {
    std::env::var(name)
        .ok()
        .and_then(|value| value.trim().parse::<u8>().ok())
        .unwrap_or(default)
}
