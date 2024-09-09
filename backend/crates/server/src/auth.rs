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
use crate::auth::param::ChallengeRequest;
use axum::extract::State;
use axum::Json;
use error::Result;
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
    pub fn generate_challenge(
        &self,
        pub_key: &solana_sdk::pubkey::Pubkey,
    ) -> param::ChallengeResponse {
        let (hmac, timestamp) = self.hmac_cloned().generate(pub_key);
        param::ChallengeResponse { hmac, timestamp }
    }

    /// Authorize the request
    pub fn authorize(&self, request: &param::AuthRequest) -> Result<param::AuthResponse> {
        if self.jwt.max_duration_sec() < request.duration {
            return Err(error::Error::InvalidDuration(
                request.duration,
                self.jwt.max_duration_sec(),
            ));
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

/// Get challenge
pub async fn get_challenge(
    State(authorizer): State<Authorizer>,
    Json(request): Json<ChallengeRequest>,
) -> Json<param::ChallengeResponse> {
    Json(authorizer.generate_challenge(&request.pub_key))
}

/// Authorize
pub async fn authorize(
    authorizer: State<Authorizer>,
    request: Json<param::AuthRequest>,
) -> Result<Json<param::AuthResponse>> {
    Ok(Json(authorizer.authorize(&request)?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::jwt::JwtConfig;
    use crate::auth::param::ChallengeResponse;
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

        let ChallengeResponse { hmac, timestamp } = response.json();
        let message = format!("{}{}", hmac, timestamp);
        let signature = keypair.sign_message(message.as_bytes());

        let duration = thread_rng().gen_range(0..max_duration_sec);
        let response = server
            .post("/authorize")
            .json(&param::AuthRequest {
                pub_key: keypair.pubkey(),
                signature,
                hmac,
                timestamp,
                duration,
            })
            .await;

        let param::AuthResponse { pub_key, jwt, exp } = response.json();
        assert_eq!(pub_key, keypair.pubkey());
        assert!(exp >= timestamp + duration);

        let claim = authorizer.jwt.verify(&jwt).unwrap();
        assert_eq!(claim.sub, keypair.pubkey().to_string());
    }
}
