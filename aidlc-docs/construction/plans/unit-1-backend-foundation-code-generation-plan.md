# Code Generation Plan - Unit 1 Backend Foundation and Local API

## Unit Context
- **Unit**: Backend Foundation and Local API
- **Stories Covered Primarily**: US-1, supporting contract work for US-2, US-5, US-6, and US-8
- **Target Code Location**: `backend/`

## Execution Steps
- [x] Step 1: Create the Rust workspace foundation for `backend/`
- [x] Step 2: Add Cargo manifest, lock file, and core dependencies for HTTP, serialization, logging, validation, and testing
- [x] Step 3: Create backend entry point and runtime bootstrap modules
- [x] Step 4: Implement health and bootstrap state endpoint contract skeletons
- [x] Step 5: Implement canonical remembered-state model and XDG state-file handling
- [x] Step 6: Implement structured API error model and common response types
- [x] Step 7: Add startup readiness flow and app-managed spawn assumptions into the backend runtime surface
- [x] Step 8: Add example-based tests for startup and restore-state behavior
- [x] Step 9: Add `proptest`-based tests for restore-state round-trip and error invariants
- [x] Step 10: Add backend README or notes explaining local run and test commands

## Story Traceability
- **US-1 Reopen the last viewing context**: Steps 4, 5, 8, 9
- **US-2 Handle first launch cleanly**: Steps 4, 5, 6, 8
- **US-5 and US-6 Support Contract Stability**: Steps 4 and 6 provide stable shapes for later playback work
- **US-8 Failure Recovery Foundation**: Step 6 provides structured retry-oriented error shapes

## Files Expected to Be Created
- `backend/Cargo.toml`
- `backend/Cargo.lock`
- `backend/src/main.rs`
- `backend/src/app.rs`
- `backend/src/http/mod.rs`
- `backend/src/http/routes.rs`
- `backend/src/http/types.rs`
- `backend/src/state/mod.rs`
- `backend/src/state/store.rs`
- `backend/src/error.rs`
- `backend/src/models.rs`
- `backend/tests/bootstrap_contract.rs`
- `backend/tests/property_state.rs`
- `backend/README.md`

## Tech Stack Assumptions
- Rust backend
- Axum-style loopback HTTP/JSON service direction
- Serde for serialization
- Tracing for structured logs
- Proptest for property-based tests

## Completion Goal
- End Unit 1 with a runnable backend skeleton that exposes real health and bootstrap behavior, persists canonical remembered state to the XDG state directory, and provides stable contract and test foundations for later units.