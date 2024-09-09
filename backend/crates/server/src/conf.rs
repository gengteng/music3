//! # Configuration
//!

use crate::auth::conf::AuthConfig;
use serde::{Deserialize, Serialize};

/// Music3 backend configuration
///
#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Config {
    /// Authorization configuration
    pub auth: AuthConfig,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let config = Config::default();
        let json = serde_json::to_string(&config).unwrap();
        let config2: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(config, config2);
    }
}
