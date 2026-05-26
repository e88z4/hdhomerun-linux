# NFR Requirements - Unit 6 DVR Domain Foundation

## Performance Requirements
- DVR readiness checks and schedule-refresh reads should usually complete within 1 second under normal local conditions.
- Rule create or update flows may take longer than simple reads, but the backend should still return confirmed post-write state without avoidable delay.
- The backend should keep readiness and schedule-projection refreshes efficient enough that the DVR workspace does not feel stale during normal navigation.

## Availability and Reliability Requirements
- Unit 6 must distinguish successful upstream reads from degraded or unavailable DVR state.
- Safe read operations may use bounded retries when upstream calls fail transiently.
- Rule mutations must not be automatically retried because duplicate writes or ambiguous write outcomes would be riskier than a clear failure.
- When upstream confirmation cannot be obtained after a write, the backend must return a clear failure or uncertain-state result rather than pretending success.

## Security Requirements
- Sensitive upstream material such as DeviceAuth-related values and vendor-returned metadata must be handled transiently and sanitized from logs.
- Any derived metadata that influences later actions must be validated before it is trusted.
- Loopback API validation remains mandatory for every DVR readiness, rule, and schedule endpoint.
- Error responses must remain sanitized and safe for direct client rendering.

## Logging and Observability Requirements
- Default runtime logging should remain info-level structured logging.
- Logs should distinguish readiness evaluation, rule validation failure, upstream read failure, upstream write failure, and post-write confirmation failure.
- Debug logging may exist for development, but it must not dump raw sensitive upstream material in normal runs.
- Request correlation or equivalent traceability should make rule mutations diagnosable without unsafe payload exposure.

## Maintainability Requirements
- Unit 6 must stay on the established backend stack rather than introducing a parallel API or runtime model.
- Validation logic, upstream adapters, and schedule-projection logic should remain modular enough that Unit 7 and Unit 8 can reuse stable contracts.
- Unsupported option handling must stay explicit and centrally defined rather than scattering ad hoc checks across handlers.

## Testability Requirements
- Unit 6 must include both example-based tests and targeted property-based tests.
- Property-based tests should cover at minimum:
  - readiness invariants around blocking conditions and usability
  - rule validation invariants for supported versus unsupported option combinations
  - schedule-projection invariants that prefer explicit upcoming-recordings evidence over guessed state
- PBT failures must remain reproducible with seed logging.

## Usability Requirements
- Readiness results must give the client enough structured information to present actionable DVR guidance.
- Rule failures should be understandable enough that the client can steer the user toward correction rather than generic failure handling.
- Schedule-projection responses should remain stable and understandable without requiring the client to decode raw vendor concepts.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required. Unit 6 needs structured audit-friendly logging for readiness and rule flows without leaking sensitive upstream material.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable and required. DVR rule, filter, and scheduling inputs must be validated at the loopback boundary.
- **SECURITY-08 Application-Level Access Control**: Applicable in local-service form. DVR endpoints remain loopback-only and are not exposed as remote APIs.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable and required. Sensitive upstream material must stay transient, and derived metadata must be validated before use.
- **SECURITY-11 Secure Design Principles**: Applicable and required. Unit 6 keeps DVR trust decisions in the backend and avoids client-side guessing.
- **Other Security Rules**: N/A for this unit because no remote public service exposure or cloud IAM boundary is introduced.

## PBT Compliance Summary
- **PBT-03 Invariant Properties**: Applicable for readiness, validation, and schedule-projection rules.
- **PBT-07 Generator Quality**: Applicable; generators are needed for readiness-condition sets, rule intents, and projection inputs.
- **PBT-08 Shrinking and Reproducibility**: Applicable and required.
- **PBT-09 Framework Selection**: Applicable and required.