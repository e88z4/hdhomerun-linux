# Packaging and Installation

This directory contains the packaging assets and package-specific notes for HDHomeRun Linux Player.

The packaged application always follows the same high-level model:

- launch the desktop client first
- use the bundled backend automatically
- default to embedded in-window playback through Qt Multimedia

## Package Formats

### AppImage

Best when you want a portable app file with no system-level install step.

Use:

```sh
chmod +x dist/HDHomeRunLinuxPlayer-x86_64.AppImage
./dist/HDHomeRunLinuxPlayer-x86_64.AppImage
```

### Debian package

Best on Debian and Debian-derived distributions.

Use:

```sh
sudo apt install ./dist/hdhomerun-linux-player_0.1.0_amd64.deb
hdhomerun-linux-player
```

### Flatpak

Best when you prefer sandboxed desktop applications.

Use:

```sh
flatpak install ./dist/HDHomeRunLinuxPlayer.flatpak
flatpak run io.github.e88z4.HDHomeRunLinuxPlayer
```

## Package Outputs

The generated package artifacts are:

- `dist/hdhomerun-linux-player_0.1.0_amd64.deb`
- `dist/HDHomeRunLinuxPlayer-x86_64.AppImage`
- `dist/HDHomeRunLinuxPlayer.flatpak`

## Build Commands

Preferred end-to-end distribution pipeline:

```sh
./packaging/build-and-verify-dist.sh
```

This command:

- rebuilds the dev client used during local iteration
- reruns the client and backend tests
- rebuilds the release client and backend
- rebuilds Debian, AppImage, and Flatpak artifacts
- verifies packaged startup for Debian, AppImage, and Flatpak

Lower-level packaging commands:

```sh
cargo build --manifest-path backend/Cargo.toml --release
cmake -S client -B build/client-release -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build/client-release
./packaging/debian/build-deb.sh
./packaging/appimage/build-appimage.sh
./packaging/flatpak/build-flatpak.sh
```

## Additional Notes

- no packaged format requires a separate host `mpv` install for normal use
- AppImage and Flatpak bundle both the client and backend
- Debian installs both binaries and depends on the host Qt runtime packages
- the Flatpak runtime currently targets KDE `6.10`

## Format-Specific Notes

- `packaging/appimage/README.md`
- `packaging/debian/README.md`
- `packaging/flatpak/README.md`
