//! # JSON Web Token
//!

use crate::auth::{
    claim::Claim,
    error::{Error, Result},
};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::sync::Arc;

/// JWT configuration
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtConfig {
    /// Audience
    pub audience: String,
    /// Secret in base64
    pub secret_bas64: String,
}

/// JSON Web Token Context
#[derive(Clone)]
pub struct Jwt {
    inner: Arc<JwtInner>,
}

impl From<JwtInner> for Jwt {
    fn from(inner: JwtInner) -> Self {
        Self {
            inner: Arc::new(inner),
        }
    }
}

impl TryFrom<&JwtConfig> for Jwt {
    type Error = Error;
    fn try_from(config: &JwtConfig) -> Result<Self> {
        Ok(Self::from(JwtInner::try_from(config)?))
    }
}

impl Jwt {
    /// Create a new JWT
    pub fn sign(&self, claims: &Claim) -> Result<String> {
        Ok(self.inner.sign(claims)?)
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
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl TryFrom<&JwtConfig> for JwtInner {
    type Error = jsonwebtoken::errors::Error;
    fn try_from(config: &JwtConfig) -> jsonwebtoken::errors::Result<Self> {
        Ok(Self::from_base64_secret(
            &config.audience,
            &config.secret_bas64,
        )?)
    }
}

impl JwtInner {
    /// Create a new JWT
    pub fn from_secret(aud: impl Into<String>, secret: &[u8]) -> Self {
        Self {
            audience: aud.into(),
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    /// Create a new JWT from base64 secret
    pub fn from_base64_secret(
        aud: impl Into<String>,
        secret: &str,
    ) -> jsonwebtoken::errors::Result<Self> {
        Ok(Self {
            audience: aud.into(),
            encoding_key: EncodingKey::from_base64_secret(secret)?,
            decoding_key: DecodingKey::from_base64_secret(secret)?,
        })
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
        Ok(
            jsonwebtoken::decode::<C>(token, &self.decoding_key, &validation)
                .map(|data| data.claims)
                .map_err(Error::JwtVerificationFailed)?,
        )
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
        let jwt = JwtInner::from_secret("music3", b"secret");
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
