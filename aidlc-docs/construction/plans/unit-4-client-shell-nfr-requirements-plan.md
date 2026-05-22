# NFR Requirements Plan - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Execution Checklist
- [x] Review Unit 4 functional design and current backend contract surface
- [x] Choose the client runtime and UI stack direction
- [x] Define responsiveness, failure, and usability expectations for the desktop shell
- [x] Define observability and security expectations for the loopback client
- [x] Define maintainability and testability expectations for the client shell
- [x] Generate nfr-requirements.md
- [x] Generate tech-stack-decisions.md

## Chosen Default Decisions
- **UI stack**: Qt6 Quick with Qt Quick Controls 2 and CMake.
- **Responsiveness target**: the app window should appear immediately, backend-wait UX should be visible within about 1 second, and the primary shell should become interactive quickly after backend readiness under normal local conditions.
- **Failure UX**: no modal dead ends for normal startup, device, or playback failures; use inline recoverable states instead.
- **Client logging**: minimal client-side lifecycle logging by default, with deeper request or state tracing only in debug mode.
- **Test depth**: keep backend contract tests authoritative, add client smoke validation and view-model or shell-state tests when Qt6 test tooling is available.