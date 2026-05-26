# Code Generation Plan - Unit 7 DVR Library Playback and Maintenance

## Execution Checklist
- [x] Review Unit 7 functional design, NFR requirements, and NFR design artifacts
- [x] Review existing backend seams for DVR, playback, and route handling
- [x] Identify brownfield file targets for recorded library, playback, deletion, and stop behavior
- [x] Step 1. Extend `backend/src/dvr.rs` with recorded-files source fetching, deterministic catalog merge, action-target validation, and delete execution support
- [x] Step 2. Extend `backend/src/models.rs` with recorded-library, recording-action, and playback-mode contract types needed by Unit 7
- [x] Step 3. Extend `backend/src/playback.rs` so the shared playback service can represent recorded playback mode and explicit Live TV stop outcomes without a second controller
- [x] Step 4. Extend `backend/src/app.rs` only as needed to expose any new shared helpers or test fixtures for Unit 7
- [x] Step 5. Extend `backend/src/http/routes.rs` and `backend/src/http/types.rs` with recorded-library, recorded-playback, delete, and explicit stop endpoint behavior
- [x] Step 6. Add or update backend tests covering duplicate merge ordering, missing-recording outcomes, delete validation, recorded playback state transitions, and explicit Live TV stop behavior
- [x] Step 7. Run targeted backend tests for the affected modules and fix any Unit 7 issues directly related to this scope
- [x] Step 8. Create Unit 7 code summary documentation in `aidlc-docs/construction/unit-7-dvr-library-playback-maintenance/code/summary.md`
- [x] Step 9. Validate that all Unit 7 changes are in-place brownfield changes with no duplicate file creation

## Brownfield File Targets
- `backend/src/dvr.rs`
- `backend/src/models.rs`
- `backend/src/playback.rs`
- `backend/src/app.rs`
- `backend/src/http/routes.rs`
- `backend/src/http/types.rs`
- `backend/tests/`
- `aidlc-docs/construction/unit-7-dvr-library-playback-maintenance/code/summary.md`

## Unit and Story Mapping
- **Unit**: Recorded Library, Recorded Playback, Deletion, and Live Stop Control
- **Folder Name**: `unit-7-dvr-library-playback-maintenance`
- **Stories Implemented by This Unit**:
  - [x] US-11 Browse recorded content with useful defaults
  - [x] US-12 Play a recorded show inside the app
  - [x] US-13 Delete a recording safely
  - [x] US-18 Stop Live TV without quitting the app
- **Supporting Stories**:
  - [ ] US-17 Recognize what will record and what already exists

## Planned Technical Shape
- Add backend-owned recorded-library resolution that merges duplicate recordings while preserving alternate-source metadata.
- Add backend-owned recorded playback and deletion actions that re-resolve current targets before execution.
- Extend the shared playback service with explicit playback mode reporting so Live TV and recorded playback share one session model.
- Add a loopback stop path that reports stable stopped outcomes while preserving remembered device or channel context.