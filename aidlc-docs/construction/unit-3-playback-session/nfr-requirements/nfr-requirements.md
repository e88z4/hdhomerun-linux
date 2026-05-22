# NFR Requirements - Unit 3 Playback Session Orchestration and Player Adapter

## Performance Requirements
- Initial playback start under normal local conditions should usually complete in about 2 to 4 seconds.
- Channel switches should feel faster than first start whenever the persistent playback session and adapter remain healthy.
- Normal stop requests should return promptly and leave the adapter in a reusable ready state when short-lived reuse is practical.

## Availability and Reliability Requirements
- Unit 3 must reuse one backend-owned playback session across normal channel changes rather than recreating the full player process for every switch.
- Unit 3 must detect a missing `mpv` runtime dependency explicitly and surface a stable client-safe failure before normal playback startup proceeds.
- Unit 3 may consume one bounded automatic retry for clearly retryable startup or stream-load failures.
- After the bounded automatic retry is exhausted, the backend must surface a structured failed session state rather than continuing to retry in the background.
- Adapter crashes or unrecoverable errors must move the session into a failed state and defer adapter rebuild until the next explicit retry path or the one bounded automatic retry path.
- Current playback state must remain queryable even after failure so the client can render a stable recovery UX.

## Security Requirements
- Playback orchestration must remain loopback-only through the backend API; the player adapter must not expose a new remote control surface.
- Client-visible responses must stay sanitized and must not leak raw `mpv` IPC payloads, process arguments, or unsafe command output.
- Playback commands must validate device and channel prerequisites through Unit 2 data before issuing player-adapter commands.
- Logging must avoid sensitive local filesystem or process details unless debug mode is explicitly enabled.

## Logging and Observability Requirements
- Default logging should capture high-level structured lifecycle events for start, switch, stop, retry, and failure transitions.
- Detailed `mpv` command and IPC-level troubleshooting detail should be reserved for explicit debug mode.
- Retry consumption and adapter-failure classification must be visible in logs without requiring packet captures or direct player inspection.

## Maintainability Requirements
- Playback orchestration must sit behind a stable adapter boundary so the project can replace process-controlled `mpv` with a future `libmpv` path without rewriting the HTTP contract.
- Unit 3 must continue to rely on Unit 2 for device and playback-source resolution rather than introducing a second authority.
- Current-state projections should stay normalized and client-safe, with adapter-specific details isolated from the public API surface.

## Testability Requirements
- Unit 3 must include HTTP contract tests for playback current, start, stop, and switch flows.
- Unit 3 must include orchestration tests against a fake player adapter to cover retry handling, session reuse, and failure projection.
- Example-based tests remain mandatory for lifecycle regressions even if later units add broader integration coverage.

## Usability Requirements
- Current-state responses must include enough normalized information for the Qt/QML client to render the current device, channel, playback state, and retry-oriented failure messaging without parsing adapter internals.
- Successful stop should preserve enough last-channel context for restore-aware UX while still exposing a stopped session state.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable and required for playback command inputs and source resolution preconditions.
- **SECURITY-08 Application-Level Access Control**: Applicable in local-service form; playback control remains loopback-only.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable and required for sanitized player-adapter failures and minimal process exposure.
- **SECURITY-11 Secure Design Principles**: Applicable and required through explicit trust boundaries between backend API, Unit 2 source resolution, and the player adapter.

## PBT Compliance Summary
- **PBT-03 Invariant Properties**: Applicable to bounded retry count, normalized session-state invariants, and sanitized failure envelopes.
- **PBT-08 Shrinking and Reproducibility**: Applicable for any future playback-state properties added to the backend.
- **PBT-09 Framework Selection**: Continues to use `proptest` when new pure playback-state properties are introduced.