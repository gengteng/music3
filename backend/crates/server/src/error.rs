//! # Error handling

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::borrow::Cow;

/// Error types for the music3 backend
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Authorization error
    #[error(transparent)]
    Auth(#[from] crate::auth::error::Error),
    /// Unexpected error
    #[error("Unexpected error: {0}")]
    Unexpected(Cow<'static, str>),
}

/// Result type for the music3 backend
pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::Auth(e) => e.into_response(),
            Error::Unexpected(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
        }
    }
}
