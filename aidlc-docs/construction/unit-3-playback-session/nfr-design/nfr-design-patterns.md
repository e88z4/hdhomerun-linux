# NFR Design Patterns - Unit 3 Playback Session Orchestration and Player Adapter

## 1. Persistent Session Reuse Pattern
- **Pattern**: Keep one backend-owned playback session alive across normal channel changes.
- **Why**: Meets the approved responsiveness target and avoids unnecessary process churn.
- **Behavior**:
  - one logical playback session at a time
  - switch commands replace the current stream inside the existing adapter session
  - normal stop keeps the adapter ready for short-lived reuse when practical

## 2. Bounded Retry and Deferred Rebuild Pattern
- **Pattern**: Allow one automatic retry for retryable startup failures and defer full rebuild otherwise.
- **Why**: Improves resilience without creating uncontrolled restart loops.
- **Behavior**:
  - one automatic retry per start attempt
  - structured failed state after retry exhaustion
  - adapter rebuild only on the bounded retry path or a later explicit retry request

## 3. Normalized Playback Projection Pattern
- **Pattern**: Expose playback state through a normalized backend-owned projection.
- **Why**: Lets the client render playback UX without parsing player-specific internals.
- **Behavior**:
  - stable session-state shape
  - stable adapter-state shape
  - sanitized failure metadata and warnings

## 4. Unit 2 Source Resolution Gate Pattern
- **Pattern**: Validate every playback command against Unit 2 device and lineup data before touching the adapter.
- **Why**: Preserves one authority for device availability and playback URL resolution.
- **Behavior**:
  - selected device validation
  - lineup lookup or cached-lineup fallback
  - rejection of restricted or unavailable channels before adapter calls

## 5. High-Level Logging with Debug Escalation Pattern
- **Pattern**: Default to lifecycle-level structured logs and reserve raw adapter detail for debug mode.
- **Why**: Meets observability needs without exposing low-level process data in normal runs.
- **Behavior**:
  - start, switch, stop, retry, and failure events at info level
  - IPC or command detail only when debug logging is enabled

## 6. Fakeable Adapter Boundary Pattern
- **Pattern**: Isolate the real `mpv` IPC adapter behind a trait-compatible boundary.
- **Why**: Supports deterministic tests and future adapter replacement.
- **Behavior**:
  - native adapter for Linux runtime behavior
  - fake adapter for contract and orchestration tests
  - orchestration logic independent from process-spawn details