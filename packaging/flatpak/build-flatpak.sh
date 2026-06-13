#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
MANIFEST="$ROOT_DIR/packaging/flatpak/io.github.e88z4.HDHomeRunLinuxPlayer.yaml"
FLATPAK_STAGE="$ROOT_DIR/dist/flatpak-root/app/bin"
BUILD_DIR="$ROOT_DIR/dist/flatpak-build"
REPO_DIR="$ROOT_DIR/dist/flatpak-repo"
BUNDLE_FILE="$ROOT_DIR/dist/HDHomeRunLinuxPlayer.flatpak"
FLATPAK_ARCH="${HDHR_FLATPAK_ARCH:-x86_64}"
BUNDLE_FILE_ARCH="$ROOT_DIR/dist/HDHomeRunLinuxPlayer-${FLATPAK_ARCH}.flatpak"
CLIENT_BIN="${HDHR_CLIENT_BIN:-$ROOT_DIR/build/client-release/hdhomerun-linux-player}"
BACKEND_BIN="${HDHR_BACKEND_BIN:-$ROOT_DIR/backend/target/release/hdhomerun-backend}"
HEADLESS_BIN="${HDHR_HEADLESS_BIN:-$ROOT_DIR/backend/target/release/hdhr-headless}"

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

if [ ! -x "$HEADLESS_BIN" ]; then
    printf 'Missing headless CLI binary: %s\n' "$HEADLESS_BIN" >&2
    exit 1
fi

rm -rf "$ROOT_DIR/dist/flatpak-root" "$BUILD_DIR" "$REPO_DIR"
mkdir -p "$FLATPAK_STAGE"

install -Dm755 "$CLIENT_BIN" "$FLATPAK_STAGE/hdhomerun-linux-player-real"
install -Dm755 "$BACKEND_BIN" "$FLATPAK_STAGE/hdhomerun-backend"
install -Dm755 "$HEADLESS_BIN" "$FLATPAK_STAGE/hdhr-headless"

flatpak-builder \
    --user \
    --force-clean \
    --default-branch=stable \
    --install-deps-from=flathub \
    --repo="$REPO_DIR" \
    "$BUILD_DIR" \
    "$MANIFEST"

flatpak build-bundle "$REPO_DIR" "$BUNDLE_FILE_ARCH" io.github.e88z4.HDHomeRunLinuxPlayer stable

# Keep a non-arch alias for local convenience and older docs/scripts.
cp -f "$BUNDLE_FILE_ARCH" "$BUNDLE_FILE"

printf 'Created Flatpak bundle: %s\n' "$BUNDLE_FILE_ARCH"
printf 'Created Flatpak bundle alias: %s\n' "$BUNDLE_FILE"