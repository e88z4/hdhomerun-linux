#!/usr/bin/env sh
set -eu

hdhr_resolve_backend_bin() {
    if [ -n "${HDHR_BACKEND_CMD:-}" ]; then
        printf '%s\n' "$HDHR_BACKEND_CMD"
        return 0
    fi

    if [ -n "${APPDIR:-}" ] && [ -x "$APPDIR/usr/bin/hdhomerun-backend" ]; then
        printf '%s\n' "$APPDIR/usr/bin/hdhomerun-backend"
        return 0
    fi

    if [ -x "/app/bin/hdhomerun-backend" ]; then
        printf '%s\n' "/app/bin/hdhomerun-backend"
        return 0
    fi

    if [ -n "${HDHR_RUNTIME_ENV_ROOT:-}" ]; then
        if [ -x "$HDHR_RUNTIME_ENV_ROOT/backend/target/debug/hdhomerun-backend" ]; then
            printf '%s\n' "$HDHR_RUNTIME_ENV_ROOT/backend/target/debug/hdhomerun-backend"
            return 0
        fi

        if [ -x "$HDHR_RUNTIME_ENV_ROOT/backend/target/release/hdhomerun-backend" ]; then
            printf '%s\n' "$HDHR_RUNTIME_ENV_ROOT/backend/target/release/hdhomerun-backend"
            return 0
        fi
    fi

    if command -v hdhomerun-backend >/dev/null 2>&1; then
        command -v hdhomerun-backend
        return 0
    fi

    return 1
}

hdhr_export_runtime_env() {
    if backend_bin="$(hdhr_resolve_backend_bin)"; then
        export HDHR_BACKEND_CMD="$backend_bin"
    else
        return 1
    fi

    export HDHR_BACKEND_PLAYER_MODE="${HDHR_BACKEND_PLAYER_MODE:-client}"
    return 0
}