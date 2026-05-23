# Desktop Client

This directory contains the Qt/QML desktop application used by HDHomeRun Linux Player.

## What The Client Does

- launches the user interface
- starts the bundled backend automatically when needed
- restores device and playback context from the backend
- renders embedded live playback with Qt Multimedia
- shows tuner diagnostics alongside playback
- supports fullscreen playback, keyboard channel switching, and client-side volume control
- presents playback controls through icon-based shell and overlay actions with tooltips
- keeps a full EPG grid docked under playback instead of a compact channel strip
- uses a playback-overlay control bar for volume and fullscreen actions in both windowed and fullscreen modes
- shows SiliconDust guide-backed channel logos, current-show titles, and program artwork when the backend resolves guide data for the selected device
- uses `Up` and `Down` to move between channels in the guide, while `Page Up` and `Page Down` control volume
- keeps diagnostics inside the playback stage as a compact inline status block instead of a separate drawer
- inhibits the screensaver and idle sleep while live playback is active

## User-Facing Behavior

The client is the entry point for the packaged application. Users normally do not need to start the backend manually.

On startup the client:

- checks whether the local backend is already reachable
- starts the bundled backend automatically if required
- loads devices, lineup data, current playback state, and tuner diagnostics

## Build From Source

Prerequisites:

- CMake
- Ninja
- Qt6 Quick, Quick Controls 2, Network, and Multimedia development packages

Build and test:

```sh
cmake -S client -B build/client -G Ninja
cmake --build build/client
ctest --test-dir build/client --output-on-failure
```

The built client binary is written to `build/client/hdhomerun-linux-player`.

## Useful Runtime Overrides

- `HDHR_BACKEND_URL`: override the backend base URL. Default is `http://127.0.0.1:38080`.
- `HDHR_BACKEND_CMD`: override the backend executable path used when the client auto-starts the backend.
- `HDHR_CLIENT_EXIT_AFTER_MS`: short-lived smoke-test exit timer used by automated checks.

Current-show titles are supplied by the backend from SiliconDust guide data when the selected tuner exposes a valid `DeviceAuth` value.

## Notes

- packaged playback defaults to embedded in-window playback; no external `mpv` install is required for normal package use
- when a stale older backend is already bound to `127.0.0.1:38080`, the client now warns that guide support is unavailable from that backend
- the client now validates local auto-start URL overrides so a custom loopback URL and port can stay aligned with the backend bind address
- keyboard defaults now use `F` for fullscreen toggle, `Esc` to leave fullscreen, `Up` / `Down` for guide row navigation, `Left` / `Right` for playable channel switching, and `Page Up` / `Page Down` for volume
