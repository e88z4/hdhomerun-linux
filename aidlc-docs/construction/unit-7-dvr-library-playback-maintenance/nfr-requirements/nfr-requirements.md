# NFR Requirements - Unit 7 DVR Library Playback and Maintenance

## Performance Requirements
- Recorded-library reads should usually complete within 2 seconds under normal local-network conditions, even when multiple storage sources are consulted.
- Recorded playback start should remain comparable to existing playback startup behavior and must avoid additional avoidable round-trips after the target is validated.
- Explicit Live TV stop should release playback resources promptly enough that the UI does not feel hung or ambiguous.
- Post-delete refresh behavior should converge quickly enough that the removed item is not normally shown again after a confirmed deletion.

## Availability and Reliability Requirements
- Library reads may return degraded results when one or more storage sources fail, but the backend must still distinguish partial success from total failure.
- Recorded playback and deletion actions must never use stale unvalidated targets.
- Delete mutations must not be automatically retried because repeated destructive actions would be riskier than a clear failure.
- Live TV stop should be idempotent from the client perspective and should return a stable stopped outcome even if the session is already inactive.

## Security Requirements
- The backend must continue to own and validate all playback and delete targets derived from recording metadata.
- Raw `CmdURL` values and similar upstream mutation details must not be exposed to the client or emitted in routine logs.
- All recording identifiers, playback requests, deletion requests, and stop requests must be validated at the loopback boundary.
- Error responses must remain safe for direct client rendering and must not reveal internal source URLs beyond what is already required for backend execution.

## Logging and Observability Requirements
- Logs should distinguish storage-source read degradation, playback-target validation failure, delete-target validation failure, delete execution failure, and Live TV stop execution failure.
- Default logging should remain structured and info-level.
- Debug logging may include resolution decisions, but it must not dump raw delete URLs or other sensitive upstream action material.
- Runtime traces should make it possible to diagnose why a recording action required refresh rather than silently succeeding.

## Maintainability Requirements
- Unit 7 should extend the existing backend playback service rather than introducing a parallel playback orchestration path.
- Catalog normalization, duplicate merge logic, playback target resolution, delete target resolution, and stop behavior should remain modular enough for Unit 8 and Unit 9 reuse.
- Local-first ordering and strict mutation validation rules should stay centralized rather than being reimplemented in multiple handlers.

## Testability Requirements
- Unit 7 must include example-based tests for duplicate merge behavior, missing-recording handling, delete-target validation, and Live TV stop semantics.
- Unit 7 should add targeted property-based tests where merge or validation invariants benefit from input variation.
- Tests for destructive actions must confirm non-retry behavior and refresh-required outcomes.
- Stop behavior tests must confirm resource-release semantics and preserved remembered context.

## Usability Requirements
- The backend must give the client enough structured state to explain degraded library reads, missing recordings, and delete failures without generic error handling.
- Merged recording items should remain stable and understandable to the client despite multiple underlying storage variants.
- Live TV stop responses should clearly communicate that playback has ended but prior selection context remains available.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required. Unit 7 adds destructive actions and must log outcomes without exposing raw mutation targets.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable and required. Recording actions and stop controls must be validated at the loopback boundary.
- **SECURITY-08 Application-Level Access Control**: Applicable in local-service form. Recording actions remain backend-owned loopback operations.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable and required. The backend must prevent stale or client-supplied destructive targets from being executed.
- **SECURITY-11 Secure Design Principles**: Applicable and required. Unit 7 preserves backend trust boundaries for playback and deletion.
- **Other Security Rules**: N/A for this unit because no public remote API or cloud identity boundary is introduced.

## PBT Compliance Summary
- **PBT-03 Invariant Properties**: Applicable for duplicate merge stability, preferred-source ordering, and missing-recording action handling.
- **PBT-07 Generator Quality**: Applicable; generators are useful for source combinations, duplicate candidate sets, and action-resolution states.
- **PBT-08 Shrinking and Reproducibility**: Applicable and required.
- **PBT-09 Framework Selection**: Applicable and required.