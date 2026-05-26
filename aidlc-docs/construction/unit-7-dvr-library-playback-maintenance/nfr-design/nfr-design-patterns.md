# NFR Design Patterns - Unit 7 DVR Library Playback and Maintenance

## 1. Local-First Deterministic Merge Pattern
- **Pattern**: Build the recording catalog from an explicitly ordered source list and merge duplicates deterministically.
- **Why**: The UI needs a stable, comprehensible library view while the backend preserves local-first behavior.
- **Behavior**:
  - ordered source resolution with local storage first
  - deterministic preferred-source selection for merged items
  - retained alternate-source metadata for diagnostics and refresh comparison

## 2. Degraded Read, Strict Action Pattern
- **Pattern**: Allow degraded library reads but require current validated targets for playback and deletion actions.
- **Why**: Read availability should remain resilient, but actions must not execute against stale or ambiguous targets.
- **Behavior**:
  - partial library results allowed with warnings
  - playback and delete actions re-resolve current targets before execution
  - missing or stale targets produce refresh-required outcomes

## 3. Snapshot Validation Pattern
- **Pattern**: Bind playback and deletion to a current recording-catalog snapshot rather than to previously returned raw source details.
- **Why**: Prevents silent drift onto a different source or stale action target after the library changes.
- **Behavior**:
  - action resolution always uses a fresh backend snapshot
  - no direct trust in prior client-held action metadata
  - structured missing-recording or validation-failed outcomes on mismatch

## 4. Shared Playback Controller Reuse Pattern
- **Pattern**: Reuse the existing playback session controller for recorded playback and explicit stop.
- **Why**: Keeps one coherent playback-state model across Live TV and DVR behavior.
- **Behavior**:
  - explicit mode transitions for recorded playback and live stop
  - no parallel playback runtime model
  - existing adapter lifecycle reused for stop and session-state reporting

## 5. Single-Attempt Destructive Action Pattern
- **Pattern**: Execute delete mutations at most once per client request and require explicit success or failure.
- **Why**: Repeating destructive actions would be riskier than surfacing a clear failure.
- **Behavior**:
  - no automatic retry for delete commands
  - strict validation before the delete call
  - sync-aware refresh after successful deletion

## 6. Prompt Stop Release Pattern
- **Pattern**: Model explicit Live TV stop as a hard stop that releases playback resources promptly while preserving remembered context.
- **Why**: Matches the requested user behavior without conflating stop with app shutdown or soft pause.
- **Behavior**:
  - terminate active live stream promptly
  - return stable stopped state even when already inactive
  - keep remembered device or channel selection for later restart

## 7. Protected Upstream Action Material Pattern
- **Pattern**: Keep delete URLs and other mutation-capable upstream details inside backend-owned models only.
- **Why**: Prevents the client from becoming a mutation authority and limits accidental leakage.
- **Behavior**:
  - raw action URLs remain backend-internal
  - logs use sanitized identifiers and outcome categories
  - loopback API contracts expose logical identifiers and structured outcomes only

## 8. Deterministic Merge and Validation Test Pattern
- **Pattern**: Keep catalog merge and action validation logic deterministic enough for focused unit and property-based testing.
- **Why**: Unit 7 risk is concentrated in duplicate handling, stale-action rejection, and stop semantics.
- **Behavior**:
  - merge invariants testable across source permutations
  - missing-recording and validation outcomes testable from snapshot changes
  - stop behavior testable without relying on real network or player state