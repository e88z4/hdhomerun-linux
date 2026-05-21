mod store;

use std::path::PathBuf;

use directories::ProjectDirs;

use crate::error::AppError;

pub use store::StateStore;

pub fn resolve_default_state_dir() -> Result<PathBuf, AppError> {
    let project_dirs = ProjectDirs::from("dev", "felix", "hdhomerun-linux-player")
        .ok_or_else(|| AppError::internal("failed to resolve XDG state directory"))?;
    let state_dir = project_dirs
        .state_dir()
        .ok_or_else(|| AppError::internal("failed to resolve backend state path"))?;
    Ok(state_dir.join("backend"))
}