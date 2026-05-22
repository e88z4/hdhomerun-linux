# Client Shell

This directory contains the first Unit 4 Qt/QML shell scaffold.

## Current Scope

- Qt6 Quick desktop shell skeleton
- player-first layout with channel rail, playback stage, and diagnostics drawer
- real loopback backend wiring for bootstrap, devices, lineup, playback current, diagnostics, device selection, and playback retry
- embedded playback stage driven by Qt Multimedia when the backend runs in client-managed playback mode

## Current Unit 4 Gaps

The shell now talks to the backend and renders an embedded playback surface, but Unit 4 still needs:

- richer client-side error handling around media-surface failures and backend reconnect paths
- deeper automation coverage for the client-backend playback flow

The current strategy uses an embedded Qt Multimedia surface in the center stage while the backend remains the canonical owner of device and playback session state. When the client launches the backend itself, it forces HDHR_BACKEND_PLAYER_MODE=client so the backend skips external mpv spawning and lets the in-window surface render the playback URL.

## Local Build Prerequisites

- CMake
- Ninja
- Qt6 Quick and Quick Controls 2 development packages

## Validation Status

- Configured successfully with CMake and Ninja on Debian sid after installing Qt6 development packages.
- Built successfully at build/client/hdhomerun-linux-player.
- Passed a short offscreen launch smoke test with QT_QPA_PLATFORM=offscreen.
- Passed the automated CTest smoke check with ctest --test-dir build/client --output-on-failure.

## Runtime Environment Variables

- `HDHR_BACKEND_URL`: override the loopback backend base URL. Default is `http://127.0.0.1:38080`.
- `HDHR_BACKEND_CMD`: override the backend executable path used when the client attempts to start the backend.
- `HDHR_BACKEND_PLAYER_MODE`: backend playback adapter mode. The client sets this to `client` when it auto-starts the backend for embedded playback.

Example build flow once Qt6 is available:

```sh
cmake -S client -B build/client -G Ninja
cmake --build build/client
ctest --test-dir build/client --output-on-failure
```
