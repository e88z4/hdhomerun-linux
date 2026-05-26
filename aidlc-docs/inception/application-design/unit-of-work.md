# Unit of Work

## Code Organization Strategy
- **Top-level layout**:
  - `backend/`
  - `client/`
  - `packaging/`
- **Reasoning**: This matches the approved architecture boundary between the reusable local service, the Qt/QML desktop client, and the packaging outputs, while keeping the repo easy to navigate.

## Unit 1: Backend Foundation and Local API
- **Purpose**: Establish the reusable service runtime, loopback HTTP/JSON boundary, structured logging, validation boundary, and canonical state ownership.
- **Primary Responsibilities**:
  - backend process entry point
  - loopback API host and endpoint skeletons
  - service launcher and health checks
  - state store foundation for remembered device and last channel
  - shared backend models and error contracts
- **Target Code Area**: `backend/`
- **Why This Is a Separate Unit**: Everything else depends on a stable backend runtime and API contract.

## Unit 2: HDHomeRun Discovery and Device Integration
- **Purpose**: Integrate `libhdhomerun` for device discovery, lineup retrieval, and tuner or signal visibility.
- **Primary Responsibilities**:
  - discover HDHomeRun devices
  - maintain active device selection
  - load lineup data
  - retrieve tuner status and signal information
  - resolve playback sources from device-side capabilities
- **Target Code Area**: `backend/`
- **Why This Is a Separate Unit**: It isolates vendor-device concerns from runtime and playback orchestration logic.

## Unit 3: Playback Session Orchestration and Player Adapter
- **Purpose**: Implement persistent playback sessions and connect backend orchestration to mpv or libmpv.
- **Primary Responsibilities**:
  - initialize playback engine integration
  - start playback
  - switch channels inside one player session
  - surface retryable playback failures
  - expose playback session state to the API layer
- **Target Code Area**: `backend/`
- **Why This Is a Separate Unit**: Playback reliability and session continuity are their own risk area and need a focused unit.

## Unit 4: Qt/QML Client Shell and Live-TV User Journey
- **Purpose**: Deliver the user-facing Linux desktop experience.
- **Primary Responsibilities**:
  - app startup and backend readiness flow
  - device selection and channel browsing UI
  - embedded player view
  - diagnostics drawer and compact status indicators
  - inline failure states and retry actions
- **Target Code Area**: `client/`
- **Why This Is a Separate Unit**: It owns the viewer experience and should remain insulated from backend implementation details.

## Unit 5: Packaging and Distribution Outputs
- **Purpose**: Produce the first deliverable in AppImage, Flatpak, and Debian package formats.
- **Primary Responsibilities**:
  - packaging metadata and build scripts
  - runtime layout decisions for bundled backend plus client
  - install and launch behavior verification
  - distribution-specific smoke-test guidance
- **Target Code Area**: `packaging/`
- **Why This Is a Separate Unit**: Packaging is a deliverable requirement, but it should mature after the runnable app path exists.

## Decomposition Summary
- **Chosen Shape**: Five-unit split aligned to architectural boundaries.
- **Execution Style**: Hybrid.
- **Critical Path**:
  1. Unit 1
  2. Unit 2
  3. Unit 3
  4. Unit 4
  5. Unit 5
- **Parallelization Opportunity**:
  - Unit 2 and Unit 3 can overlap once Unit 1 is stable enough.
  - Unit 4 can begin its UI shell work after Unit 1 defines the API contract, then deepen as Units 2 and 3 mature.
  - Unit 5 begins late but should not wait until the very end for all packaging decisions.

## DVR Increment Decomposition Summary
- **Chosen Shape**: Four DVR-focused units added after the existing baseline units.
- **Execution Style**: Backend-first with hybrid sequencing.
- **Critical Path**:
  1. Unit 6
  2. Unit 7
  3. Unit 8
  4. Unit 9

## Unit 6: DVR Readiness, Rule Lifecycle, and Schedule Projection
- **Purpose**: Establish the backend-owned DVR domain foundation around readiness detection, recording-rule management, and scheduled-state projection.
- **Primary Responsibilities**:
  - detect DVR capability and readiness from DeviceAuth, storage, and record-engine context
  - integrate vendor recording-rules APIs for series and one-time rule lifecycle
  - expose upcoming and scheduled-state projections for client workflows
  - define DVR endpoint contracts and validation for readiness, rules, and schedule state
- **Target Code Area**: `backend/`
- **Why This Is a Separate Unit**: It creates the stable backend contract that the rest of the DVR increment depends on.

## Unit 7: Recorded Library, Recorded Playback, Deletion, and Live Stop Control
- **Purpose**: Implement recording-catalog resolution and the playback-session behaviors shared by recorded playback, deletion, and explicit Live TV stop.
- **Primary Responsibilities**:
  - discover and merge recorded items across storage sources with local-first ordering
  - route recorded playback through the existing playback session controller
  - validate and execute recording deletion safely with sync-aware refresh behavior
  - add explicit live-session stop behavior that releases playback and tuner resources without app exit
- **Target Code Area**: `backend/`
- **Why This Is a Separate Unit**: It is the highest-risk integration slice because it combines storage, playback, destructive actions, and stop semantics.

## Unit 8: DVR Client Workspace and Rule-Management UX
- **Purpose**: Deliver the approved DVR user experience in the Qt/QML client.
- **Primary Responsibilities**:
  - add the DVR tab and split-workspace layout
  - render DVR readiness, recorded-library, details, and upcoming state
  - add hybrid rule-creation and rule-editing flows
  - expose recorded playback, deletion, and explicit Live TV stop controls in the right interaction points
- **Target Code Area**: `client/`
- **Why This Is a Separate Unit**: The client can now build against a stable backend DVR contract instead of inventing UI-specific behavior.

## Unit 9: DVR Integration Hardening and Verification
- **Purpose**: Finish the increment by hardening contract edges, ensuring story coverage, and preparing the construction-phase verification path.
- **Primary Responsibilities**:
  - tighten backend validation and error behavior for DVR endpoints
  - verify state propagation between guide, recordings, rules, and stop behavior
  - close integration gaps between backend and client workflows
  - prepare test-oriented acceptance coverage for the DVR increment
- **Target Code Area**: `backend/`, `client/`, and `aidlc-docs/construction/`
- **Why This Is a Separate Unit**: It reduces the risk of partially integrated DVR behavior before construction-stage implementation details begin.