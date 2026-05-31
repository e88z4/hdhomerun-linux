# Unit 10 — Performance Optimization: Backend Live TV Transcode Proxy

## Problem

On a Raspberry Pi 4 (8 GB, Debian trixie, kernel 6.12+rpt-rpi-v8), windowed
live TV playback was choppy while fullscreen was smooth.  Root cause: HDHomeRun
streams live TV as MPEG-2 Video in an MPEG-TS container.  Qt Multimedia's
GStreamer backend performs MPEG-2 decode in software.  On the Pi 4, software
MPEG-2 decode combined with the Wayland compositor's texture-upload overhead
leaves insufficient headroom for smooth windowed rendering.

The Pi 4 has dedicated V4L2 M2M hardware for both MPEG-2 decode and H.264
encode via the `bcm2835-codec` kernel module (`/dev/video10`–`/dev/video23`).
`ffmpeg` 7.1 on the Pi exposes this via the `mpeg2_v4l2m2m` decoder and
`h264_v4l2m2m` encoder.  H.264 is natively hardware-decoded by GStreamer on
the Pi with negligible CPU impact.

## Solution

A backend transcoding proxy route that re-encodes the upstream MPEG-2 TS to
H.264 TS before the Qt client ever sees it.

When `HDHR_BACKEND_TRANSCODE_ENCODER` is set, the backend rewrites the
`playback_url` in the live session state to the local proxy URL
(`http://127.0.0.1:38080/api/stream/transcode/live`).  The Qt client connects
to that URL as normal and receives an H.264 MPEG-TS stream; it never sees or
cares about the change.

The proxy route spawns `ffmpeg` with the configured decoder/encoder, pipes its
stdout as a streaming HTTP response body (`video/MP2T`), and kills the ffmpeg
process automatically when the client disconnects (via `kill_on_drop(true)`).

DVR recorded playback is **not** transcoded — the DVR stream URL is passed
directly to the client / mpv as before.

## Environment Variables

| Variable | Default | Description |
|---|---|---|
| `HDHR_BACKEND_TRANSCODE_ENCODER` | *(unset)* | **Required to enable.** Encoder name, e.g. `h264_v4l2m2m` |
| `HDHR_BACKEND_TRANSCODE_DECODER` | *(none)* | Optional decoder, e.g. `mpeg2_v4l2m2m` |
| `HDHR_BACKEND_TRANSCODE_BITRATE` | `4000k` | Target video bitrate |
| `HDHR_BACKEND_FFMPEG_BIN` | `ffmpeg` | Path to ffmpeg binary |

## Files Changed

### New

- `backend/src/transcode.rs` — transcode proxy module
  - `transcode_enabled() -> bool`
  - `transcode_proxy_url() -> String`
  - `serve_transcoded_stream(stream_url: String) -> Response` — spawns ffmpeg,
    pipes stdout as streaming HTTP response, kills on disconnect

### Modified

- `backend/Cargo.toml`
  - Added `process` feature to tokio dependency
  - Added `tokio-util = { version = "0.7", features = ["io"] }` (for `ReaderStream`)

- `backend/src/lib.rs`
  - Added `pub mod transcode;`

- `backend/src/playback.rs`
  - Added `raw_stream_url: Option<String>` to `PlaybackRuntime`
  - Added `raw_stream_url: None` to `Default for PlaybackRuntime`
  - Added `PlaybackService::raw_stream_url() -> Option<String>` accessor
  - `run_live_playback_command()`: when transcode is enabled, rewrites
    `playback_url` to the proxy URL and stores the real upstream URL in
    `runtime.raw_stream_url`
  - `stop()`: clears `runtime.raw_stream_url = None`

- `backend/src/http/routes.rs`
  - Added route: `GET /api/stream/transcode/live`
  - Added handler `stream_transcode_live` — retrieves the raw stream URL from
    the active live session and delegates to `transcode::serve_transcoded_stream`

### New (tooling)

- `packaging/deploy-backend-to-pi.sh` — quick deploy script: rsyncs backend
  source to a remote host, builds in place, installs binary to `~/bin/`

## Usage (Raspberry Pi 4)

```sh
export HDHR_BACKEND_TRANSCODE_ENCODER=h264_v4l2m2m
export HDHR_BACKEND_TRANSCODE_DECODER=mpeg2_v4l2m2m
# optional:
# export HDHR_BACKEND_TRANSCODE_BITRATE=4000k
hdhomerun-linux-player
```

Alternatively, quick deploy from the development machine:

```sh
sh packaging/deploy-backend-to-pi.sh felix@karaoke
```
