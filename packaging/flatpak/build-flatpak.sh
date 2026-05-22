#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
MANIFEST="$ROOT_DIR/packaging/flatpak/io.github.e88z4.HDHomeRunLinuxPlayer.yaml"
FLATPAK_STAGE="$ROOT_DIR/dist/flatpak-root/app/bin"
BUILD_DIR="$ROOT_DIR/dist/flatpak-build"
REPO_DIR="$ROOT_DIR/dist/flatpak-repo"
BUNDLE_FILE="$ROOT_DIR/dist/HDHomeRunLinuxPlayer.flatpak"
CLIENT_BIN="${HDHR_CLIENT_BIN:-$ROOT_DIR/build/client-release/hdhomerun-linux-player}"
BACKEND_BIN="${HDHR_BACKEND_BIN:-$ROOT_DIR/backend/target/release/hdhomerun-backend}"

if [ ! -x "$CLIENT_BIN" ]; then
    printf 'Missing client binary: %s\n' "$CLIENT_BIN" >&2
    exit 1
fi

if [ ! -x "$BACKEND_BIN" ]; then
    printf 'Missing backend binary: %s\n' "$BACKEND_BIN" >&2
    exit 1
fi

rm -rf "$ROOT_DIR/dist/flatpak-root" "$BUILD_DIR" "$REPO_DIR"
mkdir -p "$FLATPAK_STAGE"

install -Dm755 "$CLIENT_BIN" "$FLATPAK_STAGE/hdhomerun-linux-player-real"
install -Dm755 "$BACKEND_BIN" "$FLATPAK_STAGE/hdhomerun-backend"

flatpak-builder \
    --user \
    --force-clean \
    --default-branch=stable \
    --install-deps-from=flathub \
    --repo="$REPO_DIR" \
    "$BUILD_DIR" \
    "$MANIFEST"

flatpak build-bundle "$REPO_DIR" "$BUNDLE_FILE" io.github.e88z4.HDHomeRunLinuxPlayer stable
printf 'Created Flatpak bundle: %s\n' "$BUNDLE_FILE"