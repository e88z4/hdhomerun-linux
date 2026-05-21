# Application Design

## Overview

The application is designed as a bundled two-part Linux product:
- a standalone local backend service
- a Qt/QML desktop client

The backend owns device integration, playback orchestration, canonical remembered state, and diagnostics aggregation. The client owns presentation, user navigation, and local UI preferences. Communication happens over loopback HTTP/JSON only in v1.

## Key Design Decisions
- Loopback HTTP/JSON is the v1 client-backend boundary.
- The desktop app may auto-start the backend by default, while still allowing managed-service usage.
- Backend owns canonical remembered state such as last device and last watched channel.
- Tuner and signal details appear as summary indicators plus a dedicated diagnostics panel or drawer.
- Playback remains inside one persistent in-app session and does not hand off to an external player window.

## Component Summary
- **Qt/QML client shell**: overall desktop UX and navigation
- **Channel browser**: device and lineup interaction
- **Embedded player**: persistent in-app playback surface
- **Diagnostics panel**: expanded tuner and signal visibility
- **Backend API host**: loopback HTTP boundary with input validation
- **Device integration**: libhdhomerun adapter layer
- **Playback session controller**: persistent playback orchestration
- **Playback engine adapter**: mpv or libmpv wrapper
- **State store and state service**: canonical remembered state
- **Service launcher and supervisor**: runtime startup and readiness path

## Security Compliance Summary
- **SECURITY-03 Application-Level Logging**: Compliant in design intent. Structured logging is required in the backend API host and services.
- **SECURITY-04 HTTP Security Headers**: N/A for the Qt/QML desktop client architecture as currently designed. No HTML-serving web application is planned for v1.
- **SECURITY-05 Input Validation on All API Parameters**: Compliant in design intent. Validation is required at every loopback API boundary.
- **SECURITY-08 Application-Level Access Control**: Compliant in design intent for a local-only service model. The service will remain loopback-only and avoid unnecessary exposure beyond the local machine.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Compliant in design intent. Error responses must be safe, and the service should not expose internal runtime details.
- **SECURITY-11 Secure Design Principles**: Compliant. Security-sensitive concerns are concentrated at the backend API boundary and runtime supervision boundary.
- **Other Security Rules**: N/A at this stage because no cloud infrastructure, public network intermediaries, or remote data stores are part of the current design.

## PBT Compliance Summary
- **PBT-09 Framework Selection**: N/A at this stage. Framework selection belongs in NFR Requirements.
- **Other PBT Rules**: N/A at this stage. Property identification and test generation belong to later design and code stages.

## Design Conclusion

This design preserves reusability without overcomplicating v1. The backend remains reusable and future-friendly, while the client remains free to focus on a modern Linux UX. The chosen boundaries also set up the next stage cleanly: unit decomposition across backend foundation, HDHomeRun integration, playback orchestration, client shell, and packaging.