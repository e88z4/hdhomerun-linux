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

## DVR Workspace Component
- `loadDvrHome() -> DvrHomeView`
- `selectRecording(recordingId: string) -> RecordingDetailView`
- `filterRecordings(filter: RecordingFilter) -> RecordingSummary[]`
- `openRuleEditor(context: RuleEditorContext) -> RuleEditorView`

## Recording Rule Editor Component
- `createSeriesRule(request: CreateSeriesRuleRequest) -> RecordingRuleView`
- `createOneTimeRule(request: CreateOneTimeRuleRequest) -> RecordingRuleView`
- `updateRule(ruleId: string, request: UpdateRecordingRuleRequest) -> RecordingRuleView`
- `deleteRule(ruleId: string) -> DeleteRuleResult`

## DVR Integration Component
- `fetchRecordingRules(deviceAuth: DeviceAuthRef) -> RecordingRuleRecord[]`
- `fetchEpisodes(query: EpisodeQuery) -> EpisodeRecord[]`
- `fetchUpcomingRecordings(deviceAuth: DeviceAuthRef) -> UpcomingRecordingRecord[]`
- `fetchRecordedFiles(storageRef: StorageRef) -> RecordedFileRecord[]`
- `syncRecordingEvents(storageRef: StorageRef) -> SyncResult`

## Recording Catalog Resolver
- `resolvePreferredStorage(sources: StorageSourceRecord[]) -> StorageSourceRecord[]`
- `buildRecordingCatalog(sources: StorageSourceRecord[]) -> RecordingCatalog`
- `resolvePlaybackTarget(recordingId: string) -> RecordingPlaybackTarget`
- `resolveDeleteCommand(recordingId: string) -> RecordingDeleteTarget`

## Playback Stop Coordinator
- `stopLivePlayback(sessionId: string) -> PlaybackSessionStopped`

## Backend API Host DVR Endpoints
- `GET /api/dvr/readiness -> DvrReadinessView`
- `GET /api/dvr/home -> DvrHomeView`
- `GET /api/dvr/recordings -> RecordingSummary[]`
- `GET /api/dvr/recordings/{recordingId} -> RecordingDetailView`
- `POST /api/dvr/recordings/{recordingId}/play -> PlaybackSessionView`
- `POST /api/dvr/recordings/{recordingId}/delete -> DeleteRecordingResult`
- `GET /api/dvr/rules -> RecordingRuleView[]`
- `POST /api/dvr/rules/series -> RecordingRuleView`
- `POST /api/dvr/rules/one-time -> RecordingRuleView`
- `PUT /api/dvr/rules/{ruleId} -> RecordingRuleView`
- `DELETE /api/dvr/rules/{ruleId} -> DeleteRuleResult`
- `GET /api/dvr/upcoming -> UpcomingRecordingView[]`
- `POST /api/playback/stop -> PlaybackSessionStopped`