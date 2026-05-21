# Unit of Work Dependency

## Dependency Matrix

| Unit | Depends On | Dependency Type | Reason |
|---|---|---|---|
| Unit 1: Backend Foundation and Local API | None | Foundational | Establishes the service contract and runtime base |
| Unit 2: HDHomeRun Discovery and Device Integration | Unit 1 | Hard dependency | Needs backend models, logging, validation, and API hosting base |
| Unit 3: Playback Session Orchestration and Player Adapter | Unit 1, Unit 2 | Hard dependency | Needs backend runtime plus device-selected playback sources and tuner context |
| Unit 4: Qt/QML Client Shell and Live-TV User Journey | Unit 1 | Early hard dependency | Needs the client-backend API contract and readiness behavior |
| Unit 4: Qt/QML Client Shell and Live-TV User Journey | Unit 2, Unit 3 | Feature dependency | Needs real device data and playback session behavior for full UX |
| Unit 5: Packaging and Distribution Outputs | Unit 1, Unit 4 | Hard dependency | Needs runnable backend and client entry points |
| Unit 5: Packaging and Distribution Outputs | Unit 2, Unit 3 | Completion dependency | Needs integrated runtime behavior for realistic packaging validation |

## Recommended Sequencing

### Phase 1: Foundation
- Complete Unit 1 first.

### Phase 2: Core Runtime Buildout
- Start Unit 2 after Unit 1 is stable.
- Start Unit 4 shell work after Unit 1 defines the API contract.

### Phase 3: Playback Integration
- Start Unit 3 once Unit 2 can provide device and playback source information.
- Continue Unit 4 in parallel as real playback and diagnostics behavior become available.

### Phase 4: Distribution
- Start Unit 5 after a basic runnable backend-plus-client path exists.
- Finish Unit 5 after Units 2 through 4 are integrated.

## Risk Notes
- **Highest Risk Dependency**: Unit 3 depends on both vendor-device behavior and mpv integration; it is the most integration-heavy unit.
- **Most Important Contract**: The Unit 1 loopback API contract must be stable early so Unit 4 can proceed without thrashing.
- **Late-stage Risk**: Unit 5 can expose runtime assumptions that were easy to ignore in dev mode, so packaging should start before the final milestone.