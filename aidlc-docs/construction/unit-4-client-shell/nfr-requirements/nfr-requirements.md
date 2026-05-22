# NFR Requirements - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Performance Requirements
- The desktop window should appear immediately on launch, even while backend readiness is still being resolved.
- Backend-wait or restore-progress UI should be visible within about 1 second under normal local conditions.
- Once the backend is ready, the primary shell should become interactive quickly enough that startup feels continuous rather than staged.
- Channel rail navigation and diagnostics drawer expansion should feel immediate relative to backend round trips.

## Availability and Reliability Requirements
- The client must distinguish between backend unavailable, backend starting, backend ready, and backend failed states.
- The client must remain usable through recoverable failure states without requiring an app restart.
- The client shell must keep navigation, focus, and current context stable during channel switches and playback failures whenever the backend still exposes valid session context.
- The client must degrade to a clear empty or retryable state when no devices are available.

## Security Requirements
- The client must talk only to the loopback backend in v1.
- The client must not attempt direct device discovery, direct tuner polling, or direct player control.
- Client-visible error rendering must stay sanitized and must not expose raw backend internals or player-process details.
- Local UI preference storage must stay separate from backend-owned canonical state.

## Logging and Observability Requirements
- Default client logging should be sparse and focused on launch, backend readiness, and major UI state transitions.
- Debug mode may include deeper request and shell-state tracing for development.
- Client logs must not leak raw playback URLs, unsafe process details, or unrelated local filesystem paths by default.

## Maintainability Requirements
- The client shell should separate API integration, shell state, and QML presentation concerns.
- Unit 4 should treat backend response models as contracts that are adapted into UI-facing view state rather than used directly in all QML bindings.
- The shell scaffold should be incremental so mock state can be replaced with real backend-driven state without reworking the layout or navigation structure.

## Testability Requirements
- Unit 4 should support at least smoke-level launch validation once Qt6 development packages are available.
- View-state transformations and shell-state reducers should remain simple enough for focused tests.
- Backend contract tests remain the main source of truth for API correctness; client tests should focus on projection and interaction behavior.

## Usability Requirements
- The shell must keep the channel browser, playback stage, and diagnostics access understandable at a glance.
- Keyboard navigation should remain viable for the main shell surfaces.
- Failure states must preserve context and offer nearby recovery actions.
- Visual hierarchy should keep playback as the primary focus while leaving browsing and diagnostics reachable.

## Accessibility Requirements
- Primary shell controls must maintain readable contrast and clear focus indicators.
- Text and layout choices should remain legible on desktop displays without forcing full-screen use.
- Critical status changes must not depend on color alone.

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Applicable and required for client lifecycle logging.
- **SECURITY-05 Input Validation on All API Parameters**: Applicable where the client gathers user input before sending loopback requests.
- **SECURITY-08 Application-Level Access Control**: Applicable in local-service form; client must not widen access beyond loopback usage.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Applicable through sanitized error rendering and limited client-side runtime assumptions.
- **SECURITY-11 Secure Design Principles**: Applicable through clean separation between shell state, backend contracts, and playback/runtime internals.

## PBT Compliance Summary
- **PBT-03 Invariant Properties**: Potentially applicable for future pure shell-state reducers or response-to-view-model projections.
- **PBT-09 Framework Selection**: Deferred until client-side pure logic exceeds simple example-based coverage needs.