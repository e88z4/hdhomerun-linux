# NFR Design Patterns - Unit 2 HDHomeRun Discovery and Device Integration

## 1. Fixed Lightweight Refresh Pattern
- **Pattern**: Run discovery at startup, then continue with a fixed lightweight background refresh interval.
- **Why**: Matches the approved freshness target without drifting into aggressive polling complexity.
- **Behavior**:
  - startup discovery seeds the initial device list
  - background refresh runs at a predictable bounded interval
  - user-triggered foreground refresh remains possible without changing the steady-state cadence

## 2. Stale-Until-Replaced Lineup Pattern
- **Pattern**: Keep the last successful lineup in memory for the selected device and mark it stale if a refresh fails.
- **Why**: Preserves a usable client experience during transient LAN or device failures without inventing fresh data.
- **Behavior**:
  - successful lineup load replaces the in-memory lineup snapshot
  - failed refresh does not immediately erase the last successful lineup
  - stale state is explicit in backend-owned metadata

## 3. Bounded Device Call Pattern
- **Pattern**: Use bounded timeouts for LAN discovery, lineup retrieval, and tuner-status calls with no automatic retry loops outside the scheduled refresh cycle.
- **Why**: Prevents hidden retry storms and keeps transient device communication issues from monopolizing the backend.
- **Behavior**:
  - each vendor-facing call has an explicit timeout boundary
  - failures surface as structured results
  - the next scheduled refresh or explicit user action is the normal retry path

## 4. Adapter and Normalization Boundary Pattern
- **Pattern**: Separate vendor-facing device calls from normalized contract shaping using distinct discovery and lineup adapters plus a normalization layer.
- **Why**: Keeps vendor specifics isolated and preserves one client-facing data authority.
- **Behavior**:
  - discovery adapter owns `libhdhomerun` interactions
  - lineup adapter owns `lineup.json` retrieval
  - normalization layer converts vendor data into backend-owned entities
  - contract service composes normalized data into API responses

## 5. Per-Tuner Isolation Pattern
- **Pattern**: Model tuner diagnostics as per-tuner results with an overall synthesized diagnostics summary.
- **Why**: Preserves useful diagnostics even when one tuner fails to respond.
- **Behavior**:
  - each tuner lookup succeeds or fails independently
  - failed tuner entries are marked unavailable or degraded
  - an overall summary communicates whether diagnostics are complete, partial, or unavailable

## 6. Structured Device Observability Pattern
- **Pattern**: Emit high-level structured device-integration events at info level and reserve vendor-call detail for debug logging.
- **Why**: Supports diagnosability without overwhelming normal operation logs or exposing noisy low-level detail.
- **Behavior**:
  - discovery refresh, lineup load, stale-device mismatch, and partial diagnostics states are logged consistently
  - deeper call-path details appear only in explicit debug mode

## 7. Hardware-Light Testability Pattern
- **Pattern**: Keep normalization and contract-shaping logic pure enough to test with fixtures and property-based generators rather than depending on live hardware for every test.
- **Why**: Matches the approved Unit 2 testability goals and keeps CI-friendly coverage practical.
- **Behavior**:
  - adapters can be replaced with fixtures or fakes
  - lineup and discovery normalization paths are directly testable
  - per-tuner partial-failure cases are reproducible without a physical device