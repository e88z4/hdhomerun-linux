use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;

use crate::device::{
    NativeDeviceDiscovery, NativeLineupProvider, NativeTunerDiagnosticsProvider,
    SharedDeviceDiscovery, SharedLineupProvider, SharedTunerDiagnosticsProvider,
    StaticDeviceDiscovery, StaticLineupProvider, StaticTunerDiagnosticsProvider,
};
use crate::dvr::{NativeDvrProvider, SharedDvrProvider, StaticDvrFixtures, StaticDvrProvider};
use crate::guide::{NativeGuideProvider, SharedGuideProvider, StaticGuideProvider};
use crate::http::routes::router;
use crate::models::{
    BackendRuntimeState, BackendRuntimeStatus, LaunchMode, LineupChannel, TunerDiagnostic,
};
use crate::playback::{PlaybackService, SharedPlaybackService, StaticPlayerAdapter, StaticPlayerAdapterFixtures};
use crate::state::StateStore;

#[derive(Clone)]
pub struct AppState {
    runtime_state: Arc<RwLock<BackendRuntimeState>>,
    state_store: StateStore,
    device_discovery: SharedDeviceDiscovery,
    lineup_provider: SharedLineupProvider,
    guide_provider: SharedGuideProvider,
    dvr_provider: SharedDvrProvider,
    tuner_diagnostics_provider: SharedTunerDiagnosticsProvider,
    playback_service: SharedPlaybackService,
    lineup_cache: Arc<RwLock<HashMap<String, Vec<LineupChannel>>>>,
    service_version: Arc<str>,
    api_version: Arc<str>,
}

impl AppState {
    pub fn new_default(state_dir: PathBuf) -> Self {
        Self {
            runtime_state: Arc::new(RwLock::new(BackendRuntimeState {
                status: BackendRuntimeStatus::Ready,
                started_at: Some("boot".to_string()),
                last_health_check_at: Some("boot".to_string()),
                launch_mode: LaunchMode::BundledAutoStart,
            })),
            state_store: StateStore::new(state_dir.clone()),
            device_discovery: NativeDeviceDiscovery::shared(),
            lineup_provider: NativeLineupProvider::shared(),
            guide_provider: NativeGuideProvider::shared(),
            dvr_provider: NativeDvrProvider::shared(),
            tuner_diagnostics_provider: NativeTunerDiagnosticsProvider::shared(),
            playback_service: PlaybackService::shared_default(state_dir.clone()),
            lineup_cache: Arc::new(RwLock::new(HashMap::new())),
            service_version: env!("CARGO_PKG_VERSION").into(),
            api_version: crate::API_VERSION.into(),
        }
    }

    pub fn for_tests(state_dir: PathBuf) -> Self {
        Self::for_tests_with_fixtures(state_dir, Vec::new(), HashMap::new(), HashMap::new())
    }

    pub fn for_tests_with_devices(
        state_dir: PathBuf,
        devices: Vec<crate::device::DiscoveredDevice>,
    ) -> Self {
        Self::for_tests_with_fixtures(state_dir, devices, HashMap::new(), HashMap::new())
    }

    pub fn for_tests_with_fixtures(
        state_dir: PathBuf,
        devices: Vec<crate::device::DiscoveredDevice>,
        lineups: HashMap<String, Result<Vec<LineupChannel>, String>>,
        tuner_diagnostics: HashMap<String, Vec<Result<TunerDiagnostic, String>>>,
    ) -> Self {
        Self::for_tests_with_guide_fixtures(state_dir, devices, lineups, tuner_diagnostics, HashMap::new())
    }

    pub fn for_tests_with_guide_fixtures(
        state_dir: PathBuf,
        devices: Vec<crate::device::DiscoveredDevice>,
        lineups: HashMap<String, Result<Vec<LineupChannel>, String>>,
        tuner_diagnostics: HashMap<String, Vec<Result<TunerDiagnostic, String>>>,
        guide_programs: HashMap<String, String>,
    ) -> Self {
        Self::for_tests_with_playback_fixtures(
            state_dir,
            devices,
            lineups,
            tuner_diagnostics,
            guide_programs,
            StaticDvrFixtures::default(),
            StaticPlayerAdapterFixtures::default(),
        )
    }

    pub fn for_tests_with_playback_fixtures(
        state_dir: PathBuf,
        devices: Vec<crate::device::DiscoveredDevice>,
        lineups: HashMap<String, Result<Vec<LineupChannel>, String>>,
        tuner_diagnostics: HashMap<String, Vec<Result<TunerDiagnostic, String>>>,
        guide_programs: HashMap<String, String>,
        dvr_fixtures: StaticDvrFixtures,
        playback_fixtures: StaticPlayerAdapterFixtures,
    ) -> Self {
        Self {
            runtime_state: Arc::new(RwLock::new(BackendRuntimeState {
                status: BackendRuntimeStatus::Ready,
                started_at: Some("boot".to_string()),
                last_health_check_at: Some("boot".to_string()),
                launch_mode: LaunchMode::BundledAutoStart,
            })),
            state_store: StateStore::new(state_dir),
            device_discovery: StaticDeviceDiscovery::shared(devices),
            lineup_provider: StaticLineupProvider::shared(lineups),
            guide_provider: StaticGuideProvider::shared(guide_programs),
            dvr_provider: StaticDvrProvider::shared(dvr_fixtures),
            tuner_diagnostics_provider: StaticTunerDiagnosticsProvider::shared(tuner_diagnostics),
            playback_service: PlaybackService::shared_with_adapter(StaticPlayerAdapter::shared(playback_fixtures)),
            lineup_cache: Arc::new(RwLock::new(HashMap::new())),
            service_version: env!("CARGO_PKG_VERSION").into(),
            api_version: crate::API_VERSION.into(),
        }
    }

    pub fn state_store(&self) -> &StateStore {
        &self.state_store
    }

    pub fn device_discovery(&self) -> SharedDeviceDiscovery {
        Arc::clone(&self.device_discovery)
    }

    pub fn lineup_provider(&self) -> SharedLineupProvider {
        Arc::clone(&self.lineup_provider)
    }

    pub fn guide_provider(&self) -> SharedGuideProvider {
        Arc::clone(&self.guide_provider)
    }

    pub fn dvr_provider(&self) -> SharedDvrProvider {
        Arc::clone(&self.dvr_provider)
    }

    pub fn tuner_diagnostics_provider(&self) -> SharedTunerDiagnosticsProvider {
        Arc::clone(&self.tuner_diagnostics_provider)
    }

    pub fn playback_service(&self) -> SharedPlaybackService {
        Arc::clone(&self.playback_service)
    }

    pub async fn cached_lineup(&self, device_ref: &str) -> Option<Vec<LineupChannel>> {
        self.lineup_cache.read().await.get(device_ref).cloned()
    }

    pub async fn store_cached_lineup(&self, device_ref: String, channels: Vec<LineupChannel>) {
        self.lineup_cache.write().await.insert(device_ref, channels);
    }

    pub async fn runtime_state(&self) -> BackendRuntimeState {
        self.runtime_state.read().await.clone()
    }

    pub fn service_version(&self) -> &str {
        &self.service_version
    }

    pub fn api_version(&self) -> &str {
        &self.api_version
    }
}

pub fn build_app(state: AppState) -> Router {
    router(state)
}