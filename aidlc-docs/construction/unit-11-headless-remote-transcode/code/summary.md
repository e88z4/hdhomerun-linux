# Unit 11 — Headless Remote Transcoded Streaming

## Problem

Unit 10 introduced a local transcode proxy tied to an active playback session,
which worked well for embedded client playback. It did not provide a direct
headless path for remote clients that need low-bandwidth streaming from an
HDHomeRun source.

## Solution

Extended `GET /api/stream/transcode/live` to support a headless source mode.
When `channelRef` is provided, the backend resolves the source URL directly
from lineup data (and optional `deviceRef`) without requiring an active local
playback session.

Added bandwidth-oriented transcode controls with profile presets and explicit
query/environment overrides.

## API Behavior

### Session Mode (existing)

- Request: `GET /api/stream/transcode/live`
- Source: currently active playback session raw stream URL
- No active session: returns `404`

### Headless Mode (new)

- Request: `GET /api/stream/transcode/live?deviceRef=<deviceRef>&channelRef=<channelRef>`
- Source: lineup-resolved HDHomeRun playback URL
- `channelRef` accepts either channel ref (for example `channel:5.1`) or guide number (for example `5.1`)

### Optional Query Tuning

- `profile`: `very_low`, `low`, `balanced`, `high`
- `videoBitrate`: FFmpeg-style video bitrate, e.g. `900k`
- `audioBitrate`: FFmpeg-style audio bitrate, e.g. `96k`
- `maxHeight`: output height cap
- `fps`: output frame-rate cap

## Transcode Profiles

- `very_low`: `500k` video, `64k` audio, max `360p`, `20 fps`
- `low`: `900k` video, `96k` audio, max `480p`, `24 fps`
- `balanced`: `1500k` video, `128k` audio, max `720p`, `30 fps`
- `high`: `3000k` video, `160k` audio, max `1080p`, `30 fps`
- `legacy` (default): preserves Unit 10 behavior using `HDHR_BACKEND_TRANSCODE_BITRATE` and optional env overrides

## Files Changed

- `backend/src/http/routes.rs`
  - Added `StreamTranscodeQuery`
  - Extended `/api/stream/transcode/live` for headless source selection + tuning params
  - Added `resolve_transcode_source_url` helper
  - Added `streamTranscodeLive` to bootstrap contract endpoint listing

- `backend/src/transcode.rs`
  - Added `TranscodeRequestOptions`
  - Added `serve_transcoded_stream_with_options`
  - Added profile-based effective settings resolution
  - Added optional video filter chain (`scale` + `fps`)
  - Added optional AAC audio transcode for bandwidth-constrained profiles

- `backend/tests/playback_contract.rs`
  - Added endpoint contract tests for transcode session mode (`404`) and headless mode (`503` when encoder is unset)

- `backend/README.md`
  - Documented headless remote transcode usage and low-bandwidth tuning

## Validation

- `cargo test --test playback_contract`
- `cargo test transcode::tests`
