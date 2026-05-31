#!/usr/bin/env sh
# Quick backend-only deploy to a remote Pi (or any host with Rust + ffmpeg).
# Rsyncs backend source, builds on the remote, and restarts the service.
#
# Usage:
#   sh packaging/deploy-backend-to-pi.sh [user@host]
#
# Examples:
#   sh packaging/deploy-backend-to-pi.sh felix@karaoke
#   HDHR_DEPLOY_HOST=felix@karaoke sh packaging/deploy-backend-to-pi.sh

set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/.." && pwd)"

DEPLOY_HOST="${1:-${HDHR_DEPLOY_HOST:-}}"
if [ -z "$DEPLOY_HOST" ]; then
    printf 'Usage: %s user@host\n' "$0" >&2
    exit 1
fi

REMOTE_DIR="${HDHR_DEPLOY_REMOTE_DIR:-~/hdhomerun-linux-backend}"

printf '==> Syncing backend source to %s:%s\n' "$DEPLOY_HOST" "$REMOTE_DIR"
rsync -az --delete \
    --exclude='target/' \
    "$ROOT_DIR/backend/" \
    "$DEPLOY_HOST:$REMOTE_DIR/"

printf '==> Building on remote (this takes a minute on a Pi)\n'
# shellcheck disable=SC2029
ssh "$DEPLOY_HOST" "cd '$REMOTE_DIR' && cargo build --release 2>&1"

printf '==> Restarting hdhomerun-backend on remote\n'
# Kill any running hdhomerun-backend processes and start the new one in the
# background.  Adjust this block if you use systemd or another supervisor.
# shellcheck disable=SC2029
ssh "$DEPLOY_HOST" sh <<'REMOTE'
set -eu
pkill -x hdhomerun-backend 2>/dev/null || true
sleep 1
# Install to ~/bin so it is on PATH
mkdir -p ~/bin
cp ~/hdhomerun-linux-backend/target/release/hdhomerun-backend ~/bin/hdhomerun-backend
printf 'Installed: %s\n' "$(~/bin/hdhomerun-backend --version 2>&1 || echo '(no --version flag)')"
printf 'Done. Launch the app normally to start the backend.\n'
REMOTE

printf '==> Deploy complete.\n'
printf '\n'
printf 'To enable H.264 transcoding on the Pi, set these env vars before\n'
printf 'launching hdhomerun-linux-player:\n'
printf '\n'
printf '  export HDHR_BACKEND_TRANSCODE_ENCODER=h264_v4l2m2m\n'
printf '  export HDHR_BACKEND_TRANSCODE_DECODER=mpeg2_v4l2m2m\n'
printf '  export HDHR_BACKEND_TRANSCODE_BITRATE=4000k   # optional\n'
printf '\n'
printf 'Or add those lines to ~/.config/hdhomerun-linux-player/env\n'
printf 'if your packaging wrapper sources that file.\n'
