//! # JSON Web Token
//!

use crate::auth::{
    claim::Claim,
    error::{Error, Result},
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use music3_common::utils::Base64;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::ops::Deref;
use std::sync::Arc;

/// JWT configuration
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct JwtConfig {
    /// Audience
    pub audience: String,
    /// Secret in base64
    pub secret: Base64,
    /// Max duration in seconds
    pub max_duration_sec: usize,
    /// Timestamp timeout
    pub timestamp_timeout_sec: usize,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            audience: "music3".to_string(),
            secret: Base64::from(b"music3-jwt-secret"),
            max_duration_sec: 86400,
            timestamp_timeout_sec: 120,
        }
    }
}

/// JSON Web Token Context
#[derive(Clone)]
pub struct Jwt {
    inner: Arc<JwtInner>,
}

impl Deref for Jwt {
    type Target = JwtInner;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl From<JwtInner> for Jwt {
    fn from(inner: JwtInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl From<&JwtConfig> for Jwt {
    fn from(config: &JwtConfig) -> Self {
        Self::from(JwtInner::from(config))
    }
}

impl Jwt {
    /// Create a new JWT
    pub fn sign(&self, claims: &Claim) -> Result<String> {
        self.inner.sign(claims)
    }

    /// Verify a JWT
    pub fn verify(&self, token: &str) -> Result<Claim> {
        self.inner.verify(token)
    }
}

/// JSON Web Token Inner
#[derive(Clone)]
pub struct JwtInner {
    audience: String,
    max_duration_sec: usize,
    timestamp_timeout_sec: usize,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl From<&JwtConfig> for JwtInner {
    fn from(config: &JwtConfig) -> Self {
        Self::from_secret(
            &config.audience,
            config.max_duration_sec,
            config.timestamp_timeout_sec,
            config.secret.as_ref(),
        )
    }
}

impl JwtInner {
    /// Create a new JWT
    pub fn from_secret(
        aud: impl Into<String>,
        max_duration_sec: usize,
        timestamp_timeout_sec: usize,
        secret: &[u8],
    ) -> Self {
        Self {
            audience: aud.into(),
            max_duration_sec,
            timestamp_timeout_sec,
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    /// Create a new JWT
    pub fn sign<C: Serialize>(&self, claims: &C) -> Result<String> {
        let mut json = serde_json::to_value(claims).map_err(Error::FailedToSerializeClaim)?;
        if let Some(map) = json.as_object_mut() {
            map.insert("aud".to_string(), Value::String(self.audience.clone()));
        }
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &json,
            &self.encoding_key,
        )?)
    }

    /// Verify a JWT
    pub fn verify<C: DeserializeOwned>(&self, token: &str) -> Result<C> {
        let mut validation = jsonwebtoken::Validation::default();
        let mut audiences = HashSet::new();
        audiences.insert(self.audience.clone());
        validation.aud = Some(audiences);
        jsonwebtoken::decode::<C>(token, &self.decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(Error::JwtVerificationFailed)
    }

    /// Get the max duration in seconds
    pub fn max_duration_sec(&self) -> usize {
        self.max_duration_sec
    }

    /// Get the timestamp timeout in seconds
    pub fn timestamp_timeout_sec(&self) -> usize {
        self.timestamp_timeout_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{get_current_timestamp, Validation};
    use serde::{Deserialize, Serialize};
    use solana_sdk::signature::{Keypair, Signer};

    #[test]
    fn sign_and_verify_jwt() {
        let keypair = Keypair::new();
        #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
        struct Claim {
            aud: String,
            sub: String,
            exp: usize,
        }
        let jwt = JwtInner::from_secret("music3", 86400, 120, b"secret");
        let claim = Claim {
            aud: "music3".to_string(),
            sub: keypair.pubkey().to_string(),
            exp: (get_current_timestamp() - Validation::default().leeway) as usize,
        };
        println!("claim: {:?}", claim);
        let token = jwt.sign(&claim).unwrap();
        println!("token: {token}");
        let claim_: Claim = jwt.verify(&token).unwrap();
        assert_eq!(claim, claim_);
    }
}
