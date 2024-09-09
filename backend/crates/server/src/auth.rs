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
use crate::auth::conf::AuthConfig;
use axum::extract::State;
use axum::Json;
use error::Result;
use jsonwebtoken::get_current_timestamp;
use music3_common::param::auth::{AuthRequest, AuthResponse, ChallengeRequest, ChallengeResponse};
use std::sync::Arc;

pub mod claim;
pub mod conf;
pub mod error;
pub mod hmac;
pub mod jwt;

/// Authorizer
#[derive(Clone)]
pub struct Authorizer {
    jwt: jwt::Jwt,
    hmac: Arc<hmac::Hmac>,
}

impl Authorizer {
    /// Create a new hmac instance from the current hmac
    pub fn hmac_cloned(&self) -> hmac::Hmac {
        self.hmac.as_ref().clone()
    }

    /// Create a new authorizer
    pub fn new(config: AuthConfig) -> anyhow::Result<Self> {
        let jwt = jwt::Jwt::try_from(&config.jwt)?;
        let hmac = Arc::new(hmac::Hmac::try_from(config.hmac_secret.as_bytes())?);
        Ok(Self { jwt, hmac })
    }

    /// Generate a challenge
    pub fn generate_challenge(&self, pub_key: &solana_sdk::pubkey::Pubkey) -> ChallengeResponse {
        let (hmac, timestamp) = self.hmac_cloned().generate(pub_key);
        ChallengeResponse { hmac, timestamp }
    }

    /// Check if the timestamp is valid
    pub fn is_valid_timestamp(&self, timestamp: usize) -> bool {
        timestamp + self.jwt.timestamp_timeout_sec() >= get_current_timestamp() as usize
    }

    /// Verify the auth request
    pub fn verify_auth_request(&self, request: &AuthRequest) -> bool {
        if !self
            .hmac_cloned()
            .verify(&request.pub_key, &request.hmac, request.timestamp)
        {
            return false;
        }
        let message = format!("{}{}", request.hmac, request.timestamp);
        request
            .signature
            .verify(request.pub_key.as_ref(), message.as_bytes())
    }

    /// Authorize the request
    pub fn authorize(&self, request: &AuthRequest) -> Result<AuthResponse> {
        if self.jwt.max_duration_sec() < request.duration {
            return Err(error::Error::InvalidDuration(
                request.duration,
                self.jwt.max_duration_sec(),
            ));
        }
        if !self.is_valid_timestamp(request.timestamp) {
            return Err(error::Error::InvalidTimestamp);
        }
        if !self.verify_auth_request(request) {
            return Err(error::Error::InvalidSignature);
        }
        let claim = Claim::create(request.pub_key.to_string(), request.duration);
        let jwt = self.jwt.sign(&claim)?;
        Ok(AuthResponse {
            pub_key: request.pub_key,
            jwt,
            exp: claim.exp,
        })
    }
}

/// Get challenge
pub async fn get_challenge(
    State(authorizer): State<Authorizer>,
    Json(request): Json<ChallengeRequest>,
) -> Json<ChallengeResponse> {
    Json(authorizer.generate_challenge(&request.pub_key))
}

/// Authorize
pub async fn authorize(
    authorizer: State<Authorizer>,
    request: Json<AuthRequest>,
) -> Result<Json<AuthResponse>> {
    Ok(Json(authorizer.authorize(&request)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::JwtConfig;
    use axum::routing::post;
    use axum::Router;
    use axum_test::TestServer;
    use rand::{thread_rng, Rng};
    use solana_sdk::signature::Keypair;
    use solana_sdk::signer::Signer;

    #[tokio::test]
    async fn challenge_and_authorize() {
        let config = AuthConfig {
            jwt: JwtConfig::default(),
            hmac_secret: "music3-hmac-secret".to_string(),
        };

        let max_duration_sec = config.jwt.max_duration_sec;

        let authorizer = Authorizer::new(config).expect("Failed to create authorizer");
        let my_app = Router::new()
            .route("/challenge", post(get_challenge))
            .route("/authorize", post(authorize))
            .with_state(authorizer.clone());

        let server = TestServer::new(my_app).expect("Failed to create test server");

        let keypair = Keypair::new();

        let response = server
            .post("/challenge")
            .json(&ChallengeRequest {
                pub_key: keypair.pubkey(),
            })
            .await;

        let challenge: ChallengeResponse = response.json();
        let message = challenge.build_message();
        let signature = keypair.sign_message(message.as_bytes());

        let duration = thread_rng().gen_range(0..max_duration_sec);
        let response = server
            .post("/authorize")
            .json(&AuthRequest {
                pub_key: keypair.pubkey(),
                signature,
                hmac: challenge.hmac.clone(),
                timestamp: challenge.timestamp,
                duration,
            })
            .await;

        assert_eq!(response.status_code(), 200);
        let AuthResponse { pub_key, jwt, exp } = response.json();
        assert_eq!(pub_key, keypair.pubkey());
        assert!(exp >= challenge.timestamp + duration);

        let claim = authorizer.jwt.verify(&jwt).unwrap();
        assert_eq!(claim.sub, keypair.pubkey().to_string());
    }
}
