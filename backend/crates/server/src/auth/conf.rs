//! Configuration for the authorization module.
//!
use crate::auth::jwt::JwtConfig;
use serde::{Deserialize, Serialize};

/// Authorization configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthConfig {
    /// JWT configuration
    pub jwt: JwtConfig,
    /// HMAC secret
    pub hmac_secret: String,
    /// Max duration in seconds
    pub max_duration_sec: usize,
}
