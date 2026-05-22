# Logical Components - Unit 3 Playback Session Orchestration and Player Adapter

## 1. Playback Command Boundary
- **Purpose**: Exposes current, start, stop, and switch playback endpoints over the loopback API.
- **NFR Role**:
  - stable client-facing contract
  - request validation
  - structured error projection

## 2. Playback Target Resolver
- **Purpose**: Resolves the selected device and playable channel through Unit 2 discovery and lineup data.
- **NFR Role**:
  - prerequisite validation
  - source-of-truth preservation
  - stale-lineup fallback handling

## 3. Session Orchestrator
- **Purpose**: Owns logical playback session state, retry boundaries, and current-state projection.
- **NFR Role**:
  - persistent-session reuse
  - bounded retry
  - stable failure state

## 4. Player Adapter Boundary
- **Purpose**: Hides player-process control behind a replaceable internal interface.
- **NFR Role**:
  - maintainable integration seam
  - future `libmpv` migration path
  - testability with a fake adapter

## 5. Native `mpv` IPC Adapter
- **Purpose**: Spawns `mpv`, manages the IPC socket, and issues player commands.
- **NFR Role**:
  - process reuse across channel changes
  - command confirmation before projecting playback state
  - localized process-failure handling

## 6. Remembered Context Persistence Hook
- **Purpose**: Updates backend-owned remembered playback context after successful playback changes.
- **NFR Role**:
  - restore-aware usability
  - state continuity across launches
  - simple backend-owned persistence

## 7. Playback Contract Test Layer
- **Purpose**: Validates public playback routes and retry behavior without a real player process.
- **NFR Role**:
  - regression coverage for lifecycle flows
  - deterministic failure testing
  - support for future refactoring of the adapter layer