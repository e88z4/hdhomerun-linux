#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
APPDIR="$ROOT_DIR/dist/appimage-root/HDHomeRunLinuxPlayer.AppDir"
OUTPUT_APPIMAGE="$ROOT_DIR/dist/HDHomeRunLinuxPlayer-x86_64.AppImage"
CLIENT_BIN="${HDHR_CLIENT_BIN:-$ROOT_DIR/build/client-release/hdhomerun-linux-player}"
BACKEND_BIN="${HDHR_BACKEND_BIN:-$ROOT_DIR/backend/target/release/hdhomerun-backend}"
APPIMAGETOOL_BIN="${APPIMAGETOOL_BIN:-appimagetool}"

if [ "${HDHR_SKIP_BUILD:-0}" != "1" ]; then
    sh "$ROOT_DIR/packaging/common/build-release-binaries.sh"
fi

if [ ! -x "$CLIENT_BIN" ]; then
    printf 'Missing client binary: %s\n' "$CLIENT_BIN" >&2
    exit 1
fi

if [ ! -x "$BACKEND_BIN" ]; then
    printf 'Missing backend binary: %s\n' "$BACKEND_BIN" >&2
    exit 1
fi

rm -rf "$APPDIR"
mkdir -p \
    "$APPDIR/usr/bin" \
    "$APPDIR/usr/lib/hdhomerun-linux-player" \
    "$APPDIR/usr/share/applications" \
    "$APPDIR/usr/share/metainfo" \
    "$APPDIR/usr/share/icons/hicolor/scalable/apps"

install -Dm755 "$ROOT_DIR/packaging/appimage/AppRun" "$APPDIR/AppRun"
install -Dm755 "$CLIENT_BIN" "$APPDIR/usr/bin/hdhomerun-linux-player"
install -Dm755 "$BACKEND_BIN" "$APPDIR/usr/bin/hdhomerun-backend"
install -Dm644 "$ROOT_DIR/packaging/common/hdhomerun-linux-player.desktop" "$APPDIR/io.github.e88z4.HDHomeRunLinuxPlayer.desktop"
install -Dm644 "$ROOT_DIR/packaging/common/hdhomerun-linux-player.desktop" "$APPDIR/usr/share/applications/io.github.e88z4.HDHomeRunLinuxPlayer.desktop"
install -Dm644 "$ROOT_DIR/packaging/common/io.github.e88z4.HDHomeRunLinuxPlayer.svg" "$APPDIR/io.github.e88z4.HDHomeRunLinuxPlayer.svg"
install -Dm644 "$ROOT_DIR/packaging/common/io.github.e88z4.HDHomeRunLinuxPlayer.svg" "$APPDIR/usr/share/icons/hicolor/scalable/apps/io.github.e88z4.HDHomeRunLinuxPlayer.svg"
install -Dm644 "$ROOT_DIR/packaging/common/io.github.e88z4.HDHomeRunLinuxPlayer.appdata.xml" "$APPDIR/usr/share/metainfo/io.github.e88z4.HDHomeRunLinuxPlayer.appdata.xml"
install -Dm755 "$ROOT_DIR/packaging/common/export-runtime-env.sh" "$APPDIR/usr/lib/hdhomerun-linux-player/export-runtime-env.sh"

ARCH=x86_64 "$APPIMAGETOOL_BIN" "$APPDIR" "$OUTPUT_APPIMAGE"
printf 'Created AppImage: %s\n' "$OUTPUT_APPIMAGE"