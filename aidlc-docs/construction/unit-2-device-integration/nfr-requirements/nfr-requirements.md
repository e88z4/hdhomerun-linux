# NFR Requirements - Unit 2 HDHomeRun Discovery and Device Integration

## Performance Requirements
- Startup discovery should begin automatically during client bootstrap and should usually return results within about 1 to 2 seconds on a healthy local network.
- Selected-device lineup loading through `lineup.json` should usually complete within about 1 to 2 seconds on a healthy local network.
- Lightweight periodic background refresh must not create noticeable UI stalls or block foreground selection and lineup requests.
- Tuner diagnostics retrieval should be fast enough to support an interactive diagnostics panel without requiring aggressive polling.

## Availability and Reliability Requirements
- Unit 2 must support startup discovery plus lightweight periodic background refresh in v1.
- If a remembered device is missing, the backend must degrade to a structured selection-needed result instead of undefined fallback behavior.
- Partial tuner-status failures must return available tuner diagnostics and explicitly mark failed tuner entries as unavailable.
- Lineup retrieval failures must produce stable structured unavailable or error results rather than malformed or silently truncated contracts.
- Discovery and lineup refresh behavior should tolerate transient LAN-device failures without leaving the backend in an ambiguous state.

## Security Requirements
- Unit 1 loopback-only backend exposure remains mandatory in Unit 2.
- LAN communication with HDHomeRun devices must be outbound from the local backend only; Unit 2 must not introduce remote inbound control surfaces.
- Device-provided strings and HTTP metadata must be validated and normalized before they become part of the client-visible contract.
- Errors and logs must not expose raw library pointers, socket internals, or unsafe implementation details.
- Background refresh behavior must avoid excessive device polling that could resemble accidental misuse or misconfiguration.

## Logging and Observability Requirements
- Default runtime logging for Unit 2 should remain high-level structured logging at info level.
- Device-call detail, raw response troubleshooting, and deeper discovery traces may be emitted only in explicit debug logging modes.
- Logs should make it possible to distinguish discovery failure, lineup retrieval failure, stale remembered-device mismatch, and partial tuner-status failure.
- Observability should support diagnosis of LAN-device communication issues without exposing sensitive internal details to the client.

## Maintainability Requirements
- The device-integration layer should isolate `libhdhomerun` and HTTP lineup retrieval behind backend-owned adapter boundaries.
- Discovery, lineup normalization, and tuner-diagnostic mapping should produce stable internal models that later units can reuse without reshaping public contracts.
- Unit 2 should extend the Unit 1 backend foundation incrementally rather than introducing a second state or contract authority.
- Refresh cadence and timeout behavior should remain explicit configuration points in code, not implicit scattered constants.

## Testability Requirements
- Unit 2 must include example-based tests for discovery normalization, remembered-device reconciliation, restricted-channel mapping, and lineup parsing behavior.
- Property-based tests should cover pure normalization invariants such as explicit availability classification, stable device-reference mapping, and sanitized contract shaping.
- Property-based testing should remain focused on pure logic boundaries rather than attempting to fuzz live device communication directly.
- Tests must be able to simulate partial tuner-status failures without requiring physical hardware for every run.
- PBT failures must remain reproducible with seed logging.

## Usability Requirements
- Startup discovery and initial lineup loading should feel responsive on a healthy local network.
- Background refresh should improve device freshness without causing distracting UI churn.
- Restricted channels should remain visible with clear unavailable or restricted semantics.
- Diagnostics UX should still be useful when one tuner fails to report status, because other tuners remain visible.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required for structured discovery, lineup, and tuner diagnostics events.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable and required for Unit 2 request validation and device metadata normalization.
- **SECURITY-08 Application-Level Access Control**: Applicable in loopback-only form; Unit 2 must not expand backend exposure beyond local access.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable and required for safe polling cadence, sanitized vendor errors, and explicit refresh behavior.
- **SECURITY-11 Secure Design Principles**: Applicable and required through minimal exposure, strict normalization, and separation of vendor-facing and client-facing models.

## PBT Compliance Summary
- **PBT-02 Round-Trip Properties**: Applicable for normalized entity serialization where Unit 2 persists or reuses normalized references.
- **PBT-03 Invariant Properties**: Applicable for availability classification, selection-required behavior, and sanitized contract invariants.
- **PBT-07 Generator Quality**: Applicable; generators are required for normalized devices, lineup entries, and restricted-channel tag combinations.
- **PBT-08 Shrinking and Reproducibility**: Applicable and required.
- **PBT-09 Framework Selection**: Applicable and continues to use the Rust-standard project choice from Unit 1.