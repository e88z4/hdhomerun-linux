# Business Logic Model

## Scope
Unit 9 hardens and verifies the integrated DVR increment after backend contracts, recorded-library behavior, and the first client workspace are all in place. The unit focuses on edge handling, state convergence, and verification coverage rather than introducing a new feature slice.

## Hardening Focus
- stale or missing recording actions must return clear recovery behavior
- rule-creation context must stay explicit about whether it is trusted, inferred, or unavailable
- playback mode transitions must remain coherent across live, recorded, and stopped states
- degraded DVR environments must surface understandable guidance across the client and operator-facing verification artifacts

## State Propagation Model

### Core Integrated Surfaces
1. DVR readiness
2. recordings catalog
3. upcoming schedule and projection
4. rule list
5. shared playback session state

Unit 9 treats these as one integrated state system. Hardening work should verify that a mutation in one surface leads to predictable refresh behavior in the others.

## Mutation Convergence Model

### Delete
1. User requests delete.
2. Backend validates the current recording snapshot.
3. Client receives confirmed or missing-recording outcome.
4. Client refreshes recordings and related DVR context.
5. Workspace resolves to a clear next state rather than retaining stale selection assumptions.

### Delete And Re-record
1. User requests delete and recovery.
2. Delete path completes first.
3. Recovery rule is attempted only if rule context remains trustworthy enough.
4. Client and backend refresh schedule or rules state explicitly.
5. If context is insufficient, the user is told exactly why rerecord recovery stopped.

### Recorded Playback
1. User starts recorded playback from the DVR workspace.
2. Shared playback session transitions to recorded mode.
3. DVR workspace context remains active.
4. Stop or subsequent live playback must preserve a coherent session outcome.

## Stale-State Recovery Model
- Missing recording outcomes lead to refresh guidance, not silent failure.
- Invalid airing outcomes lead to explicit rule-creation rejection messaging.
- Degraded readiness or schedule state keeps the DVR workspace usable where possible while maintaining visible warning context.

## Verification Model

### Local Verification
- focused unit and presentation-level tests for deterministic state helpers
- local build and smoke checks for client shell startup and backend contract continuity

### Real-Device Verification
- readiness and schedule checks against a DVR-capable device
- recorded playback and deletion checks using a safely staged recording
- cleanup confirmation for temporary rule or recording side effects

## Boundary Rule
Unit 9 may tighten contracts, refresh behavior, and test coverage across backend and client surfaces, but it must not undermine the backend-owned authority model established in Units 6 and 7 or the DVR-first client workspace established in Unit 8.