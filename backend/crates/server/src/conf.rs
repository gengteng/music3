//! # Configuration
//!

use crate::auth::conf::AuthConfig;
use serde::{Deserialize, Serialize};

/// Music3 backend configuration
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Authorization configuration
    pub auth: AuthConfig,
}
