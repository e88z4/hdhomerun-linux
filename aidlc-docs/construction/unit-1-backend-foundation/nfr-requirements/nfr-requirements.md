# NFR Requirements - Unit 1 Backend Foundation and Local API

## Performance Requirements
- Backend auto-start under normal local conditions should reach readiness within 3 seconds.
- Health and bootstrap endpoints should respond quickly enough to keep desktop startup feeling immediate once the backend is ready.
- Structured validation and error shaping must not introduce user-visible startup lag beyond the approved readiness target.

## Availability and Reliability Requirements
- Backend runtime must distinguish between process started and service ready.
- Client startup flow must be able to detect readiness, failed startup, and unhealthy states cleanly.
- Loopback API contract shapes must remain stable even when later unit behaviors are not fully implemented yet.
- The backend should degrade to predictable structured error or placeholder responses rather than failing with undefined behavior.

## Security Requirements
- Backend must bind only to localhost in v1.
- No remote or LAN access support is part of Unit 1.
- Every loopback API input must be validated at the API boundary.
- Errors must be sanitized and must not expose internal runtime details.
- Structured logging must omit sensitive data.

## Logging and Observability Requirements
- Default runtime logging should be info-level structured logging.
- Debug logging may be enabled explicitly for development or troubleshooting.
- Logs should include timestamps, severity, and request or correlation information where applicable.
- Startup failures and validation failures must be diagnosable without exposing secrets.

## Maintainability Requirements
- Unit 1 should define stable response contracts that later units can consume without rework.
- Canonical remembered state must stay backend-owned.
- State persistence must be simple, explicit, and easy to reason about in early iterations.
- The Rust backend foundation should favor a small, understandable core rather than early overengineering.

## Testability Requirements
- Unit 1 must include both example-based tests and property-based tests where appropriate.
- Property-based tests should at minimum cover:
  - restore-state serialization round-trip
  - structured error invariants
  - sanitized bootstrap and error-response invariants
- PBT failures must be reproducible with seed logging.

## Usability Requirements
- Backend startup and readiness behavior must support a smooth desktop app launch experience.
- Structured errors should be suitable for client-side rendering into clear retry-oriented UX.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable and required.
- **SECURITY-08 Application-Level Access Control**: Applicable in local-service form as loopback-only exposure; remote access remains out of scope.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable and required for sanitized errors and minimal exposure.
- **SECURITY-11 Secure Design Principles**: Applicable and required through layered validation, logging, and loopback-only runtime design.
- **Other Security Rules**: N/A for this unit because there are no public network intermediaries, remote data stores, or cloud IAM boundaries in Unit 1.

## PBT Compliance Summary
- **PBT-02 Round-Trip Properties**: Applicable for persisted restore state serialization.
- **PBT-03 Invariant Properties**: Applicable for structured error and sanitized-response invariants.
- **PBT-07 Generator Quality**: Applicable; domain generators are required for restore-state records and API error objects.
- **PBT-08 Shrinking and Reproducibility**: Applicable and required.
- **PBT-09 Framework Selection**: Applicable and required.