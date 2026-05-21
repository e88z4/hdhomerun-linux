use std::io;
use std::path::PathBuf;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::models::ApiErrorResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("state I/O failure at {path:?}")]
    StateIo { source: io::Error, path: PathBuf },

    #[error("state parse failure at {path:?}")]
    StateParse {
        source: serde_json::Error,
        path: PathBuf,
    },

    #[error("serialization failure")]
    Serialization { source: serde_json::Error },

    #[error("validation failure: {0}")]
    Validation(String),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Validation(_) => StatusCode::BAD_REQUEST,
            Self::StateIo { .. } | Self::StateParse { .. } | Self::Serialization { .. } | Self::Internal(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    fn to_api_error(&self) -> ApiErrorResponse {
        match self {
            Self::Validation(message) => ApiErrorResponse::new(
                "validation_failed",
                message,
                "Correct the request and retry.",
            ),
            Self::StateIo { .. } | Self::StateParse { .. } | Self::Serialization { .. } => ApiErrorResponse::new(
                "state_unavailable",
                "The backend state could not be loaded safely.",
                "Retry after the backend finishes initialization.",
            ),
            Self::Internal(_) => ApiErrorResponse::new(
                "internal_error",
                "The backend encountered an internal error.",
                "Retry the request. If the problem persists, restart the backend.",
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(self.to_api_error());
        (status, body).into_response()
    }
}