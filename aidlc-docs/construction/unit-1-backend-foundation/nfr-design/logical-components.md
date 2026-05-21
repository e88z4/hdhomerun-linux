# Logical Components - Unit 1 Backend Foundation and Local API

## 1. Runtime Supervisor
- **Purpose**: Coordinates backend process lifecycle as seen by the client startup path.
- **NFR Role**:
  - bounded startup wait
  - readiness probe coordination
  - explicit failure and retry boundaries

## 2. Loopback API Boundary
- **Purpose**: Exposes the Rust service to the local client through HTTP/JSON.
- **NFR Role**:
  - localhost-only binding
  - request validation
  - stable contract shapes

## 3. Contract Placeholder Layer
- **Purpose**: Holds provisional device and playback contract responses until later units are implemented.
- **NFR Role**:
  - contract stability
  - early client integration
  - predictable non-ready behavior

## 4. Error Mapping Layer
- **Purpose**: Translates internal failures and validation problems into stable structured error envelopes.
- **NFR Role**:
  - sanitized responses
  - retry-oriented UX support
  - security-safe error handling

## 5. State Persistence Layer
- **Purpose**: Owns canonical remembered state using the XDG state directory strategy.
- **NFR Role**:
  - persistent restore context
  - simple local durability
  - backend-owned state authority

## 6. Structured Logging Layer
- **Purpose**: Provides consistent event logging for startup, requests, and failures.
- **NFR Role**:
  - info-level default logging
  - explicit debug mode
  - diagnosable startup and validation behavior

## 7. Testability Support Layer
- **Purpose**: Keeps serializable state and error contracts easy to validate through standard tests and property-based tests.
- **NFR Role**:
  - generator-friendly domain structures
  - reproducible PBT support
  - complement to example-based regression tests

## Component Relationships
- Runtime Supervisor depends on Loopback API Boundary for readiness checks.
- Loopback API Boundary depends on Error Mapping Layer and Contract Placeholder Layer.
- State Persistence Layer feeds restore-state flows through the Loopback API Boundary.
- Structured Logging Layer spans all runtime-facing components.
- Testability Support Layer constrains how State Persistence and Error Mapping structures are modeled.