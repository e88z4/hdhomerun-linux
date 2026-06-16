use std::net::SocketAddr;
use std::process::Stdio;

use axum::body::Body;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use tracing::{info, warn};

use crate::tuner_limits::TunerPermit;

const FFMPEG_BIN_ENV: &str = "HDHR_BACKEND_FFMPEG_BIN";
const TRANSCODE_DECODER_ENV: &str = "HDHR_BACKEND_TRANSCODE_DECODER";
const TRANSCODE_ENCODER_ENV: &str = "HDHR_BACKEND_TRANSCODE_ENCODER";
const TRANSCODE_BITRATE_ENV: &str = "HDHR_BACKEND_TRANSCODE_BITRATE";
const TRANSCODE_AUDIO_BITRATE_ENV: &str = "HDHR_BACKEND_TRANSCODE_AUDIO_BITRATE";
const TRANSCODE_AUDIO_CHANNELS_ENV: &str = "HDHR_BACKEND_TRANSCODE_AUDIO_CHANNELS";
const TRANSCODE_PROFILE_ENV: &str = "HDHR_BACKEND_TRANSCODE_PROFILE";
const TRANSCODE_MAX_HEIGHT_ENV: &str = "HDHR_BACKEND_TRANSCODE_MAX_HEIGHT";
const TRANSCODE_FPS_ENV: &str = "HDHR_BACKEND_TRANSCODE_FPS";

const DEFAULT_FFMPEG_BIN: &str = "ffmpeg";
const DEFAULT_BITRATE: &str = "4000k";

#[derive(Clone, Debug, Default)]
pub struct TranscodeRequestOptions {
    pub profile: Option<String>,
    pub video_bitrate: Option<String>,
    pub audio_bitrate: Option<String>,
    pub max_height: Option<u16>,
    pub fps: Option<u8>,
}

#[derive(Clone, Debug)]
struct EffectiveTranscodeSettings {
    video_bitrate: String,
    audio_bitrate: Option<String>,
    audio_channels: Option<u8>,
    max_height: Option<u16>,
    fps: Option<u8>,
    profile: String,
}

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

pub async fn serve_transcoded_stream(stream_url: String) -> Response {
    serve_transcoded_stream_with_options_and_permit(
        stream_url,
        TranscodeRequestOptions::default(),
        None,
    )
    .await
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
pub async fn serve_transcoded_stream_with_options(
    stream_url: String,
    options: TranscodeRequestOptions,
) -> Response {
    serve_transcoded_stream_with_options_and_permit(stream_url, options, None).await
}

pub async fn serve_transcoded_stream_with_options_and_permit(
    stream_url: String,
    options: TranscodeRequestOptions,
    permit: Option<TunerPermit>,
) -> Response {
    let ffmpeg_bin =
        std::env::var(FFMPEG_BIN_ENV).unwrap_or_else(|_| DEFAULT_FFMPEG_BIN.to_string());
    let encoder = std::env::var(TRANSCODE_ENCODER_ENV).unwrap_or_default();
    let settings = effective_settings(&options);

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
        .arg(&settings.video_bitrate)
        .arg("-bufsize")
        .arg({
            // bufsize = 2 × bitrate (rough rule for hardware encoders)
            let kbps: u32 = settings
                .video_bitrate
                .trim_end_matches('k')
                .trim_end_matches('K')
                .parse()
                .unwrap_or(4000);
            format!("{}k", kbps * 2)
        })
        .args(video_filter_args(settings.max_height, settings.fps));

    if let Some(audio_bitrate) = settings.audio_bitrate.as_deref() {
        cmd.arg("-c:a")
            .arg("aac")
            .arg("-b:a")
            .arg(audio_bitrate);
        if let Some(audio_channels) = settings.audio_channels {
            cmd.arg("-ac").arg(audio_channels.to_string());
        }
    } else {
        cmd.arg("-c:a").arg("copy");
    }

    cmd
        .arg("-f")
        .arg("mpegts")
        .arg("pipe:1")
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .kill_on_drop(true);

    info!(
        stream_url = %stream_url,
        encoder = %encoder,
        bitrate = %settings.video_bitrate,
        audio_bitrate = %settings.audio_bitrate.as_deref().unwrap_or("copy"),
        audio_channels = ?settings.audio_channels,
        max_height = ?settings.max_height,
        fps = ?settings.fps,
        profile = %settings.profile,
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
                let _permit = permit;
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

fn effective_settings(options: &TranscodeRequestOptions) -> EffectiveTranscodeSettings {
    let configured_profile = options
        .profile
        .clone()
        .or_else(|| std::env::var(TRANSCODE_PROFILE_ENV).ok())
        .unwrap_or_else(|| "legacy".to_string())
        .trim()
        .to_ascii_lowercase();

    let mut settings = match configured_profile.as_str() {
        "very_low" | "very-low" => EffectiveTranscodeSettings {
            video_bitrate: "500k".to_string(),
            audio_bitrate: Some("64k".to_string()),
            audio_channels: Some(2),
            max_height: Some(360),
            fps: Some(20),
            profile: "very_low".to_string(),
        },
        "low" => EffectiveTranscodeSettings {
            video_bitrate: "900k".to_string(),
            audio_bitrate: Some("96k".to_string()),
            audio_channels: Some(2),
            max_height: Some(480),
            fps: Some(24),
            profile: "low".to_string(),
        },
        "balanced" => EffectiveTranscodeSettings {
            video_bitrate: "1500k".to_string(),
            audio_bitrate: Some("128k".to_string()),
            audio_channels: Some(2),
            max_height: Some(720),
            fps: Some(30),
            profile: "balanced".to_string(),
        },
        "high" => EffectiveTranscodeSettings {
            video_bitrate: "3000k".to_string(),
            audio_bitrate: Some("160k".to_string()),
            audio_channels: Some(2),
            max_height: Some(1080),
            fps: Some(30),
            profile: "high".to_string(),
        },
        _ => EffectiveTranscodeSettings {
            video_bitrate: std::env::var(TRANSCODE_BITRATE_ENV)
                .unwrap_or_else(|_| DEFAULT_BITRATE.to_string()),
            audio_bitrate: std::env::var(TRANSCODE_AUDIO_BITRATE_ENV)
                .ok()
                .filter(|value| !value.trim().is_empty()),
            audio_channels: std::env::var(TRANSCODE_AUDIO_CHANNELS_ENV)
                .ok()
                .and_then(|value| value.parse::<u8>().ok())
                .filter(|value| *value > 0),
            max_height: std::env::var(TRANSCODE_MAX_HEIGHT_ENV)
                .ok()
                .and_then(|value| value.parse::<u16>().ok())
                .filter(|value| *value > 0),
            fps: std::env::var(TRANSCODE_FPS_ENV)
                .ok()
                .and_then(|value| value.parse::<u8>().ok())
                .filter(|value| *value > 0),
            profile: "legacy".to_string(),
        },
    };

    if let Some(video_bitrate) = options.video_bitrate.as_ref() {
        if !video_bitrate.trim().is_empty() {
            settings.video_bitrate = video_bitrate.clone();
        }
    }
    if let Some(audio_bitrate) = options.audio_bitrate.as_ref() {
        if audio_bitrate.trim().is_empty() {
            settings.audio_bitrate = None;
        } else {
            settings.audio_bitrate = Some(audio_bitrate.clone());
        }
    }
    if let Some(max_height) = options.max_height {
        if max_height > 0 {
            settings.max_height = Some(max_height);
        }
    }
    if let Some(fps) = options.fps {
        if fps > 0 {
            settings.fps = Some(fps);
        }
    }

    settings
}

fn video_filter_args(max_height: Option<u16>, fps: Option<u8>) -> Vec<String> {
    let mut filters = Vec::new();

    if let Some(max_height) = max_height {
        // Keep original aspect ratio while capping height, using a syntax that
        // ffmpeg's filter parser accepts without shell escaping.
        filters.push(format!(
            "scale=4096:{h}:force_original_aspect_ratio=decrease:force_divisible_by=2:flags=fast_bilinear",
            h = max_height
        ));
    }
    if let Some(fps) = fps {
        filters.push(format!("fps={fps}"));
    }

    if filters.is_empty() {
        Vec::new()
    } else {
        vec!["-vf".to_string(), filters.join(",")]
    }
}

#[cfg(test)]
mod tests {
    use super::{
        TranscodeRequestOptions, effective_settings, video_filter_args,
    };

    #[test]
    fn low_profile_sets_bandwidth_friendly_defaults() {
        let settings = effective_settings(&TranscodeRequestOptions {
            profile: Some("low".to_string()),
            ..TranscodeRequestOptions::default()
        });

        assert_eq!(settings.profile, "low");
        assert_eq!(settings.video_bitrate, "900k");
        assert_eq!(settings.audio_bitrate.as_deref(), Some("96k"));
        assert_eq!(settings.audio_channels, Some(2));
        assert_eq!(settings.max_height, Some(480));
        assert_eq!(settings.fps, Some(24));
    }

    #[test]
    fn request_overrides_replace_profile_defaults() {
        let settings = effective_settings(&TranscodeRequestOptions {
            profile: Some("very_low".to_string()),
            video_bitrate: Some("700k".to_string()),
            audio_bitrate: Some("80k".to_string()),
            max_height: Some(404),
            fps: Some(18),
        });

        assert_eq!(settings.video_bitrate, "700k");
        assert_eq!(settings.audio_bitrate.as_deref(), Some("80k"));
        assert_eq!(settings.max_height, Some(404));
        assert_eq!(settings.fps, Some(18));
    }

    #[test]
    fn video_filters_emit_expected_ffmpeg_arguments() {
        let filters = video_filter_args(Some(480), Some(24));

        assert_eq!(filters[0], "-vf");
        assert!(filters[1].contains("scale=4096:480:force_original_aspect_ratio=decrease:force_divisible_by=2:flags=fast_bilinear"));
        assert!(filters[1].contains("fps=24"));
    }
}
