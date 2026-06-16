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
- `HDHR_XMLTV_SOURCE`: optional XMLTV file path or HTTP(S) URL used to enrich `/api/lineup` with current-program titles

## Headless Remote Transcoded Streaming

The backend can run in headless mode and expose a low-bandwidth transcoded MPEG-TS stream for remote clients.

Recommended environment for remote streaming:

```bash
export HDHR_BACKEND_BIND=0.0.0.0:38080
export HDHR_BACKEND_PLAYER_MODE=client
export HDHR_BACKEND_TRANSCODE_ENCODER=libx264
export HDHR_BACKEND_TRANSCODE_PROFILE=low
cargo run
```

Headless stream endpoint:

- `GET /api/stream/transcode/live?deviceRef=<deviceRef>&channelRef=<channelRef>`

VLC-friendly playlist endpoint:

- `GET /api/stream/transcode/playlist.m3u?deviceRef=<deviceRef>`

The playlist endpoint publishes channel entries with quality variants (`very_low`, `low`, `balanced`, `high`) so VLC can switch channels/quality directly from one imported M3U.

Optional playlist query params:

- `deviceRef`: explicit source device; if omitted, the remembered selected device is used
- `publicBase`: optional absolute base URL (for example `http://192.168.1.10:38080`) to emit absolute stream URLs

The endpoint accepts either a channel ref (for example `channel:5.1`) or guide number (for example `5.1`) in `channelRef`.

Low-bandwidth tuning query params:

- `profile`: `very_low`, `low`, `balanced`, `high`
- `videoBitrate`: FFmpeg bitrate value such as `800k`
- `audioBitrate`: FFmpeg bitrate value such as `96k`
- `maxHeight`: maximum output height, e.g. `480`
- `fps`: output frame rate cap, e.g. `24`

Example remote URL:

```text
http://<backend-host>:38080/api/stream/transcode/live?deviceRef=hdhr-1234abcd&channelRef=5.1&profile=low
```

Example playlist URL for VLC:

```text
http://<backend-host>:38080/api/stream/transcode/playlist.m3u?deviceRef=hdhr-1234abcd
```

Environment-based defaults are also supported:

- `HDHR_BACKEND_TRANSCODE_PROFILE`
- `HDHR_BACKEND_TRANSCODE_BITRATE`
- `HDHR_BACKEND_TRANSCODE_AUDIO_BITRATE`
- `HDHR_BACKEND_TRANSCODE_MAX_HEIGHT`
- `HDHR_BACKEND_TRANSCODE_FPS`

Dynamic tuner permit controls:

- `HDHR_BACKEND_TUNER_RESERVE_RECORDING`: reserve this many tuners for recording-priority capacity (default `0`)
- `HDHR_BACKEND_FRONTEND_MIN_FREE`: when no frontend live session is active, keep at least this many tuners available for frontend playback by limiting new headless streams (default `1`)
- `HDHR_BACKEND_HEADLESS_MAX_PER_DEVICE`: optional hard cap for headless transcode streams per device

Concurrency behavior:

- frontend live playback and headless transcoded streaming can run concurrently on the same machine
- permits are tracked per device using each device's discovered `tuner_count`
- headless streams are denied with `503` when capacity is exhausted or when frontend-reserved headroom must be preserved

## Headless CLI

The backend now ships a dedicated CLI binary that wraps common API calls and
service management tasks for headless streaming.

Build:

```bash
cargo build --release --bin hdhr-headless
```

Binary path:

```text
target/release/hdhr-headless
```

### Common Commands

List devices:

```bash
target/release/hdhr-headless devices --backend-url http://127.0.0.1:39090
```

List channels:

```bash
target/release/hdhr-headless channels --backend-url http://127.0.0.1:39090
```

Generate a player URL (no manual curl query building):

```bash
target/release/hdhr-headless stream-url \
  --device-ref hdhr-10ab47d5 \
  --channel-ref 29.1 \
  --profile low \
  --public-base http://192.168.1.10:39090
```

### Run as a User Service (systemd)

Install + enable + start headless service:

```bash
target/release/hdhr-headless service install \
  --bind 0.0.0.0:39090 \
  --encoder libx264 \
  --profile low \
  --backend-bin /absolute/path/to/hdhomerun-backend
```

Service lifecycle:

```bash
target/release/hdhr-headless service status
target/release/hdhr-headless service logs
target/release/hdhr-headless service restart
target/release/hdhr-headless service stop
target/release/hdhr-headless service uninstall
```

## Test

```bash
cargo test
```
