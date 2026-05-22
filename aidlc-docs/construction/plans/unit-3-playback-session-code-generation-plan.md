# Code Generation Plan - Unit 3 Playback Session Orchestration and Player Adapter

## Execution Checklist
- [x] Review Unit 3 functional, NFR, and infrastructure artifacts against the current backend code
- [x] Choose the first backend slice for playback state, playback commands, and adapter orchestration
- [x] Define the test seam between the orchestration layer and the player adapter
- [x] Decide how playback commands resolve devices and channels through Unit 2
- [x] Implement the minimum tested slice for current, start, stop, and switch playback APIs

## Current Backend Baseline
- `GET /api/playback/current` existed only as a provisional placeholder.
- Unit 2 already provided device discovery, selected-device reconciliation, lineup data, and tuner diagnostics.
- The backend had no playback orchestration state or player adapter boundary.

## Approved Direction
- Add a dedicated playback orchestration module inside the Rust backend.
- Promote `/api/playback/current` to a real endpoint and add `POST /api/playback/start`, `POST /api/playback/stop`, and `POST /api/playback/switch`.
- Resolve playback targets through Unit 2 device and lineup data rather than creating a separate source resolver.
- Introduce a stable player-adapter interface with two implementations:
  - native Linux `mpv` IPC adapter for runtime behavior
  - fake adapter for deterministic test coverage
- Cover the slice with HTTP contract tests plus orchestration behavior tests for retry and session reuse.

## Implemented Outcome
- Playback session and adapter state models were added to the backend contract.
- A new `backend/src/playback.rs` module now owns session orchestration and adapter integration.
- Playback contract tests validate idle state, start, switch, stop, and bounded retry behavior.
- The playback adapter now performs an explicit `mpv` dependency preflight so the backend can warn early and fail playback start with a stable structured error when the player binary is unavailable.