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