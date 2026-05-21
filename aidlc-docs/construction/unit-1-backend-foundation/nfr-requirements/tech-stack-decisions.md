# Tech Stack Decisions - Unit 1 Backend Foundation and Local API

## Primary Language
- **Choice**: Rust
- **Rationale**:
  - Strong fit for a long-lived local service process
  - Good error modeling and type-safe API contract handling
  - Good packaging story for Linux desktop distribution
  - Strong property-based testing ecosystem

## Runtime and HTTP Layer
- **Recommended Direction**: Tokio-based async runtime with an HTTP framework such as Axum
- **Rationale**:
  - Good fit for a loopback HTTP/JSON service
  - Clean request validation and middleware story
  - Good integration with structured logging and testing

## Serialization
- **Recommended Direction**: Serde-based JSON serialization
- **Rationale**:
  - Stable and explicit contract modeling
  - Useful for both loopback API payloads and persisted local state

## Logging
- **Recommended Direction**: `tracing` plus structured subscriber configuration
- **Default Level**: info
- **Rationale**:
  - Structured logs for runtime and request-level events
  - Easy debug-mode escalation without changing production defaults

## Error Modeling
- **Recommended Direction**: typed application errors converted into stable API error payloads
- **Rationale**:
  - Keeps internal failures separate from client-visible error contracts
  - Supports the required `code`, `message`, and `retryHint` shape

## Local Persistence
- **Recommended Direction**: simple serialized local state file for Unit 1 rather than introducing a database immediately
- **Rationale**:
  - Unit 1 only needs canonical remembered state, not complex query capabilities
  - Simpler startup and packaging footprint
  - Supports round-trip property testing directly

## Property-Based Testing Framework
- **Choice**: `proptest`
- **Rationale**:
  - Best-fit standard framework for Rust in this project context
  - Supports custom generators, shrinking, and reproducible failures
  - Matches the enabled partial PBT policy for this repo

## Example-Based Testing
- **Recommended Direction**: Rust standard test framework with explicit regression tests alongside `proptest`
- **Rationale**:
  - Business-critical startup and restore behaviors still need pinned example-based tests
  - PBT complements but does not replace those tests

## Stack Constraints
- Loopback-only binding in v1
- No remote-access support in Unit 1
- Stable provisional contract endpoints must be available early
- Startup target is readiness within 3 seconds under normal local conditions