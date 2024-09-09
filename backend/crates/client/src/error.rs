//! # Error module
//!

/// Error type
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Non-2xx response
    #[error("The server returned non-2xx response: ({0}) {1}")]
    Non2xxResponse(reqwest::StatusCode, String),
    /// Failed to parse URL
    #[error("Failed to parse URL: {0}")]
    UrlParse(#[from] url::ParseError),
    /// Reqwest client error
    #[error("Reqwest client error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

/// Result type
pub type Result<T> = std::result::Result<T, Error>;
