#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
STAGE_DIR="$ROOT_DIR/dist/debian-root"
OUTPUT_DIR="$ROOT_DIR/dist"
PACKAGE_NAME="hdhomerun-linux-player"
VERSION="${HDHR_PACKAGE_VERSION:-0.1.0}"
ARCHITECTURE="${HDHR_PACKAGE_ARCH:-$(dpkg --print-architecture)}"
CLIENT_BIN="${HDHR_CLIENT_BIN:-$ROOT_DIR/build/client-release/hdhomerun-linux-player}"
BACKEND_BIN="${HDHR_BACKEND_BIN:-$ROOT_DIR/backend/target/release/hdhomerun-backend}"
PACKAGE_FILE="$OUTPUT_DIR/${PACKAGE_NAME}_${VERSION}_${ARCHITECTURE}.deb"

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

rm -rf "$STAGE_DIR"
mkdir -p \
    "$STAGE_DIR/DEBIAN" \
    "$STAGE_DIR/usr/bin" \
    "$STAGE_DIR/usr/lib/hdhomerun-linux-player" \
    "$STAGE_DIR/usr/share/applications" \
    "$STAGE_DIR/usr/share/metainfo" \
    "$STAGE_DIR/usr/share/icons/hicolor/scalable/apps"

cat > "$STAGE_DIR/DEBIAN/control" <<EOF
Package: $PACKAGE_NAME
Version: $VERSION
Section: video
Priority: optional
Architecture: $ARCHITECTURE
Maintainer: Felix <felix@example.invalid>
Depends: qml6-module-qtmultimedia, qml6-module-qtquick-controls, qml6-module-qtquick-layouts
Description: HDHomeRun live TV player for Linux
 Native Linux HDHomeRun player composed of a local backend service and a desktop UI.
 The packaged runtime defaults to embedded playback in the Qt client while the backend runs headless.
EOF

install -Dm755 "$CLIENT_BIN" "$STAGE_DIR/usr/bin/hdhomerun-linux-player"
install -Dm755 "$BACKEND_BIN" "$STAGE_DIR/usr/bin/hdhomerun-backend"
install -Dm755 "$ROOT_DIR/packaging/common/export-runtime-env.sh" "$STAGE_DIR/usr/lib/hdhomerun-linux-player/export-runtime-env.sh"
install -Dm644 "$ROOT_DIR/packaging/common/hdhomerun-linux-player.desktop" "$STAGE_DIR/usr/share/applications/io.github.e88z4.HDHomeRunLinuxPlayer.desktop"
install -Dm644 "$ROOT_DIR/packaging/common/io.github.e88z4.HDHomeRunLinuxPlayer.svg" "$STAGE_DIR/usr/share/icons/hicolor/scalable/apps/io.github.e88z4.HDHomeRunLinuxPlayer.svg"
install -Dm644 "$ROOT_DIR/packaging/common/io.github.e88z4.HDHomeRunLinuxPlayer.appdata.xml" "$STAGE_DIR/usr/share/metainfo/io.github.e88z4.HDHomeRunLinuxPlayer.appdata.xml"

dpkg-deb --root-owner-group --build "$STAGE_DIR" "$PACKAGE_FILE"
printf 'Created Debian package: %s\n' "$PACKAGE_FILE"