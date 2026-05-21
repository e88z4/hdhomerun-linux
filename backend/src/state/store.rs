use std::fs;
use std::path::PathBuf;

use crate::error::AppError;
use crate::models::RememberedContext;

#[derive(Clone)]
pub struct StateStore {
    state_dir: PathBuf,
}

impl StateStore {
    pub fn new(state_dir: PathBuf) -> Self {
        Self { state_dir }
    }

    pub fn load_context(&self) -> Result<Option<RememberedContext>, AppError> {
        let path = self.context_path();
        if !path.exists() {
            return Ok(None);
        }

        let contents = fs::read_to_string(&path)
            .map_err(|source| AppError::StateIo { source, path: path.clone() })?;
        let context = serde_json::from_str(&contents).map_err(|source| AppError::StateParse {
            source,
            path: path.clone(),
        })?;

        Ok(Some(context))
    }

    pub fn save_context(&self, context: &RememberedContext) -> Result<(), AppError> {
        let path = self.context_path();
        self.ensure_parent(&path)?;

        let body = serde_json::to_string_pretty(context)
            .map_err(|source| AppError::Serialization { source })?;
        fs::write(&path, body).map_err(|source| AppError::StateIo { source, path })
    }

    pub fn clear_context(&self) -> Result<(), AppError> {
        let path = self.context_path();
        if path.exists() {
            fs::remove_file(&path).map_err(|source| AppError::StateIo { source, path })?;
        }
        Ok(())
    }

    pub fn context_path(&self) -> PathBuf {
        self.state_dir.join("remembered-context.json")
    }

    fn ensure_parent(&self, path: &PathBuf) -> Result<(), AppError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| AppError::StateIo {
                source,
                path: parent.to_path_buf(),
            })?;
        }
        Ok(())
    }
}