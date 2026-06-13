# Unit 12 — Headless CLI and Service Workflow

## Problem

Headless streaming was functional but operationally cumbersome. Users had to
manually compose multiple curl requests for device selection, lineup discovery,
and stream URL generation. Running the backend as a persistent service also
required manual systemd setup.

## Solution

Added a dedicated CLI binary: `hdhr-headless`.

The CLI wraps key backend APIs and provides user-service management for
headless transcode streaming.

## Delivered Capabilities

- Device discovery listing (`devices`)
- Device selection (`select-device`)
- Channel lineup listing (`channels`)
- Headless transcode URL builder (`stream-url`)
- systemd user-service lifecycle (`service install/start/stop/restart/status/logs/uninstall`)

## Service Workflow

`service install` writes a user unit under:

- `~/.config/systemd/user/hdhomerun-headless.service`

Environment defaults in installed unit:

- `HDHR_BACKEND_BIND=0.0.0.0:39090`
- `HDHR_BACKEND_PLAYER_MODE=client`
- `HDHR_BACKEND_TRANSCODE_ENCODER=libx264`
- `HDHR_BACKEND_TRANSCODE_PROFILE=low`

Then runs:

- `systemctl --user daemon-reload`
- `systemctl --user enable --now hdhomerun-headless.service`

## Files Changed

- `backend/src/bin/hdhr-headless.rs`
  - New CLI binary with API wrappers and systemd user-service orchestration

- `backend/README.md`
  - Added CLI usage and service lifecycle documentation

- `aidlc-docs/aidlc-state.md`
  - Advanced stage marker to Unit 12 complete

## Validation

- `cargo build --release --bin hdhr-headless`
- `./target/release/hdhr-headless help`
- `./target/release/hdhr-headless stream-url --device-ref ... --channel-ref ...`
- `./target/release/hdhr-headless channels --backend-url http://127.0.0.1:39090`
