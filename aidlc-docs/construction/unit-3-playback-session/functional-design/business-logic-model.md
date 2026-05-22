# Business Logic Model

## Overview

Unit 3 defines how the backend owns and reuses one persistent live-TV playback session while handing direct HDHomeRun stream URLs into a backend-controlled `mpv` adapter. Its job is to turn device-side playback sources from Unit 2 into reliable start, stop, switch, and retry behavior without pushing orchestration responsibility into the future Qt/QML client.

## Core Workflows

### 1. Start Playback from Channel Selection
1. Client requests playback start for a selected channel on the selected device.
2. Backend validates that device selection and channel resolution are still valid through Unit 2 data.
3. Backend resolves the canonical playback URL for the requested channel.
4. Backend ensures a persistent player session exists, starting the `mpv` adapter process if needed.
5. Backend commands the player adapter to load the resolved stream URL.
6. If startup fails in a clearly retryable way, backend performs one bounded automatic retry.
7. Backend returns structured current-session state or a structured retryable failure result.

### 2. Persistent Session Reuse on Channel Switch
1. Client requests a channel switch while playback session state already exists.
2. Backend keeps the existing player session alive.
3. Backend resolves the next channel's playback source through Unit 2 data.
4. Backend commands the adapter to replace the current stream inside the same persistent session.
5. Backend updates current-session state without tearing down the entire playback runtime unless recovery rules require it.

### 3. Stop Playback
1. Client requests playback stop.
2. Backend stops the active stream inside the persistent player session.
3. Backend keeps or tears down the adapter process according to session lifecycle rules, but the public session state becomes stopped immediately.
4. Backend updates remembered playback context for future restore behavior.

### 4. Current Session State Reporting
1. Client requests current playback state.
2. Backend returns normalized session state derived from orchestration state and adapter status.
3. Response includes selected device, current channel, session status, retryability metadata, and any user-safe warnings.
4. Client does not infer session state by talking directly to the player or HDHomeRun device.

### 5. Retryable Startup Failure Handling
1. Backend detects a startup or tune failure from the player adapter or initial stream load path.
2. Backend classifies whether the failure is retryable.
3. If retryable and no retry has yet been consumed for this start attempt, backend performs one bounded automatic retry.
4. If the retry still fails, backend returns a structured failure state that supports inline retry UX.
5. Backend preserves enough session context that an explicit user retry can attempt the same channel again without rebuilding unrelated state.

### 6. Restore-Aware Session Bootstrap
1. Client starts and requests bootstrap state.
2. Backend combines remembered playback context from Unit 1 and verified device availability from Unit 2.
3. If valid remembered playback context exists, Unit 3 can later request session start using the remembered channel inside the same persistent adapter model.
4. If playback prerequisites are no longer valid, backend surfaces a structured non-playing state rather than guessing replacement playback state.

## Functional Responsibilities
- Own the persistent playback session lifecycle in the backend.
- Start, stop, and switch channels without delegating orchestration to the client.
- Reuse one player session across channel switches where possible.
- Translate direct HDHomeRun playback URLs into adapter commands.
- Perform one bounded automatic retry for clearly retryable startup failures.
- Expose stable session-state and playback-command contracts for the client.

## State Transitions

### Playback Session Lifecycle
- `idle` -> `starting`
- `starting` -> `playing`
- `starting` -> `retrying_start`
- `retrying_start` -> `playing`
- `retrying_start` -> `failed`
- `playing` -> `switching`
- `switching` -> `playing`
- `switching` -> `failed`
- `playing` -> `stopped`
- `failed` -> `starting`

### Adapter Availability
- `not_started` -> `process_starting`
- `process_starting` -> `adapter_ready`
- `adapter_ready` -> `adapter_loading_stream`
- `adapter_loading_stream` -> `adapter_streaming`
- `adapter_loading_stream` -> `adapter_error`

## Testable Properties
- **Invariant**: Start, stop, and switch-channel requests never bypass backend orchestration state.
- **Invariant**: Channel switching reuses the same logical playback session unless a recovery rule explicitly requires restart.
- **Invariant**: Automatic retry is bounded to one retry per startup attempt.
- **Invariant**: Current playback state remains structured and user-safe even when adapter startup fails.
- **Round-trip**: A session state update always refers to a valid selected device and playback source resolved through Unit 2.