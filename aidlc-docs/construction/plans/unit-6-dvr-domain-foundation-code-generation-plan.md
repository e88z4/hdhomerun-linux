# Code Generation Plan - Unit 6 DVR Domain Foundation

## Execution Checklist
- [x] Read unit design artifacts from functional design, NFR requirements, and NFR design
- [x] Read unit story map and dependencies for Unit 6
- [x] Review workspace root, project structure, and current backend code seams
- [x] Define exact brownfield file targets for Unit 6 generation
- [x] Step 1. Add a new backend DVR domain module in `backend/src/dvr.rs` for readiness evaluation, rule validation, upstream adapter traits, and schedule projection logic
- [x] Step 2. Extend `backend/src/models.rs` with DVR domain request and response types for readiness, rule lifecycle, and scheduled-state projection
- [x] Step 3. Extend `backend/src/app.rs` and `backend/src/lib.rs` to wire shared DVR-domain dependencies into `AppState` and test fixtures
- [x] Step 4. Extend `backend/src/http/routes.rs` with Unit 6 DVR endpoints for readiness, rule creation, rule listing, and upcoming-state projection
- [x] Step 5. Extend `backend/src/error.rs` only as needed to preserve structured DVR-domain validation and uncertain-state errors
- [x] Step 6. Add or update backend unit tests covering readiness invariants, supported versus unsupported rule validation, stale-airing rejection, and scheduled-state projection precedence
- [x] Step 7. Run targeted backend tests for the affected modules and fix any issues directly related to Unit 6
- [x] Step 8. Create Unit 6 code summary documentation in `aidlc-docs/construction/unit-6-dvr-domain-foundation/code/summary.md`
- [x] Step 9. Validate that modified files are in-place brownfield changes with no duplicate file creation

## Unit Context
- **Unit**: DVR Readiness, Rule Lifecycle, and Schedule Projection
- **Folder Name**: `unit-6-dvr-domain-foundation`
- **Stories Implemented by This Unit**:
  - [x] US-9 Understand whether DVR is available
  - [x] US-14 Create a series recording rule
  - [x] US-15 Create a one-time recording rule
  - [x] US-17 Recognize what will record and what already exists
- **Supporting Stories**:
  - [ ] US-10 Switch between Live TV and DVR workspaces
  - [ ] US-16 Manage rule detail with flexible options
- **Dependencies**:
  - Unit 1 backend foundation and local API
  - Unit 2 device discovery and lineup context
  - existing guide integration for episode and schedule context where applicable
- **Service Boundaries**:
  - backend owns all DVR vendor integration and validation
  - client receives only structured loopback responses
  - recorded-library execution remains out of scope for Unit 6 and belongs to Unit 7

## Exact Brownfield Targets
- `backend/src/dvr.rs` (new)
- `backend/src/models.rs` (modify)
- `backend/src/app.rs` (modify)
- `backend/src/lib.rs` (modify)
- `backend/src/http/routes.rs` (modify)
- `backend/src/error.rs` (modify only if needed for structured outcomes)
- `aidlc-docs/construction/unit-6-dvr-domain-foundation/code/summary.md` (new)

## Generation Notes
- Focus on backend-owned DVR readiness, rule validation, and schedule projection only.
- Do not implement recorded-library playback or deletion in this unit.
- Reuse the existing Rust backend stack, structured error style, and static-fixture testing approach.
- Prefer provider traits and static fixtures so Unit 7 and later code can build on the same seams.

## Story Traceability
- **US-9** maps to Step 1, Step 2, Step 3, Step 4, and Step 6.
- **US-14** maps to Step 1, Step 2, Step 4, and Step 6.
- **US-15** maps to Step 1, Step 2, Step 4, and Step 6.
- **US-17** maps to Step 1, Step 2, Step 4, and Step 6.

## Plan Summary
- **Total Steps**: 9
- **Approach**: add one new backend domain module, extend shared models and state wiring, add new DVR HTTP endpoints, verify the domain behavior with targeted tests, and document the generated code.
- **Single Source of Truth**: This plan governs Unit 6 code generation.