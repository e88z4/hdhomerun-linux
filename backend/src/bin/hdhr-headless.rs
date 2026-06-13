use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use directories::BaseDirs;
use reqwest::Url;
use serde_json::{Value, json};

const DEFAULT_BACKEND_URL: &str = "http://127.0.0.1:38080";
const DEFAULT_SERVICE_NAME: &str = "hdhomerun-headless.service";

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "help" | "--help" | "-h" => {
            print_help();
            Ok(())
        }
        "devices" => cmd_devices(&args[2..]),
        "select-device" => cmd_select_device(&args[2..]),
        "channels" => cmd_channels(&args[2..]),
        "stream-url" => cmd_stream_url(&args[2..]),
        "service" => cmd_service(&args[2..]),
        other => Err(format!("unknown command: {other}").into()),
    }
}

fn print_help() {
    println!(
        "hdhr-headless - Headless streaming helper for HDHomeRun backend\n\n\
Commands:\n\
  devices                            List discovered devices\n\
  select-device --device-ref <id>    Persist selected device\n\
  channels [--device-ref <id>]       List channels for selected or explicit device\n\
  stream-url --device-ref <id> --channel-ref <ref> [--profile <name>] [--backend-url <url>] [--public-base <url>]\n\
                                    Build transcode stream URL for players\n\
  service install [--bind <addr>] [--encoder <name>] [--profile <name>] [--backend-bin <path>] [--service-name <name>]\n\
                                    Install + enable + start user systemd service\n\
  service start|stop|restart|status|logs|uninstall [--service-name <name>]\n\
Examples:\n\
  hdhr-headless devices\n\
  hdhr-headless channels\n\
  hdhr-headless stream-url --device-ref hdhr-10ab47d5 --channel-ref 29.1 --profile low --public-base http://192.168.1.10:39090\n\
  hdhr-headless service install --bind 0.0.0.0:39090 --encoder libx264 --profile low\n\
  hdhr-headless service status\n"
    );
}

fn cmd_devices(args: &[String]) -> Result<(), Box<dyn Error>> {
    let opts = parse_opts(args)?;
    let backend = backend_url(&opts)?;
    let response = get_json(&format!("{backend}/api/devices"))?;

    let devices = response
        .get("devices")
        .and_then(Value::as_array)
        .ok_or("invalid devices payload")?;

    if devices.is_empty() {
        println!("no devices found");
        return Ok(());
    }

    for device in devices {
        let device_ref = str_field(device, "deviceRef");
        let name = str_field(device, "name");
        let selected = device
            .get("isSelected")
            .and_then(Value::as_bool)
            .unwrap_or(false);
        if selected {
            println!("* {device_ref}\t{name}");
        } else {
            println!("  {device_ref}\t{name}");
        }
    }

    Ok(())
}

fn cmd_select_device(args: &[String]) -> Result<(), Box<dyn Error>> {
    let opts = parse_opts(args)?;
    let backend = backend_url(&opts)?;
    let device_ref = required_opt(&opts, "--device-ref")?;

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(format!("{backend}/api/devices/select"))
        .json(&json!({"deviceRef": device_ref}))
        .send()?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().unwrap_or_else(|_| "<unavailable>".to_string());
        return Err(format!("device select failed: {status} {body}").into());
    }

    println!("selected device: {device_ref}");
    Ok(())
}

fn cmd_channels(args: &[String]) -> Result<(), Box<dyn Error>> {
    let opts = parse_opts(args)?;
    let backend = backend_url(&opts)?;

    if let Some(device_ref) = opts.get("--device-ref") {
        let client = reqwest::blocking::Client::new();
        let response = client
            .post(format!("{backend}/api/devices/select"))
            .json(&json!({"deviceRef": device_ref}))
            .send()?;
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| "<unavailable>".to_string());
            return Err(format!("device select failed: {status} {body}").into());
        }
    }

    let payload = get_json(&format!("{backend}/api/lineup"))?;
    let channels = payload
        .get("channels")
        .and_then(Value::as_array)
        .ok_or("invalid lineup payload")?;

    if channels.is_empty() {
        println!("no channels available");
        return Ok(());
    }

    println!("guide\tname\tavailability\tchannelRef");
    for channel in channels {
        let guide = str_field(channel, "guideNumber");
        let name = str_field(channel, "guideName");
        let availability = str_field(channel, "availability");
        let channel_ref = str_field(channel, "channelRef");
        println!("{guide}\t{name}\t{availability}\t{channel_ref}");
    }

    Ok(())
}

fn cmd_stream_url(args: &[String]) -> Result<(), Box<dyn Error>> {
    let opts = parse_opts(args)?;

    let backend = backend_url(&opts)?;
    let device_ref = required_opt(&opts, "--device-ref")?;
    let channel_ref = required_opt(&opts, "--channel-ref")?;
    let profile = opts
        .get("--profile")
        .cloned()
        .unwrap_or_else(|| "low".to_string());

    let mut url = Url::parse(&backend)?.join("/api/stream/transcode/live")?;
    {
        let mut pairs = url.query_pairs_mut();
        pairs.append_pair("deviceRef", &device_ref);
        pairs.append_pair("channelRef", &channel_ref);
        pairs.append_pair("profile", &profile);

        for (src, dst) in [
            ("--video-bitrate", "videoBitrate"),
            ("--audio-bitrate", "audioBitrate"),
            ("--max-height", "maxHeight"),
            ("--fps", "fps"),
        ] {
            if let Some(value) = opts.get(src) {
                pairs.append_pair(dst, value);
            }
        }
    }

    let final_url = if let Some(public_base) = opts.get("--public-base") {
        let mut public_url = Url::parse(public_base)?.join("/api/stream/transcode/live")?;
        public_url.set_query(url.query());
        public_url
    } else {
        url
    };

    println!("{}", final_url.as_str());
    Ok(())
}

fn cmd_service(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args.is_empty() {
        return Err("missing service subcommand (install/start/stop/restart/status/logs/uninstall)".into());
    }

    let subcommand = args[0].as_str();
    let opts = parse_opts(&args[1..])?;
    let service_name = opts
        .get("--service-name")
        .cloned()
        .unwrap_or_else(|| DEFAULT_SERVICE_NAME.to_string());

    match subcommand {
        "install" => service_install(&service_name, &opts),
        "start" => systemctl_user(["start", &service_name]),
        "stop" => systemctl_user(["stop", &service_name]),
        "restart" => systemctl_user(["restart", &service_name]),
        "status" => systemctl_user(["status", &service_name, "--no-pager"]),
        "logs" => journalctl_user(["-u", &service_name, "-n", "100", "--no-pager"]),
        "uninstall" => service_uninstall(&service_name),
        _ => Err(format!("unknown service subcommand: {subcommand}").into()),
    }
}

fn service_install(service_name: &str, opts: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    let bind = opts
        .get("--bind")
        .cloned()
        .unwrap_or_else(|| "0.0.0.0:39090".to_string());
    let encoder = opts
        .get("--encoder")
        .cloned()
        .unwrap_or_else(|| "libx264".to_string());
    let profile = opts
        .get("--profile")
        .cloned()
        .unwrap_or_else(|| "low".to_string());

    let backend_bin = opts
        .get("--backend-bin")
        .map(PathBuf::from)
        .unwrap_or_else(default_backend_bin);

    if !backend_bin.exists() {
        return Err(format!("backend binary does not exist: {}", backend_bin.display()).into());
    }

    let unit_path = user_unit_path(service_name)?;
    let unit_parent = unit_path
        .parent()
        .ok_or("invalid unit file path")?;
    fs::create_dir_all(unit_parent)?;

    let unit = format!(
        "[Unit]\n\
Description=HDHomeRun Headless Streaming Backend\n\
After=network-online.target\n\
Wants=network-online.target\n\
\n\
[Service]\n\
Type=simple\n\
Environment=HDHR_BACKEND_BIND={bind}\n\
Environment=HDHR_BACKEND_PLAYER_MODE=client\n\
Environment=HDHR_BACKEND_TRANSCODE_ENCODER={encoder}\n\
Environment=HDHR_BACKEND_TRANSCODE_PROFILE={profile}\n\
ExecStart={}\n\
Restart=on-failure\n\
RestartSec=2\n\
\n\
[Install]\n\
WantedBy=default.target\n",
        backend_bin.display()
    );

    fs::write(&unit_path, unit)?;
    println!("wrote {}", unit_path.display());

    systemctl_user(["daemon-reload"])?;
    systemctl_user(["enable", "--now", service_name])?;
    systemctl_user(["status", service_name, "--no-pager"])?;

    println!(
        "\nservice installed. stream URL example:\n{}",
        Url::parse(&format!("http://{bind}"))?.join("/api/stream/transcode/live?deviceRef=<deviceRef>&channelRef=29.1&profile=low")?
    );

    Ok(())
}

fn service_uninstall(service_name: &str) -> Result<(), Box<dyn Error>> {
    let _ = systemctl_user(["disable", "--now", service_name]);
    let _ = systemctl_user(["daemon-reload"]);

    let unit_path = user_unit_path(service_name)?;
    if unit_path.exists() {
        fs::remove_file(&unit_path)?;
        println!("removed {}", unit_path.display());
    }

    systemctl_user(["daemon-reload"])?;
    Ok(())
}

fn user_unit_path(service_name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let base_dirs = BaseDirs::new().ok_or("failed to determine home directory")?;
    Ok(base_dirs
        .home_dir()
        .join(".config")
        .join("systemd")
        .join("user")
        .join(service_name))
}

fn default_backend_bin() -> PathBuf {
    if let Ok(current_exe) = std::env::current_exe() {
        if let Some(parent) = current_exe.parent() {
            let sibling = parent.join("hdhomerun-backend");
            if sibling.exists() {
                return sibling;
            }
        }
    }

    let candidate = Path::new("target/release/hdhomerun-backend");
    if candidate.exists() {
        candidate.to_path_buf()
    } else {
        PathBuf::from("hdhomerun-backend")
    }
}

fn backend_url(opts: &HashMap<String, String>) -> Result<String, Box<dyn Error>> {
    let base = opts
        .get("--backend-url")
        .cloned()
        .unwrap_or_else(|| DEFAULT_BACKEND_URL.to_string());
    let url = Url::parse(&base)?;
    Ok(format!("{}://{}", url.scheme(), url.host_str().unwrap_or("127.0.0.1"))
        + &url
            .port()
            .map(|p| format!(":{p}"))
            .unwrap_or_default())
}

fn get_json(url: &str) -> Result<Value, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().unwrap_or_else(|_| "<unavailable>".to_string());
        return Err(format!("request failed: {status} {body}").into());
    }
    Ok(response.json::<Value>()?)
}

fn required_opt(opts: &HashMap<String, String>, key: &str) -> Result<String, Box<dyn Error>> {
    opts.get(key)
        .cloned()
        .ok_or_else(|| format!("missing required option: {key}").into())
}

fn parse_opts(args: &[String]) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut options = HashMap::new();
    let mut index = 0;

    while index < args.len() {
        let key = &args[index];
        if !key.starts_with("--") {
            return Err(format!("unexpected argument: {key}").into());
        }

        let value = args
            .get(index + 1)
            .ok_or_else(|| format!("missing value for option: {key}"))?
            .clone();

        if value.starts_with("--") {
            return Err(format!("missing value for option: {key}").into());
        }

        options.insert(key.clone(), value);
        index += 2;
    }

    Ok(options)
}

fn str_field(value: &Value, field: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string()
}

fn systemctl_user<'a, I>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = &'a str>,
{
    run_command("systemctl", ["--user"].into_iter().chain(args))
}

fn journalctl_user<'a, I>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = &'a str>,
{
    run_command("journalctl", ["--user"].into_iter().chain(args))
}

fn run_command<'a, I>(program: &str, args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = &'a str>,
{
    let args: Vec<&str> = args.into_iter().collect();
    let status = Command::new(program).args(&args).status()?;
    if !status.success() {
        return Err(format!("command failed: {program} {}", args.join(" ")).into());
    }
    Ok(())
}
