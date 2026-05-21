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