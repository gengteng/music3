//! # Authorization
//!
//! # 鉴权流程：
//! 1. 客户端发送 PubKey 给服务端。
//! 2. 服务端返回基于 PubKey 和服务端当前时间戳生成的 HMAC，同时返回使用的时间戳。
//! 3. 客户端将 HMAC 和服务端返回的时间戳使用私钥签名，即 sign(base64(hmac) + toString(timestamp))。
//! 4. 客户端发送 PubKey、签名、时间戳和期望的 JWT 有效期给服务端。
//! 5. 服务端验证时间戳是否是最近的时间（比如 5 分钟），验证 HMAC 和签名，通过后下发 JWT。
//!

use crate::auth::claim::Claim;
use std::sync::Arc;

pub mod claim;
pub mod conf;
pub mod error;
pub mod hmac;
pub mod jwt;
pub mod param;

/// Authorizer
#[derive(Clone)]
pub struct Authorizer {
    jwt: jwt::Jwt,
    hmac: Arc<hmac::Hmac>,
    max_duration_sec: usize,
}

impl Authorizer {
    /// Create a new hmac instance from the current hmac
    pub fn hmac_cloned(&self) -> hmac::Hmac {
        self.hmac.as_ref().clone()
    }

    /// Create a new authorizer
    pub fn new(config: conf::AuthConfig) -> anyhow::Result<Self> {
        let jwt = jwt::Jwt::try_from(&config.jwt)?;
        let hmac = Arc::new(hmac::Hmac::try_from(config.hmac_secret.as_bytes())?);
        Ok(Self {
            jwt,
            hmac,
            max_duration_sec: config.max_duration_sec,
        })
    }

    /// Generate HMAC
    pub fn generate_hmac(&self, pub_key: &solana_sdk::pubkey::Pubkey) -> param::HmacResponse {
        let (hmac, timestamp) = self.hmac_cloned().generate(pub_key);
        param::HmacResponse { hmac, timestamp }
    }

    /// Authorize the request
    pub fn authorize(&self, request: &param::AuthRequest) -> anyhow::Result<param::AuthResponse> {
        if self.max_duration_sec < request.duration {
            return Err(anyhow::anyhow!("The duration is too long"));
        }
        let claim = Claim::create(request.pub_key.to_string(), request.duration);
        let jwt = self.jwt.sign(&claim)?;
        Ok(param::AuthResponse {
            pub_key: request.pub_key,
            jwt,
            exp: claim.exp,
        })
    }
}
