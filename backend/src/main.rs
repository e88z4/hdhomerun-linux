use std::net::SocketAddr;

use hdhomerun_backend::app::{AppState, build_app};
use hdhomerun_backend::state::resolve_default_state_dir;
use tokio::signal;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let bind_addr: SocketAddr = std::env::var("HDHR_BACKEND_BIND")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| SocketAddr::from(([127, 0, 0, 1], 38080)));

    let state_dir = resolve_default_state_dir()?;
    let app_state = AppState::new_default(state_dir);
    let app = build_app(app_state);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    info!(address = %bind_addr, "backend listening on loopback");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    let _ = signal::ctrl_c().await;
}