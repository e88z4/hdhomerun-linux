#!/usr/bin/env sh
set -eu

SCRIPT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)"
HDHR_RUNTIME_ENV_ROOT="$(CDPATH='' cd -- "$SCRIPT_DIR/../.." && pwd)"
export HDHR_RUNTIME_ENV_ROOT
. "$SCRIPT_DIR/export-runtime-env.sh"

if hdhr_export_runtime_env; then
    printf 'Resolved backend: %s\n' "$HDHR_BACKEND_CMD"
    printf 'Playback mode: %s\n' "$HDHR_BACKEND_PLAYER_MODE"
else
    printf 'Unable to resolve hdhomerun-backend. Install it or set HDHR_BACKEND_CMD.\n' >&2
    exit 1
fi

if [ "${1:-}" != "" ]; then
    if [ -x "$1" ]; then
        printf 'Found launcher: %s\n' "$1"
    else
        printf 'Launcher is missing or not executable: %s\n' "$1" >&2
        exit 1
    fi
fi