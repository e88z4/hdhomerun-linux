# Domain Entities

## 1. AppShellState
- **Purpose**: Represents the top-level client shell lifecycle.
- **Fields**:
  - `phase`: `launching | backend_wait | restoring_context | device_selection | ready | playback_loading | playing | switching_channel | playback_failed`
  - `selectedDeviceRef`
  - `focusedChannelRef`
  - `diagnosticsVisible`
  - `blockingMessage`

## 2. LaunchOverlayState
- **Purpose**: Represents the startup overlay shown while the client verifies backend readiness and restore context.
- **Fields**:
  - `headline`
  - `detail`
  - `isRetryVisible`
  - `status`: `working | failed | hidden`

## 3. DeviceSelectorItem
- **Purpose**: Represents one device choice in the client shell.
- **Fields**:
  - `deviceRef`
  - `displayName`
  - `isSelected`
  - `isAvailable`

## 4. ChannelRailItem
- **Purpose**: Represents one channel row in the persistent channel browser.
- **Fields**:
  - `channelRef`
  - `guideNumber`
  - `guideName`
  - `availability`
  - `isFocused`
  - `isPlaying`

## 5. PlaybackStageState
- **Purpose**: Represents the central viewing stage.
- **Fields**:
  - `sessionStatus`
  - `title`
  - `subtitle`
  - `warning`
  - `failureCode`
  - `retryEnabled`

## 6. DiagnosticsDrawerState
- **Purpose**: Represents the expanded diagnostics panel.
- **Fields**:
  - `isOpen`
  - `summary`
  - `activeTunerIndex`
  - `tunerRows`
  - `lastUpdatedAt`

## 7. InlineFailureViewState
- **Purpose**: Represents a recoverable failure panel inside the shell.
- **Fields**:
  - `code`
  - `message`
  - `retryLabel`
  - `showDiagnosticsAction`
  - `preservedChannelRef`

## Entity Relationships
- `AppShellState.selectedDeviceRef` maps to the active backend device context.
- `AppShellState.focusedChannelRef` maps to the currently focused or playing `ChannelRailItem`.
- `PlaybackStageState` is derived from backend playback current or playback command responses.
- `DiagnosticsDrawerState` is derived from backend tuner diagnostics responses.
- `InlineFailureViewState` is typically derived from backend playback or bootstrap failures.

## Entity Constraints
- Only one `AppShellState` exists at a time.
- `ChannelRailItem.isPlaying` must align with backend-confirmed playback state.
- `PlaybackStageState.retryEnabled` must not be true when the backend exposes a non-retryable failure.
- `DeviceSelectorItem.isSelected` must not contradict backend-selected device context.