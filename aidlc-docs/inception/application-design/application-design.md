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

## DVR Increment Overview

The DVR increment extends the existing live-TV architecture instead of introducing a second application flow. The client gains a DVR workspace and rule-editing surfaces, while the backend stays solely responsible for vendor DVR APIs, storage discovery, recorded-file resolution, and destructive operations. Recorded playback and explicit Live TV stop both reuse the existing playback boundary so playback state remains coherent.

## DVR Key Design Decisions
- The DVR tab uses a split layout so the recording list remains stable while details and actions update independently.
- Rule management follows a hybrid UX: contextual entry points plus a fuller editor for advanced rule options.
- The backend exposes a clear DVR endpoint group rather than overloading guide endpoints, but it still reuses the established playback boundary for play and stop actions.
- Recorded-library assembly uses local-first storage ordering, with non-local sources included only after local candidates are considered.
- Delete and playback actions derived from DVR metadata remain backend-owned so the client never handles raw vendor command targets.

## DVR Component Summary
- **DVR workspace component**: split-view recordings, details, and actions.
- **Recording rule editor component**: hybrid quick-create and full-edit flow.
- **DVR library service**: readiness computation and unified home payload.
- **Recording rule service**: vendor-side rule lifecycle.
- **Recording playback service**: recorded-file handoff into existing playback orchestration.
- **Recording maintenance service**: deletion validation, sync, and refresh.
- **DVR integration component**: vendor DVR and record-engine API adapter.
- **Recording catalog resolver**: local-first storage prioritization and normalization.
- **Playback stop coordinator**: explicit user stop for live sessions without app exit.

## DVR Process Summary
1. The client opens the DVR workspace and requests readiness plus home data.
2. The backend discovers DVR capability, loads recordings, and builds a local-first merged catalog.
3. The user can inspect a recording, manage a rule, start playback, or delete an item.
4. Playback requests flow back through the established playback session controller.
5. Stop-live requests terminate the current live session cleanly while the rest of the app stays active.

## DVR Security and Extension Compliance Summary
- **SECURITY-03 Application-Level Logging**: Compliant in design intent. DVR actions should emit structured backend logs for rule changes, delete attempts, playback resolution, and stop requests without leaking sensitive tokens.
- **SECURITY-04 HTTP Security Headers**: N/A for this local desktop architecture. No browser-delivered HTML surface is introduced by the DVR increment.
- **SECURITY-05 Input Validation on All API Parameters**: Compliant in design intent. Recording IDs, rule payloads, filter inputs, and stop requests must be validated at the loopback API boundary.
- **SECURITY-08 Application-Level Access Control**: Compliant in design intent for a local-only service model. DVR endpoints remain loopback-only and are not exposed as remote APIs.
- **SECURITY-09 Security Hardening and Misconfiguration Prevention**: Compliant in design intent. Vendor-returned playback or delete command targets must be validated against discovered local DVR storage context before use.
- **SECURITY-11 Secure Design Principles**: Compliant. The client remains presentation-only for DVR integration, while privileged DVR operations stay concentrated in backend services.
- **PBT Rules**: N/A at this stage. Property selection for catalog ordering, rule translation, and stop idempotency belongs in later design and test stages.

## DVR Design Conclusion

This DVR design keeps the system coherent by extending the existing backend-centric architecture rather than creating a second control plane. It gives the next stages a clear decomposition path: DVR library and readiness, rule lifecycle, recorded playback and deletion, client DVR workspace, and playback stop behavior.