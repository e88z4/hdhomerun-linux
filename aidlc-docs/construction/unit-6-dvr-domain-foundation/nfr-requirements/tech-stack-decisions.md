# Tech Stack Decisions - Unit 6 DVR Domain Foundation

## Primary Language
- **Choice**: Rust
- **Rationale**:
  - Matches the established backend implementation direction.
  - Supports strong domain modeling for readiness, rule validation, and projection logic.
  - Avoids cross-unit stack drift in the middle of the DVR increment.

## Runtime and HTTP Layer
- **Choice**: Reuse the established Tokio and Axum-style backend direction from earlier units.
- **Rationale**:
  - Keeps loopback API behavior consistent across live-TV and DVR endpoints.
  - Fits the existing validation, middleware, and structured-error design.

## Serialization and Contract Modeling
- **Choice**: Serde-based JSON serialization and strongly typed request or response models.
- **Rationale**:
  - Keeps DVR readiness, rule, and schedule contracts explicit.
  - Helps enforce structured validation and safe error shaping.

## Logging
- **Choice**: Reuse `tracing`-based structured logging with info as the default level.
- **Rationale**:
  - Supports diagnosis of readiness evaluation, rule writes, and confirmation failures.
  - Keeps sensitive upstream material out of normal log flows.

## Error Modeling
- **Choice**: Typed DVR-domain errors converted into stable loopback API error payloads.
- **Rationale**:
  - Separates client-safe outcomes such as invalid-airing or unsupported-option failures from internal upstream failure details.
  - Preserves the non-guessing error policy required by the functional design.

## Upstream Interaction Model
- **Choice**: Use explicit adapter boundaries for vendor DVR APIs and related readiness or schedule reads.
- **Rationale**:
  - Keeps upstream-specific behavior isolated from domain logic.
  - Makes validation and retry policy easier to test.

## Retry Strategy
- **Choice**: Bounded retries for safe read operations only; no automatic retries for rule mutations.
- **Rationale**:
  - Reduces noise from transient read failures without risking duplicate or ambiguous writes.
  - Fits the requirement for confirmed post-write state.

## Property-Based Testing Framework
- **Choice**: `proptest`
- **Rationale**:
  - Matches the approved repo direction for Rust backend units.
  - Supports generators and shrinking for readiness-condition sets, rule-option combinations, and schedule-projection inputs.

## Example-Based Testing
- **Choice**: Rust standard test framework alongside `proptest`
- **Rationale**:
  - Business-critical DVR flows need pinned example regressions for vendor-specific edge cases.
  - PBT complements rather than replaces example-based tests.

## Stack Constraints
- No stack drift away from the established Rust backend direction.
- Loopback-only exposure remains mandatory.
- Sensitive upstream material must remain transient and sanitized.
- Confirmed post-write state is required before success is reported.