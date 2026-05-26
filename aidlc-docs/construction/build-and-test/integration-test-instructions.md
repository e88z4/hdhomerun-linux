# Integration Test Instructions

## Purpose
Validate interactions between backend DVR authority, playback orchestration, Qt/QML client behavior, and real-device HDHomeRun DVR scenarios.

## Scenario 1: Backend DVR Contract Integration
- **Description**: verify DVR recording browse, play, stale delete, and recorded-stop behavior through the loopback contract layer
- **Setup**: no real device required; static fixtures are sufficient
- **Run**:
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
cargo test --test dvr_contract --test playback_contract
```
- **Expected Results**:
  - recorded playback enters `recorded` mode
  - stopping recorded playback clears stale recorded context
  - repeated delete requests return the missing-recording outcome with refresh guidance

## Scenario 2: Client And Backend Embedded Playback Integration
- **Description**: verify the client can launch against the local backend build and keep playback shell state coherent
- **Setup**:
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux
cmake -S client -B build/client -G Ninja
cmake --build build/client
```
- **Run**:
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/build/client
ctest --output-on-failure
```
- **Expected Results**:
  - client smoke and helper tests pass
  - DVR helper tests preserve trusted rule-context behavior and grouping behavior

## Scenario 3: Real-Device DVR Workflow Verification
- **Description**: validate the integrated application against a real HDHomeRun device with storage attached
- **Setup**:
```bash
cd /home/felix/src/hdhomerun/hdhomerun-linux/backend
HDHR_BACKEND_PLAYER_MODE=client cargo run
```
- **Test Steps**:
  1. Follow `aidlc-docs/construction/unit-7-dvr-library-playback-maintenance/code/real-device-backend-smoke-test.md`.
  2. When a real recording exists, follow `aidlc-docs/construction/unit-7-dvr-library-playback-maintenance/code/recorded-playback-delete-smoke-test.md`.
  3. Confirm that a repeated delete returns the missing-recording outcome and refresh guidance.
- **Cleanup**:
  - stop the backend process
  - remove any temporary recording rules created only for validation

## Logs And Diagnostics
- **Backend Logs**: terminal output from `cargo run`
- **Client Logs**: terminal output when launching the desktop client from the build directory
- **Contract Failures**: Rust test output from the targeted `cargo test` commands