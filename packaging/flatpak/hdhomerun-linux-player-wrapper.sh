#!/usr/bin/env sh
set -eu

ENV_SCRIPT="/app/libexec/hdhomerun-linux-player/export-runtime-env.sh"
MAIN_BIN="/app/bin/hdhomerun-linux-player-real"

if [ ! -r "$ENV_SCRIPT" ]; then
    printf 'Flatpak runtime is missing %s\n' "$ENV_SCRIPT" >&2
    exit 1
fi

. "$ENV_SCRIPT"
hdhr_export_runtime_env || true

if [ ! -x "$MAIN_BIN" ]; then
    printf 'Flatpak runtime is missing the main launcher: %s\n' "$MAIN_BIN" >&2
    exit 1
fi

exec "$MAIN_BIN" "$@"