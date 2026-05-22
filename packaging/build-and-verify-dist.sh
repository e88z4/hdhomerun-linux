#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"
DEV_BUILD_DIR="${HDHR_DEV_BUILD_DIR:-$ROOT_DIR/build/client}"
DEBIAN_VERIFY_DIR="$ROOT_DIR/dist/debian-verify"

cmake -S "$ROOT_DIR/client" -B "$DEV_BUILD_DIR" -G Ninja
cmake --build "$DEV_BUILD_DIR"
ctest --test-dir "$DEV_BUILD_DIR" --output-on-failure

cargo test --manifest-path "$ROOT_DIR/backend/Cargo.toml" --quiet
sh "$ROOT_DIR/packaging/common/build-release-binaries.sh"

HDHR_SKIP_BUILD=1 sh "$ROOT_DIR/packaging/debian/build-deb.sh"
HDHR_SKIP_BUILD=1 sh "$ROOT_DIR/packaging/appimage/build-appimage.sh"
HDHR_SKIP_BUILD=1 sh "$ROOT_DIR/packaging/flatpak/build-flatpak.sh"

rm -rf "$DEBIAN_VERIFY_DIR"
mkdir -p "$DEBIAN_VERIFY_DIR"
dpkg-deb -x "$ROOT_DIR/dist/hdhomerun-linux-player_0.1.0_amd64.deb" "$DEBIAN_VERIFY_DIR"

env PATH="$DEBIAN_VERIFY_DIR/usr/bin:$PATH" \
    QT_QPA_PLATFORM=offscreen \
    HDHR_CLIENT_EXIT_AFTER_MS=750 \
    "$DEBIAN_VERIFY_DIR/usr/bin/hdhomerun-linux-player" >/dev/null 2>&1

env APPIMAGE_EXTRACT_AND_RUN=1 \
    QT_QPA_PLATFORM=offscreen \
    HDHR_CLIENT_EXIT_AFTER_MS=750 \
    "$ROOT_DIR/dist/HDHomeRunLinuxPlayer-x86_64.AppImage" >/dev/null 2>&1

flatpak install --user --noninteractive --reinstall "$ROOT_DIR/dist/HDHomeRunLinuxPlayer.flatpak" >/dev/null
timeout 20s flatpak run --user \
    --env=QT_QPA_PLATFORM=offscreen \
    --env=HDHR_CLIENT_EXIT_AFTER_MS=750 \
    io.github.e88z4.HDHomeRunLinuxPlayer >/dev/null 2>&1
flatpak uninstall --user --noninteractive io.github.e88z4.HDHomeRunLinuxPlayer >/dev/null 2>&1 || true

printf 'Distribution build and verification completed successfully.\n'