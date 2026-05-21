# Component Methods

## Desktop Client Shell
- `initializeApp() -> AppBootstrapState`
- `ensureBackendAvailable() -> BackendAvailability`
- `openDiagnosticsPanel() -> void`
- `showError(message: UiError) -> void`

## Channel Browser Component
- `loadDevices() -> DeviceSummary[]`
- `selectDevice(deviceId: string) -> DeviceSelectionResult`
- `loadChannels(deviceId: string) -> ChannelSummary[]`
- `playChannel(channelId: string) -> PlaybackSessionView`

## Embedded Player Component
- `bindSession(session: PlaybackSessionView) -> void`
- `showLoading(channelId: string) -> void`
- `showPlaying(session: PlaybackSessionView) -> void`
- `showPlaybackFailure(error: PlaybackFailureView) -> void`

## Diagnostics Panel Component
- `loadDiagnostics() -> DiagnosticsView`
- `refreshDiagnostics() -> DiagnosticsView`
- `showRetryOptions(failure: PlaybackFailureView) -> void`

## Backend API Host
- `GET /api/devices -> DeviceSummary[]`
- `POST /api/devices/select -> DeviceSelectionResult`
- `GET /api/channels -> ChannelSummary[]`
- `POST /api/playback/start -> PlaybackSessionView`
- `POST /api/playback/switch -> PlaybackSessionView`
- `GET /api/playback/current -> PlaybackSessionView`
- `GET /api/diagnostics -> DiagnosticsView`
- `POST /api/playback/retry -> PlaybackSessionView | PlaybackFailureView`

## Device Integration Component
- `discoverDevices() -> DeviceRecord[]`
- `getLineup(deviceRef: DeviceRef) -> ChannelRecord[]`
- `getTunerStatus(deviceRef: DeviceRef, tunerRef: TunerRef) -> TunerStatusRecord`
- `getSignalInfo(deviceRef: DeviceRef, tunerRef: TunerRef) -> SignalInfoRecord`
- `resolvePlaybackSource(deviceRef: DeviceRef, channelRef: ChannelRef) -> PlaybackSource`

## Playback Session Controller
- `restoreLastSession() -> RestoreSessionResult`
- `startPlayback(deviceRef: DeviceRef, channelRef: ChannelRef) -> PlaybackSession`
- `switchChannel(sessionId: string, channelRef: ChannelRef) -> PlaybackSession`
- `getCurrentSession() -> PlaybackSession | null`
- `retryLastFailure() -> PlaybackSession | PlaybackFailure`

## Playback Engine Adapter
- `initializePlayer() -> void`
- `loadSource(source: PlaybackSource) -> PlayerState`
- `replaceSource(source: PlaybackSource) -> PlayerState`
- `getPlayerState() -> PlayerState`
- `stop() -> void`

## State Store
- `loadRememberedDevice() -> DeviceRef | null`
- `saveRememberedDevice(deviceRef: DeviceRef) -> void`
- `loadLastChannel() -> ChannelRef | null`
- `saveLastChannel(channelRef: ChannelRef) -> void`
- `loadUiPreferences() -> UiPreferenceRecord`

## Service Launcher and Supervisor
- `startBundledBackend() -> ProcessHandle`
- `probeBackendHealth() -> HealthStatus`
- `connectToManagedBackend() -> HealthStatus`