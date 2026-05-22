# Packaging Strategy

This directory now targets the embedded-playback architecture established in Unit 4. The packaged application should launch the Qt client first, point it at the packaged backend binary, and run the backend in client-managed playback mode so the center stage renders in-window through Qt Multimedia.

## Chosen Runtime Strategy

The packaged runtime should be consistent across formats:

- **AppImage**: ship the Qt client and backend inside the AppDir and export a packaged backend path through the launcher.
- **Flatpak**: ship the Qt client and backend inside the sandbox and export the backend path from the Flatpak wrapper.
- **Debian**: install both binaries and rely on distro Qt Multimedia and QML runtime packages instead of an external `mpv` dependency.

External `mpv` remains a fallback backend mode for development and diagnostics, but it is no longer the primary packaged playback path.

## Current Scope

These assets now focus on:

- runtime environment resolution for the packaged backend path and embedded-playback mode
- per-format launcher conventions for client-first startup
- smoke-test guidance for packaged client and backend binaries
- host prerequisite checks for AppImage, Flatpak, Debian tooling, and client build tooling

## Directory Layout

- `common/`: shared runtime helpers used across package formats
- `appimage/`: AppImage launcher conventions and notes
- `flatpak/`: Flatpak manifest and wrapper conventions
- `debian/`: Debian package metadata skeleton

## Useful Checks

- `./packaging/common/smoke-test-runtime.sh`
- `./packaging/common/check-host-dependencies.sh`

## Current Artifact Commands

- `cargo build --manifest-path backend/Cargo.toml --release`
- `cmake -S client -B build/client-release -G Ninja -DCMAKE_BUILD_TYPE=Release`
- `cmake --build build/client-release`
- `./packaging/debian/build-deb.sh`
- `./packaging/appimage/build-appimage.sh`

## Current Outputs

- `dist/hdhomerun-linux-player_0.1.0_amd64.deb`
- `dist/HDHomeRunLinuxPlayer-x86_64.AppImage`
- `dist/HDHomeRunLinuxPlayer.flatpak`

## Current Risk

- The Flatpak manifest currently targets KDE runtime `6.8`, which flatpak-builder reports as end-of-life. The bundle is generated successfully, but the runtime version should be upgraded before treating the Flatpak packaging path as production-ready.
