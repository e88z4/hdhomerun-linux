# Components

## 1. Desktop Client Shell
- **Purpose**: Own the Qt/QML application shell and top-level user navigation.
- **Responsibilities**:
  - application startup
  - backend availability checks
  - route the user between discovery, channel browsing, playback, and diagnostics surfaces
- **Interfaces**:
  - consumes the loopback HTTP/JSON API
  - owns UI state such as active view, drawer visibility, and local presentation preferences

## 2. Channel Browser Component
- **Purpose**: Present discovered devices and the active device lineup.
- **Responsibilities**:
  - render discovered devices and selected device state
  - render channel list
  - trigger playback requests when a channel is chosen
- **Interfaces**:
  - reads device and channel data from the backend API
  - sends selection and play requests to the backend API

## 3. Embedded Player Component
- **Purpose**: Host the persistent in-app playback surface.
- **Responsibilities**:
  - render video playback inside the desktop app
  - reflect loading, playing, and failure states
  - survive channel changes without discarding the player context
- **Interfaces**:
  - receives playback session state from the backend
  - binds to the playback engine integration layer

## 4. Diagnostics Panel Component
- **Purpose**: Surface tuner status and signal information without crowding the main playback surface.
- **Responsibilities**:
  - show compact status indicators in the main view
  - expand into a dedicated diagnostics panel or drawer
  - reflect playback or tuner failures with actionable retry UX
- **Interfaces**:
  - consumes tuner and signal information from the backend API

## 5. Backend API Host
- **Purpose**: Expose the local loopback HTTP/JSON interface to the client.
- **Responsibilities**:
  - bind only to the local machine
  - validate all client inputs at the API boundary
  - serialize backend domain objects into client-facing JSON
  - provide structured logging for requests and failures
- **Interfaces**:
  - HTTP endpoints for devices, channels, playback sessions, state, and diagnostics

## 6. Device Integration Component
- **Purpose**: Encapsulate all libhdhomerun interactions.
- **Responsibilities**:
  - discover devices
  - query device and tuner status
  - obtain lineup and channel context
  - isolate vendor-library usage from higher layers
- **Interfaces**:
  - internal backend service API only

## 7. Playback Session Controller
- **Purpose**: Orchestrate persistent live playback state.
- **Responsibilities**:
  - start and switch live playback sessions
  - keep one coherent player session across channel changes
  - translate backend device state into playback actions
  - surface retryable failure states to the API host
- **Interfaces**:
  - internal backend service API only
  - uses the playback engine adapter

## 8. Playback Engine Adapter
- **Purpose**: Wrap mpv or libmpv integration behind a stable internal interface.
- **Responsibilities**:
  - open stream sources
  - switch streams within the same player context
  - report playback state and failures
- **Interfaces**:
  - internal backend service API only

## 9. State Store
- **Purpose**: Persist canonical application state owned by the backend.
- **Responsibilities**:
  - remember last used device
  - remember last watched channel
  - persist backend-owned session context across launches when appropriate
- **Interfaces**:
  - internal backend service API only

## 10. Service Launcher and Supervisor
- **Purpose**: Support bundled service startup paths.
- **Responsibilities**:
  - let the desktop app auto-start the backend by default
  - support managed-service style execution where appropriate
  - verify the backend is available before the client proceeds
- **Interfaces**:
  - local process management boundary between app startup and backend runtime

## DVR Increment Components

## 11. DVR Workspace Component
- **Purpose**: Present the DVR tab as a split workspace with a recording library on one side and details or actions on the other.
- **Responsibilities**:
  - load and render DVR readiness, recordings, and upcoming status in one navigation surface
  - keep list selection, filters, and details-panel state coherent while data refreshes
  - surface actions for playback, deletion, and rule management without mixing vendor logic into the client
- **Interfaces**:
  - consumes DVR endpoints from the loopback HTTP/JSON API
  - delegates playback actions to existing playback session flows

## 12. Recording Rule Editor Component
- **Purpose**: Support the approved hybrid rule-management UX.
- **Responsibilities**:
  - open quick-create actions from guide, episodes, or recordings context
  - provide a fuller dedicated editor for advanced options such as padding and channel constraints
  - render validation errors returned by the backend without duplicating rule logic in QML
- **Interfaces**:
  - consumes DVR rule endpoints from the backend API

## 13. DVR Integration Component
- **Purpose**: Encapsulate all HDHomeRun DVR API interactions that are not part of live playback discovery.
- **Responsibilities**:
  - call vendor-side DVR endpoints for recording rules, episodes, upcoming state, and related metadata
  - query discovered storage engines and record-engine endpoints for recorded files and sync operations
  - normalize vendor and local DVR payloads into backend domain models
- **Interfaces**:
  - internal backend service API only

## 14. Recording Catalog Resolver
- **Purpose**: Resolve the authoritative recorded-library view from multiple storage sources.
- **Responsibilities**:
  - prioritize local storage sources ahead of non-local sources
  - merge duplicate or overlapping recording representations into one client-facing catalog model
  - retain source metadata for diagnostics, playback routing, and deletion commands
- **Interfaces**:
  - internal backend service API only

## 15. Playback Stop Coordinator
- **Purpose**: Provide a clear backend-owned stop boundary for live sessions.
- **Responsibilities**:
  - terminate live playback without shutting down the app shell
  - release playback-session state and tuner-facing session context cleanly
  - preserve the distinction between stopping playback and exiting the app
- **Interfaces**:
  - internal backend service API only