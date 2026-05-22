# HDHomeRun Linux Player

HDHomeRun Linux Player is a native Linux desktop application for watching live TV from an HDHomeRun tuner.

It ships as a Qt/QML desktop client with a bundled local backend. The backend discovers devices, loads lineups, tracks playback state, and exposes tuner diagnostics. The client handles the on-screen playback experience with embedded Qt Multimedia playback.

## What You Get

- live TV playback for playable HDHomeRun channels
- automatic device discovery on the local network
- channel rail for quick switching
- tuner diagnostics in a side drawer
- fullscreen playback mode for a larger viewing surface
- keyboard shortcuts for fullscreen, channel surfing, and volume control
- visible volume buttons in both windowed and fullscreen playback modes
- packaged distribution for Debian, AppImage, and Flatpak

## Current Scope

- Linux desktop only
- live playback only
- no recording support
- DRM or otherwise restricted channels are shown but are not playable

## Package Options

Choose the package format that best matches your system:

- AppImage: portable, no install step required beyond making the file executable
- Debian package: best fit for Debian and Debian-derived distributions
- Flatpak: sandboxed install path that behaves consistently across supported desktops

Current package artifacts are generated under `dist/`:

- `dist/hdhomerun-linux-player_0.1.0_amd64.deb`
- `dist/HDHomeRunLinuxPlayer-x86_64.AppImage`
- `dist/HDHomeRunLinuxPlayer.flatpak`

## Quick Start

### AppImage

```sh
chmod +x dist/HDHomeRunLinuxPlayer-x86_64.AppImage
./dist/HDHomeRunLinuxPlayer-x86_64.AppImage
```

### Debian package

```sh
sudo apt install ./dist/hdhomerun-linux-player_0.1.0_amd64.deb
hdhomerun-linux-player
```

### Flatpak

```sh
flatpak install ./dist/HDHomeRunLinuxPlayer.flatpak
flatpak run io.github.e88z4.HDHomeRunLinuxPlayer
```

## First Launch

On first launch the app should:

- start its bundled local backend automatically
- discover reachable HDHomeRun devices on your network
- ask you to select a device if more than one tuner is available or no previous device is remembered
- load the channel lineup for the selected device
- start playback in the center stage when you choose a playable channel

## Keyboard Shortcuts

- `F`: toggle fullscreen mode
- `Esc`: exit fullscreen mode
- `Up`: volume up
- `Down`: volume down
- `Right`: next playable channel
- `Left`: previous playable channel

## Requirements

- an HDHomeRun tuner on the same local network as the Linux machine
- playable channels exposed by the HDHomeRun lineup
- working desktop audio and video support on the host system

No external `mpv` install is required for the packaged application.

## Troubleshooting

### No devices found

- make sure the Linux machine and the HDHomeRun device are on the same network
- confirm the tuner is powered on and reachable
- check whether local firewall rules are blocking discovery traffic

### Some channels are visible but do not play

- restricted or DRM-protected channels are listed but intentionally marked as unavailable
- verify that the channel is playable from the tuner lineup itself

### The app starts but playback has no picture or audio

- confirm the host has working multimedia support for Qt Multimedia and FFmpeg-backed playback
- try another known-good channel to rule out a lineup or signal issue
- open the diagnostics drawer and inspect tuner lock and signal details

## Advanced Runtime Overrides

These are mainly for troubleshooting and development:

- `HDHR_BACKEND_URL`: point the client at a manually managed backend URL instead of the default loopback address
- `HDHR_BACKEND_CMD`: override the backend executable path used by client auto-start
- `HDHR_BACKEND_BIND`: override the backend bind address when launching the backend manually

## Project Docs

- `client/README.md`: desktop client build and runtime notes
- `backend/README.md`: backend service notes and manual run options
- `packaging/README.md`: package-specific install and build details
- `aidlc-docs/`: project workflow and implementation records
