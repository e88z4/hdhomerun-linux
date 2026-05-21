# Requirements

## Intent Analysis Summary
- **User Request**: Create an HDHomeRun player for Linux using the SiliconDust sources and documentation in the workspace.
- **Request Type**: New project in an existing vendor-code workspace.
- **Scope Estimate**: Multiple components, including a backend service, desktop client, playback integration, and Linux packaging.
- **Complexity Estimate**: Complex.

## Product Summary

Build a modern Linux TV player for HDHomeRun devices, specifically suitable for an HDHomeRun Flex 4K environment. The product will be a native Linux desktop application composed of a reusable local backend service bundled with a Qt/QML desktop client. The system will use `libhdhomerun` as the primary device integration boundary and `mpv/libmpv` as the playback engine for reliable MPEG-TS live-TV playback.

## Functional Requirements

### FR-1 Device Discovery
- The backend must discover compatible HDHomeRun devices on the local network.
- The UI must present discovered devices to the user.
- The system must support selecting a target device when more than one device is found.

### FR-2 Channel and Tuner Data
- The system must retrieve the available channel lineup for the selected device.
- The UI must show a browsable channel list.
- The system must expose tuner status and basic signal information in the UI.

### FR-3 Live Playback
- The system must start live playback for a selected channel.
- The player must keep playback inside a persistent in-app session.
- The system must support switching channels within that persistent player session.
- The playback engine must handle MPEG-TS live streams reliably on Linux.

### FR-4 Backend and Client Separation
- The product must be built as two bundled components:
  - a local backend service
  - a desktop UI client
- The backend must own device discovery, tuner control, and playback-session orchestration.
- The desktop client must communicate with the backend through a stable local interface.

### FR-5 Linux Packaging
- The first deliverable must include:
  - AppImage packaging
  - Flatpak packaging
  - Debian package packaging

### FR-6 Local Testing
- The implementation must be runnable and testable locally on Linux during development.
- The product must support validation against a real HDHomeRun device on the local network.

## Explicitly Deferred Requirements
- Recording or DVR functionality is out of scope for v1.
- Browser extension delivery is out of scope.
- Browser-native playback is out of scope.

## Architectural Requirements

### AR-1 Device Integration Strategy
- `libhdhomerun` must be the center of device discovery and tuner-control integration.
- The design may use the device HTTP stream endpoint where that is the simplest reliable playback input path.
- The architecture must avoid unnecessary reinvention of SiliconDust device-control logic.

### AR-2 UI Technology Direction
- The desktop client should be implemented with Qt/QML.
- The UI should look modern, polished, and intentionally designed rather than utilitarian.

### AR-3 Reusability
- The backend must be intentionally reusable from day one.
- The backend should be structured so future clients such as CLI tools, alternate UIs, or automation can reuse the same service boundary.

## Non-Functional Requirements

### NFR-1 Playback Reliability
- Live playback must prioritize reliability over novelty.
- The playback path must be appropriate for MPEG-TS streams on Linux.
- Channel changes should be responsive enough to feel like a coherent live-TV product.

### NFR-2 Security Baseline
- Security baseline rules are enabled and must be treated as blocking constraints during implementation.
- The local backend service must avoid unnecessary exposure beyond the local machine.

### NFR-3 Testability
- Property-based testing rules are partially enabled.
- Pure functions and serialization-style logic introduced by the new project should be designed so they can be property-tested.
- The system should also support practical local integration tests against a live device.

### NFR-4 Maintainability
- Device logic, playback orchestration, and UI concerns should remain clearly separated.
- The initial project layout should support future expansion without forcing a rewrite.

### NFR-5 User Experience
- The product should present a modern and sleek interface.
- The core path from launch to live playback should stay simple and fast.

## Acceptance Criteria
- A Linux user can launch the packaged app.
- The app discovers an HDHomeRun device on the local network.
- The user can browse channels and inspect tuner status or signal info.
- The user can start live playback for a selected channel.
- The user can switch channels without leaving the player view.
- The project can produce AppImage, Flatpak, and Debian package outputs.

## Key Decisions Captured
- Native desktop app selected over browser extension.
- Two-part bundled architecture selected over a single-process-only design.
- Qt/QML selected over GTK.
- Persistent in-app player session selected for channel switching.
- Live-TV-only v1 scope selected; recording deferred.
- Packaging breadth is part of the first deliverable, not a later follow-up.