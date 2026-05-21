# NFR Design Patterns - Unit 1 Backend Foundation and Local API

## 1. Bounded Readiness Pattern
- **Pattern**: Client launches backend and waits within a bounded startup window.
- **Why**: Matches the approved 3-second readiness target and avoids uncontrolled background recovery behavior.
- **Behavior**:
  - one bounded wait window on startup
  - explicit failure state if readiness is not reached
  - retry only when the client or user initiates it

## 2. Loopback-Only Service Pattern
- **Pattern**: Bind HTTP server to localhost only.
- **Why**: Satisfies the approved v1 security boundary and reduces accidental network exposure.
- **Behavior**:
  - no LAN binding
  - no remote mode in Unit 1
  - API contract designed for local client consumption only

## 3. Stable Contract Placeholder Pattern
- **Pattern**: Provide stable endpoint shapes even when downstream feature units are not complete.
- **Why**: Lets the client integrate early and reduces contract churn.
- **Behavior**:
  - health and state endpoints are real
  - device and playback endpoints may return provisional or not-yet-available contract responses
  - responses remain structurally stable

## 4. Structured Error Envelope Pattern
- **Pattern**: Convert all client-visible failures into a standard error envelope.
- **Why**: Supports retry-oriented UX and avoids leaking internal runtime details.
- **Behavior**:
  - every API error includes `code`, `message`, and `retryHint`
  - optional sanitized `details` field only when safe
  - no stack traces or internal paths

## 5. XDG State Persistence Pattern
- **Pattern**: Persist canonical backend state in the Linux XDG state directory.
- **Why**: Fits Linux desktop conventions without overcomplicating Unit 1.
- **Behavior**:
  - canonical remembered state stored outside packaged runtime directories
  - state survives app restarts
  - state remains backend-owned even if the client caches UI-only preferences elsewhere

## 6. Structured Logging Pattern
- **Pattern**: Use structured info-level logging with opt-in debug mode.
- **Why**: Supports troubleshooting and security-baseline logging constraints without overwhelming normal runs.
- **Behavior**:
  - info by default
  - debug enabled explicitly
  - request, startup, and validation events are logged consistently

## 7. Property-Testable Contract Pattern
- **Pattern**: Keep persisted state and API error structures simple and explicit so they can be tested with example-based tests plus `proptest`.
- **Why**: Matches the partial PBT policy and the selected Rust testing approach.
- **Behavior**:
  - round-trip serialization properties for persisted state
  - invariant testing for structured error envelopes
  - reproducible shrinking and seed logging