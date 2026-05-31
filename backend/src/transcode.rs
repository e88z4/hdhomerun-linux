use std::net::SocketAddr;
use std::process::Stdio;

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use tracing::{info, warn};

const FFMPEG_BIN_ENV: &str = "HDHR_BACKEND_FFMPEG_BIN";
const TRANSCODE_DECODER_ENV: &str = "HDHR_BACKEND_TRANSCODE_DECODER";
const TRANSCODE_ENCODER_ENV: &str = "HDHR_BACKEND_TRANSCODE_ENCODER";
const TRANSCODE_BITRATE_ENV: &str = "HDHR_BACKEND_TRANSCODE_BITRATE";

const DEFAULT_FFMPEG_BIN: &str = "ffmpeg";
const DEFAULT_BITRATE: &str = "4000k";

/// Returns true when at least one transcode env var is set to a non-empty value.
///
/// Operators enable transcoding by setting `HDHR_BACKEND_TRANSCODE_ENCODER` to
/// the desired FFmpeg encoder name, for example:
///
/// ```text
/// # Raspberry Pi 4 – V4L2 mem2mem hardware H.264 encoder
/// HDHR_BACKEND_TRANSCODE_ENCODER=h264_v4l2m2m
///
/// # Generic software fallback
/// HDHR_BACKEND_TRANSCODE_ENCODER=libx264
/// ```
///
/// Optionally set `HDHR_BACKEND_TRANSCODE_DECODER` to force a specific input
/// decoder (e.g. `mpeg2_v4l2m2m` for Pi 4 V4L2 hardware MPEG-2 decode).
pub fn transcode_enabled() -> bool {
    std::env::var(TRANSCODE_ENCODER_ENV)
        .map(|v| !v.trim().is_empty())
        .unwrap_or(false)
}

/// Returns the local proxy URL that the client should use as the playback URL
/// when transcoding is enabled.  Uses the same bind address as the backend
/// (read from `HDHR_BACKEND_BIND` or the default `127.0.0.1:38080`).
pub fn transcode_proxy_url() -> String {
    let bind: SocketAddr = std::env::var("HDHR_BACKEND_BIND")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 38080)));
    format!("http://{bind}/api/stream/transcode/live")
}

/// Spawns `ffmpeg` to transcode the given MPEG-TS `stream_url` and returns a
/// streaming HTTP response.  The response body is the transcoded MPEG-TS
/// stream piped from ffmpeg's stdout.
///
/// The ffmpeg child process is killed automatically when the HTTP connection
/// is closed (the client disconnects or stops the player), because dropping
/// the read end of the stdout pipe triggers SIGPIPE in ffmpeg.
///
/// Env var summary:
/// - `HDHR_BACKEND_FFMPEG_BIN`        – ffmpeg binary path (default: `ffmpeg`)
/// - `HDHR_BACKEND_TRANSCODE_DECODER` – input video decoder (optional, e.g. `mpeg2_v4l2m2m`)
/// - `HDHR_BACKEND_TRANSCODE_ENCODER` – output video encoder (required, e.g. `h264_v4l2m2m`)
/// - `HDHR_BACKEND_TRANSCODE_BITRATE` – output video bitrate (default: `4000k`)
pub async fn serve_transcoded_stream(stream_url: String) -> Response {
    let ffmpeg_bin =
        std::env::var(FFMPEG_BIN_ENV).unwrap_or_else(|_| DEFAULT_FFMPEG_BIN.to_string());
    let encoder = std::env::var(TRANSCODE_ENCODER_ENV).unwrap_or_default();
    let bitrate =
        std::env::var(TRANSCODE_BITRATE_ENV).unwrap_or_else(|_| DEFAULT_BITRATE.to_string());

    if encoder.trim().is_empty() {
        warn!("transcode requested but HDHR_BACKEND_TRANSCODE_ENCODER is not set");
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            "transcoding is not configured: set HDHR_BACKEND_TRANSCODE_ENCODER",
        )
            .into_response();
    }

    let mut cmd = Command::new(&ffmpeg_bin);
    cmd.arg("-hide_banner").arg("-loglevel").arg("error");

    // Optional hardware decoder (must come before -i)
    if let Ok(decoder) = std::env::var(TRANSCODE_DECODER_ENV) {
        if !decoder.trim().is_empty() {
            cmd.arg("-c:v").arg(decoder);
        }
    }

    cmd.arg("-i")
        .arg(&stream_url)
        .arg("-c:v")
        .arg(&encoder)
        .arg("-b:v")
        .arg(&bitrate)
        .arg("-bufsize")
        .arg({
            // bufsize = 2 × bitrate (rough rule for hardware encoders)
            let kbps: u32 = bitrate
                .trim_end_matches('k')
                .trim_end_matches('K')
                .parse()
                .unwrap_or(4000);
            format!("{}k", kbps * 2)
        })
        .arg("-c:a")
        .arg("copy")
        .arg("-f")
        .arg("mpegts")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);

    info!(
        stream_url = %stream_url,
        encoder = %encoder,
        bitrate = %bitrate,
        "starting live transcode"
    );

    match cmd.spawn() {
        Ok(mut child) => {
            let stdout = child
                .stdout
                .take()
                .expect("stdout was configured as piped");

            // Wait for process exit in a detached task so we can log the outcome.
            // stdout is moved into the ReaderStream, so when the stream is dropped
            // (client disconnect) the read-end of the pipe is closed and ffmpeg
            // receives SIGPIPE, causing it to exit and unblocking this wait task.
            tokio::spawn(async move {
                match child.wait().await {
                    Ok(status) if !status.success() => {
                        warn!(?status, "transcode ffmpeg process exited with non-zero status");
                    }
                    Err(e) => warn!(error = %e, "error waiting for transcode ffmpeg process"),
                    _ => {}
                }
            });

            Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "video/MP2T")
                .header("Cache-Control", "no-cache, no-store")
                .body(Body::from_stream(ReaderStream::new(stdout)))
                .expect("valid response builder")
        }
        Err(e) => {
            warn!(error = %e, ffmpeg_bin = %ffmpeg_bin, "failed to spawn ffmpeg for transcoding");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("failed to start transcoder: {e}"),
            )
                .into_response()
        }
    }
}
