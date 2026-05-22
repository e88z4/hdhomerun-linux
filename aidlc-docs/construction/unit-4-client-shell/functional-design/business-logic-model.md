# Business Logic Model

## Overview

Unit 4 defines how the Qt/QML desktop client turns the backend's loopback API into a coherent living-room-style Linux TV experience. Its job is not to own playback or tuner logic directly, but to orchestrate launch, browsing, persistent playback presentation, diagnostics visibility, and failure recovery in a way that feels immediate and understandable.

## Core Workflows

### 1. Launch and Bootstrap
1. Client starts and shows a launch overlay immediately.
2. Client probes backend health and starts or waits for the bundled backend when necessary.
3. Client requests bootstrap and runtime state.
4. If remembered context is valid and auto-resume is available, the client prepares the playback stage and restores the last context.
5. If remembered device state is invalid or missing, the client enters device-selection mode without crashing or showing an empty main shell.

### 2. Device Selection and Channel Browsing
1. Client loads discovered devices from the backend.
2. Client presents the active device in a visible header selector and the current lineup in a persistent left rail.
3. When the user changes devices, the client requests backend selection-state change and reloads lineup plus diagnostics context.
4. The channel rail stays available even during playback so switching remains lightweight.

### 3. Start Playback from the Channel Rail
1. User selects a playable channel from the left rail.
2. Client immediately marks the playback stage as loading and preserves visible channel focus.
3. Client sends a playback start command to the backend.
4. Client projects returned session state into loading, playing, or failure UI without parsing adapter internals.

### 4. Persistent Playback Presentation
1. Once playback is active, the central playback stage remains the dominant visual region.
2. Channel changes reuse the same viewing stage while the client updates titles, state badges, and summary diagnostics.
3. The client never tears down the whole shell just because playback changes or fails.

### 5. Diagnostics Drawer
1. User opens the diagnostics drawer from a summary indicator or error state.
2. Client requests tuner diagnostics for the selected device.
3. Drawer shows the active playback context first and then full tuner visibility.
4. Closing the drawer returns the user to the same playback context without losing state.

### 6. Failure and Retry Recovery
1. If bootstrap fails, no devices are found, or playback start fails, the client shows an inline recoverable state rather than a modal dead end.
2. Retry actions remain close to the failure surface.
3. Diagnostics and lineup context remain available whenever practical so the user can recover without restarting the app.

## Functional Responsibilities
- Own desktop launch, navigation, and view-state projection.
- Turn backend session state into a stable playback stage.
- Keep channel browsing available alongside playback.
- Surface diagnostics without displacing playback context.
- Preserve enough context for retry-oriented recovery UX.

## State Transitions

### Shell Lifecycle
- `launching` -> `backend_wait`
- `backend_wait` -> `restoring_context`
- `restoring_context` -> `device_selection`
- `restoring_context` -> `ready`
- `ready` -> `playback_loading`
- `playback_loading` -> `playing`
- `playback_loading` -> `playback_failed`
- `playing` -> `switching_channel`
- `switching_channel` -> `playing`
- `switching_channel` -> `playback_failed`

### Diagnostics Visibility
- `collapsed` -> `expanded`
- `expanded` -> `collapsed`

## Testable Properties
- **Invariant**: The client never assumes playback is active without backend confirmation.
- **Invariant**: Channel browsing remains available during playing, switching, and recoverable failure states.
- **Invariant**: Device selection changes are explicit and do not silently replace an active playback context.
- **Invariant**: Failure UI retains channel and device context whenever the backend still provides them.