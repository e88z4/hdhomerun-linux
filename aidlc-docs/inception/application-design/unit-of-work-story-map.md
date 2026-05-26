# Unit of Work Story Map

## Story to Unit Mapping

| Story | Primary Unit | Supporting Units | Rationale |
|---|---|---|---|
| US-1 Reopen the last viewing context | Unit 1 | Unit 2, Unit 3, Unit 4 | Restore behavior depends on canonical backend state, device availability, playback session handling, and UI launch flow |
| US-2 Handle first launch cleanly | Unit 4 | Unit 1, Unit 2 | First-launch UX is client-led but depends on backend startup and discovery behavior |
| US-3 See discovered devices | Unit 2 | Unit 4 | Device discovery is backend-driven and presented by the client |
| US-4 Browse channel lineup | Unit 2 | Unit 4 | Lineup retrieval is backend-driven and rendered in the client |
| US-5 Start live playback immediately from channel selection | Unit 3 | Unit 2, Unit 4 | Playback session orchestration is the core behavior, using resolved device sources and client interaction |
| US-6 Switch channels inside one persistent player session | Unit 3 | Unit 2, Unit 4 | Persistent playback session behavior belongs to playback orchestration, not the UI alone |
| US-7 View tuner status and signal info | Unit 2 | Unit 3, Unit 4 | Device and tuner metrics originate in backend integration and are surfaced in the diagnostics UX |
| US-8 Recover from tuner or stream startup failures | Unit 3 | Unit 2, Unit 4 | Retryable failure states originate in backend playback control and are surfaced in the client |

## Coverage Check
- **All stories assigned**: Yes
- **Stories with backend foundation implications**: US-1, US-2, US-5, US-6, US-8
- **Stories with direct device integration implications**: US-1, US-2, US-3, US-4, US-7, US-8
- **Stories with direct client UX implications**: All stories
- **Packaging linkage**: Unit 5 does not own user stories directly, but it is required to satisfy the deliverable acceptance criteria around Linux distribution outputs.

## Delivery Interpretation
- Units 1 through 4 implement the user-visible product.
- Unit 5 turns that runnable product into the promised first deliverable formats.

## DVR Story to Unit Mapping

| Story | Primary Unit | Supporting Units | Rationale |
|---|---|---|---|
| US-9 Understand whether DVR is available | Unit 6 | Unit 8 | DVR readiness is backend-owned and then surfaced in the DVR workspace |
| US-10 Switch between Live TV and DVR workspaces | Unit 8 | Unit 6 | The tabbed workflow is client-led but depends on backend DVR readiness and summary state |
| US-11 Browse recorded content with useful defaults | Unit 7 | Unit 8 | Local-first recording-catalog behavior is backend-led and rendered in the client |
| US-12 Play a recorded show inside the app | Unit 7 | Unit 8 | Recorded playback reuses backend playback orchestration and is launched from the DVR UI |
| US-13 Delete a recording safely | Unit 7 | Unit 8, Unit 9 | Deletion is backend-owned, UI-triggered, and benefits from hardening coverage |
| US-14 Create a series recording rule | Unit 6 | Unit 8 | Series-rule lifecycle belongs to the backend contract and is exercised through client UX |
| US-15 Create a one-time recording rule | Unit 6 | Unit 8 | One-time rule creation is vendor-model-specific backend work surfaced in the client |
| US-16 Manage rule detail with flexible options | Unit 8 | Unit 6 | The richer rule-editor UX is client-led but depends on backend support for advanced rule fields |
| US-17 Recognize what will record and what already exists | Unit 6 | Unit 7, Unit 8, Unit 9 | Scheduled and recorded state requires backend projection, recorded-catalog integration, client presentation, and later hardening |
| US-18 Stop Live TV without quitting the app | Unit 7 | Unit 8 | Live-stop behavior shares the playback-session boundary while the control is presented in the client |

## DVR Coverage Check
- **All DVR stories assigned**: Yes
- **Stories with backend DVR-domain implications**: US-9, US-11, US-12, US-13, US-14, US-15, US-17, US-18
- **Stories with direct client UX implications**: US-9 through US-18
- **Stories with playback-boundary implications**: US-12, US-13, US-18
- **Stories with rule-lifecycle implications**: US-14, US-15, US-16, US-17

## DVR Delivery Interpretation
- Unit 6 establishes the DVR contract and rule domain.
- Unit 7 makes DVR execution real through recordings, playback, deletion, and stop behavior.
- Unit 8 delivers the visible DVR product experience.
- Unit 9 hardens the increment so construction can proceed with clearer verification targets.