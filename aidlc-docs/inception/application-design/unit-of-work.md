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