# Unit 13 — Headless CLI Binary Distribution Integration

## Problem

Unit 12 introduced the `hdhr-headless` CLI for operating headless streaming,
but packaging pipelines did not include the new binary consistently across
Debian, AppImage, and Flatpak outputs.

## Solution

Updated all binary distribution packaging flows so `hdhr-headless` ships beside
`hdhomerun-backend` and `hdhomerun-linux-player`.

## Changes

### Packaging Scripts

- `packaging/common/build-release-binaries.sh`
  - Explicitly builds `hdhr-headless` in release mode

- `packaging/debian/build-deb.sh`
  - Validates `HDHR_HEADLESS_BIN`
  - Installs `hdhr-headless` to `/usr/bin/hdhr-headless`

- `packaging/appimage/build-appimage.sh`
  - Validates `HDHR_HEADLESS_BIN`
  - Installs `hdhr-headless` to AppImage `usr/bin/hdhr-headless`

- `packaging/flatpak/build-flatpak.sh`
  - Validates `HDHR_HEADLESS_BIN`
  - Stages `hdhr-headless` in `dist/flatpak-root/app/bin/hdhr-headless`

- `packaging/flatpak/io.github.e88z4.HDHomeRunLinuxPlayer.yaml`
  - Installs staged `hdhr-headless` to `/app/bin/hdhr-headless`

### Documentation

- `packaging/README.md`
- `packaging/debian/README.md`
- `packaging/appimage/README.md`
- `packaging/flatpak/README.md`

Updated to state that packaged outputs include the headless helper CLI.

## Validation

- `sh packaging/common/build-release-binaries.sh`
- `HDHR_SKIP_BUILD=1 sh packaging/debian/build-deb.sh`
- `dpkg-deb -c dist/hdhomerun-linux-player_0.4.0_amd64.deb | grep hdhr-headless`
- `HDHR_SKIP_BUILD=1 sh packaging/appimage/build-appimage.sh`
- Verified `dist/appimage-root/HDHomeRunLinuxPlayer.AppDir/usr/bin/hdhr-headless`
- `HDHR_SKIP_BUILD=1 sh packaging/flatpak/build-flatpak.sh`
- `flatpak run --command=hdhr-headless io.github.e88z4.HDHomeRunLinuxPlayer help`

## Outcome

`hdhr-headless` is now part of the binary distribution artifacts and available in
all supported package formats.
