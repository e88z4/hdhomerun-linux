# Code Generation Plan - Unit 9 DVR Integration Hardening and Verification

## Execution Checklist
- [x] Review Unit 9 functional design, NFR requirements, and NFR design artifacts
- [x] Review integrated DVR implementation from Units 6 through 8
- [x] Identify likely hardening targets across backend, client, tests, and verification artifacts
- [x] Step 1. Tighten backend DVR edge handling for stale recording actions, rule mutation outcomes, and refresh-oriented warning behavior where gaps remain
- [x] Step 2. Tighten client DVR recovery behavior for stale selection, banner-backed recovery guidance, and playback-mode transition cleanup where gaps remain
- [x] Step 3. Add or expand deterministic tests for stale delete handling, rule-context trust behavior, and playback transition correctness
- [x] Step 4. Add or refine local smoke-oriented coverage and operator-facing verification artifacts for the integrated DVR increment
- [x] Step 5. Build and run relevant backend and client checks for the integrated DVR scope and fix issues directly related to Unit 9
- [x] Step 6. Create Unit 9 code summary documentation in `aidlc-docs/construction/unit-9-dvr-integration-hardening-verification/code/summary.md`
- [x] Step 7. Validate that Unit 9 changes preserve backend-owned DVR authority and the shared playback architecture established in earlier units

## Brownfield File Targets
- `backend/src/dvr.rs`
- `backend/src/http/routes.rs`
- `backend/src/models.rs`
- `backend/tests/`
- `client/src/appcontroller.h`
- `client/src/appcontroller.cpp`
- `client/src/dvrworkspacehelpers.*`
- `client/tests/`
- `aidlc-docs/construction/unit-9-dvr-integration-hardening-verification/code/summary.md`
- existing DVR verification documentation under `aidlc-docs/construction/`

## Unit and Story Mapping
- **Unit**: DVR Integration Hardening and Verification
- **Folder Name**: `unit-9-dvr-integration-hardening-verification`
- **Stories Hardened by This Unit**:
  - [x] US-11 Browse recorded content with useful defaults
  - [x] US-12 Play a recorded show inside the app
  - [x] US-13 Delete a recording safely
  - [x] US-16 Manage rule detail with flexible options
  - [x] US-17 Recognize what will record and what already exists
  - [x] US-18 Stop Live TV without quitting the app

## Planned Technical Shape
- Preserve the backend as the only authority for destructive and rule-mutation actions.
- Preserve the shared playback model while tightening transition and recovery behavior.
- Prefer deterministic helper and contract-edge tests over broad UI abstraction changes.
- Keep real-device verification resumable and explicit rather than implicit in code comments.