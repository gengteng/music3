//! # Error types for the auth module
//!

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum_extra::typed_header::TypedHeaderRejection;

/// Error types for the auth module
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Invalid signature
    #[error("Invalid signature")]
    InvalidSignature,
    /// Serialize claim
    #[error("Failed to serialize claim: {0}")]
    FailedToSerializeClaim(#[from] serde_json::Error),
    /// JWT not provided
    #[error("JWT not provided or invalid: {0}")]
    JwtNotProvidedOrInvalid(#[from] TypedHeaderRejection),
    /// JWT verification failed
    #[error("JWT verification failed: {0}")]
    JwtVerificationFailed(#[from] jsonwebtoken::errors::Error),
    /// Invalid duration
    #[error("Invalid duration, expected: [0, {0}], got: {1}")]
    InvalidDuration(usize, usize),
    /// Invalid timestamp
    #[error("Invalid Timestamp")]
    InvalidTimestamp,
}

/// Result type for the auth module
pub type Result<T> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}
