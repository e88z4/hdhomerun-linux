# Domain Entities

## 1. PlaybackSessionState
- **Purpose**: Represents the backend-owned current live-TV playback session.
- **Fields**:
  - `sessionId`
  - `status`: `idle | starting | retrying_start | playing | switching | stopped | failed`
  - `selectedDeviceRef`
  - `channelRef`
  - `playbackUrl`
  - `retryCount`
  - `warning`
  - `updatedAt`

## 2. PlayerAdapterState
- **Purpose**: Represents the backend view of the persistent `mpv` adapter runtime.
- **Fields**:
  - `adapterStatus`: `not_started | process_starting | adapter_ready | adapter_loading_stream | adapter_streaming | adapter_error`
  - `processId`
  - `lastCommand`
  - `lastError`
  - `updatedAt`

## 3. PlaybackCommandRequest
- **Purpose**: Represents a validated playback command coming from the client.
- **Fields**:
  - `commandType`: `start | stop | switch_channel`
  - `deviceRef`
  - `channelRef`
  - `requestedAt`

## 4. PlaybackStartResult
- **Purpose**: Represents the outcome of a playback start or switch attempt.
- **Fields**:
  - `sessionState`
  - `adapterState`
  - `succeeded`
  - `usedAutomaticRetry`
  - `warnings`

## 5. RetryablePlaybackFailure
- **Purpose**: Represents a structured playback failure that may support retry-oriented UX.
- **Fields**:
  - `code`
  - `message`
  - `retryable`
  - `retryConsumed`
  - `channelRef`
  - `deviceRef`

## 6. PlaybackCurrentResponseModel
- **Purpose**: Represents the normalized API contract returned by `/api/playback/current`.
- **Fields**:
  - `sessionState`
  - `adapterState`
  - `currentChannel`
  - `selectedDeviceRef`
  - `warnings`

## Entity Relationships
- `PlaybackSessionState.selectedDeviceRef` must refer to the currently selected Unit 2 device context.
- `PlaybackSessionState.channelRef` must refer to a valid Unit 2 channel when the session is in `starting`, `retrying_start`, `playing`, or `switching` state.
- `PlaybackStartResult` contains the resulting `PlaybackSessionState` and `PlayerAdapterState`.
- `RetryablePlaybackFailure` may be attached to or derived from `PlaybackSessionState.status=failed`.
- `PlaybackCurrentResponseModel` reflects the latest backend-owned `PlaybackSessionState` and `PlayerAdapterState`.

## Entity Constraints
- At most one active `PlaybackSessionState` exists in v1.
- `retryCount` is bounded to one automatic retry for a given start attempt.
- `PlaybackCommandRequest.commandType=switch_channel` requires an existing logical session context.
- `PlaybackSessionState.playbackUrl` must come from Unit 2 playback-source resolution rather than a fabricated source.
- `PlayerAdapterState.adapterStatus=adapter_streaming` must align with `PlaybackSessionState.status=playing`.