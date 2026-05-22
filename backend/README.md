# Backend Foundation

This directory contains the local backend service used by HDHomeRun Linux Player.

## Current Scope

- loopback HTTP API for health, bootstrap, devices, lineup, tuner diagnostics, and playback state
- canonical remembered-state persistence in the XDG state directory
- bundled backend process used by the desktop client and packaged formats
- playback orchestration for client-managed embedded playback and development fallback modes

## Normal Usage

End users typically do not start the backend directly. The desktop client launches it automatically.

Manual startup is mainly useful for debugging or development.

## Run

```bash
cargo run
```

The backend binds to `127.0.0.1:38080` by default.

Useful overrides:

- `HDHR_BACKEND_BIND`: override the bind address, for example `127.0.0.1:39090`
- `HDHR_BACKEND_PLAYER_MODE`: choose playback adapter mode; packaged runs default to `client`

## Test

```bash
cargo test
```
