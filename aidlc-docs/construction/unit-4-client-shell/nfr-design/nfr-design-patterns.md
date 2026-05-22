# NFR Design Patterns - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## 1. Bounded Launch Overlay Pattern
- **Pattern**: Show the shell immediately but gate primary interaction behind a bounded launch overlay while backend readiness and bootstrap are resolved.
- **Why**: Preserves perceived responsiveness while keeping startup truthful.
- **Behavior**:
  - immediate window visibility
  - explicit backend wait messaging
  - recoverable startup failure state with retry action

## 2. Contract Projection Pattern
- **Pattern**: Project backend responses into client shell state before QML binds to them.
- **Why**: Prevents backend contract details from leaking into every visual component.
- **Behavior**:
  - one adaptation layer for bootstrap, devices, lineup, playback, and diagnostics
  - presentation state stays stable as contracts evolve

## 3. Player-First Persistent Layout Pattern
- **Pattern**: Keep one persistent playback stage at the center of the shell while channel browsing and diagnostics remain adjacent.
- **Why**: Matches the approved live-TV interaction model and avoids view churn.
- **Behavior**:
  - channel rail remains available
  - playback stage remains dominant
  - diagnostics do not replace the main viewing context

## 4. Inline Recovery Surface Pattern
- **Pattern**: Render recoverable backend, discovery, and playback failures inline in the current workflow.
- **Why**: Keeps the user oriented and reduces restart-style recovery behavior.
- **Behavior**:
  - retry action stays near the failure
  - current channel and device context remain visible where safe
  - diagnostics access remains available

## 5. Explicit Device Handoff Pattern
- **Pattern**: Treat device changes as explicit shell actions rather than silent background state swaps.
- **Why**: Prevents accidental disruption of active playback and makes selection state understandable.
- **Behavior**:
  - visible device selector
  - explicit switch flow when playback is active
  - backend-selected device remains authoritative

## 6. Mock-to-Real Shell Scaffold Pattern
- **Pattern**: Build the client shell shape with mock-driven state first, then replace it incrementally with real backend integration.
- **Why**: Allows rapid UI structure progress without binding the first shell version to unfinished contract edges.
- **Behavior**:
  - layout and interaction surfaces stabilize early
  - mock state is deliberately shaped to match backend-driven view models
  - real integration can replace scaffolding one surface at a time