# Infrastructure Design Plan - Unit 4 Qt/QML Client Shell and Live-TV User Journey

## Execution Checklist
- [x] Review Unit 4 functional, NFR, and packaging context
- [x] Define runtime relationship between the Qt client, loopback backend, and backend-owned playback process
- [x] Define local filesystem, config, and runtime expectations for the client
- [x] Define development and packaged execution expectations
- [x] Generate infrastructure-design.md
- [x] Generate deployment-architecture.md

## Chosen Default Decisions
- **Runtime shape**: separate Qt client process talking to the loopback backend over HTTP.
- **Client startup**: desktop client remains the primary launcher and backend readiness coordinator.
- **Preference storage**: client stores only local presentation preferences in XDG config-style locations.
- **Display support**: design for normal Linux desktop environments with Qt support for Wayland and X11 fallback.