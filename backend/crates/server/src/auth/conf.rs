//! Configuration for the authorization module.
//!
use crate::auth::jwt::JwtConfig;
use serde::{Deserialize, Serialize};

/// Authorization configuration
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct AuthConfig {
    /// JWT configuration
    pub jwt: JwtConfig,
    /// HMAC secret
    pub hmac_secret: String,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt: JwtConfig::default(),
            hmac_secret: "music3-hmac-secret".to_string(),
        }
    }
}
