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

    # On ARM64 (e.g. Raspberry Pi 4) enable the backend transcode proxy by
    # default so that live TV is re-encoded to H.264 via V4L2 M2M hardware.
    # This eliminates software MPEG-2 decode overhead in windowed mode.
    # Users can override any of these by setting them before launching.
    if [ "$(uname -m)" = "aarch64" ]; then
        export HDHR_BACKEND_TRANSCODE_ENCODER="${HDHR_BACKEND_TRANSCODE_ENCODER:-h264_v4l2m2m}"
        export HDHR_BACKEND_TRANSCODE_DECODER="${HDHR_BACKEND_TRANSCODE_DECODER:-mpeg2_v4l2m2m}"
    fi

    return 0
}