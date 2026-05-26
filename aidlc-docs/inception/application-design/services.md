# Services

## 1. Client Gateway Service
- **Purpose**: Present the desktop client with a small stable API surface over loopback HTTP.
- **Responsibilities**:
  - route client requests to backend application services
  - validate input payloads and reject malformed requests
  - translate internal errors into safe client-facing responses
- **Security Notes**:
  - bind to loopback only
  - avoid exposing stack traces or internal paths in error responses
  - keep logging structured and free of sensitive data

## 2. Device Service
- **Purpose**: Manage device discovery, selection, and lineup access.
- **Responsibilities**:
  - discover HDHomeRun devices
  - maintain active device context
  - fetch device lineup and tuner-related metadata

## 3. Playback Service
- **Purpose**: Manage live playback session orchestration.
- **Responsibilities**:
  - start playback for a selected channel
  - switch channels inside one persistent session
  - coordinate playback-engine operations and failure recovery

## 4. Diagnostics Service
- **Purpose**: Expose tuner and signal visibility to the client.
- **Responsibilities**:
  - aggregate tuner status and signal metrics
  - provide summary and expanded diagnostics payloads
  - help produce actionable retry guidance on failures

## 5. State Service
- **Purpose**: Manage canonical remembered application state.
- **Responsibilities**:
  - persist last device and last channel
  - expose restore-on-launch behavior
  - separate canonical backend state from optional client UI preferences

## 6. Runtime Supervision Service
- **Purpose**: Manage backend startup modes.
- **Responsibilities**:
  - support desktop-app-driven backend startup as the main path
  - support pre-started managed service mode where available
  - report health and readiness to the desktop client

## Orchestration Pattern
- Desktop app launches.
- Desktop app checks backend readiness.
- If backend is not running, bundled app-start starts it.
- Client requests restore state from backend.
- Backend returns remembered device and channel context when possible.
- Client loads device and channel views.
- User selects a channel or an auto-restored session begins playback.
- Diagnostics remain available as summary plus dedicated drawer content.

## DVR Increment Services

## 7. DVR Library Service
- **Purpose**: Provide the client with a unified DVR home view and recorded-library model.
- **Responsibilities**:
  - compute DVR readiness from device discovery, DeviceAuth availability, and storage availability
  - aggregate recordings, upcoming state, and summary counts into one split-view-friendly payload
  - order storage inputs so local sources win before non-local sources when building the visible library

## 8. Recording Rule Service
- **Purpose**: Own creation, editing, listing, and deletion of recording rules.
- **Responsibilities**:
  - translate client rule requests into vendor DVR API operations
  - support both series and one-time rules with advanced rule options
  - project rule status back into guide, episode, and upcoming views

## 9. Recording Playback Service
- **Purpose**: Bridge recorded items into the existing playback system.
- **Responsibilities**:
  - resolve recorded-file playback targets from the catalog resolver
  - reuse the existing playback session boundary instead of creating a separate player implementation
  - keep live and recorded playback transitions explicit so the client can show accurate state

## 10. Recording Maintenance Service
- **Purpose**: Handle destructive recording operations safely.
- **Responsibilities**:
  - validate deletion eligibility and the resolved command target before issuing delete operations
  - perform record-engine sync steps when required so the library view stays current
  - ensure client-facing delete outcomes are idempotent and easy to retry
- **Security Notes**:
  - treat vendor or storage-provided command URLs as untrusted until validated against the discovered storage context
  - avoid following arbitrary hosts or schemes outside the approved local DVR environment

## 11. Playback Session Service Updates
- **Purpose**: Extend the existing playback path with explicit stop behavior.
- **Responsibilities**:
  - stop the current live session without terminating the desktop application
  - release playback state cleanly before the user switches to recordings or other views
  - reuse the same stop path for failures, user-initiated stop, and future session resets where appropriate

## DVR-Oriented Orchestration Pattern
- Client opens the DVR tab.
- Client requests DVR home state from the backend.
- DVR Library Service computes readiness and loads a merged recording catalog with local-first ordering.
- Recording Rule Service loads existing rules and upcoming schedule state.
- User chooses a recording to play, delete, or inspect.
- Recording Playback Service resolves the playback target and hands it to the existing playback session flow.
- If the user stops Live TV, Playback Session Service ends the live session while the client remains active.