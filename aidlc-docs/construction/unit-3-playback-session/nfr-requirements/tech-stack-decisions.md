# Tech Stack Decisions - Unit 3 Playback Session Orchestration and Player Adapter

## Playback Adapter Strategy
- **Choice**: Backend-owned `mpv` process controlled through its JSON IPC socket.
- **Rationale**:
  - Fits the approved persistent-session model without forcing an early `libmpv` embedding decision.
  - Keeps the backend as the canonical playback orchestrator while preserving a clean swap point for later adapter evolution.
  - Supports channel replacement inside one long-lived player process.

## Orchestration Language and Runtime
- **Choice**: Keep Unit 3 inside the existing Rust backend on Tokio and Axum.
- **Rationale**:
  - Avoids introducing a second orchestration runtime.
  - Reuses existing structured error handling, state persistence, and HTTP contract infrastructure.

## Source Resolution Strategy
- **Choice**: Resolve playback URLs through the existing Unit 2 lineup pipeline.
- **Rationale**:
  - Preserves one source-of-truth for device and channel validation.
  - Avoids a second discovery or playback-source resolver in the playback path.

## Retry Strategy
- **Choice**: One bounded automatic retry after a retryable adapter startup or stream-load failure.
- **Rationale**:
  - Improves first-use resilience without creating uncontrolled background recovery loops.
  - Matches the approved retry posture and keeps failures predictable for the client.

## Process Lifecycle Strategy
- **Choice**: Keep `mpv` alive in a short-lived reusable ready state after a normal stop when practical.
- **Rationale**:
  - Improves subsequent channel-start or switch responsiveness.
  - Aligns with the approved persistent-session behavior.

## Test Strategy
- **Choice**: HTTP contract tests plus orchestration tests backed by a fake player adapter.
- **Rationale**:
  - Exercises the public API shape and the session-state rules without requiring `mpv` in CI.
  - Keeps the adapter boundary honest and replaceable.

## Stack Constraints
- Playback control remains loopback-only through the backend API.
- Default logs stay high-level and structured; raw IPC detail is debug-only.
- The current implementation performs an explicit preflight check for an `mpv` executable before startup and supports overriding the executable path with `HDHR_BACKEND_MPV_BIN`.
- Packaging work still needs to decide whether `mpv` is bundled directly or declared as an external dependency.