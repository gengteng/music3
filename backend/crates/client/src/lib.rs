#![doc = include_str!("../README.md")]
#![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

pub mod error;
pub mod solana;
use error::Result;
use music3_common::param::auth::{AuthRequest, AuthResponse, ChallengeRequest, ChallengeResponse};
use reqwest::{multipart, Url};
use solana_sdk::pubkey::Pubkey;

/// Music3 Client
#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    base_url: Url,
}

impl From<Url> for Client {
    fn from(base_url: Url) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
        }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::from(Url::parse("https://music3-dev.shuttleapp.rs").expect("Invalid static URL"))
    }
}

impl Client {
    /// Get a challenge
    pub async fn get_challenge(&self, pub_key: Pubkey) -> Result<ChallengeResponse> {
        let url = self.base_url.join("/auth/challenge")?;
        let response = self
            .client
            .post(url)
            .json(&ChallengeRequest { pub_key })
            .send()
            .await?;
        let status = response.status();
        if !status.is_success() {
            return Err(error::Error::Non2xxResponse(status, response.text().await?));
        }

        Ok(response.json().await?)
    }

    /// Authorize
    pub async fn authorize(&self, request: &AuthRequest) -> Result<AuthResponse> {
        let url = self.base_url.join("/auth/authorize")?;
        let response = self.client.post(url).json(request).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(error::Error::Non2xxResponse(status, response.text().await?));
        }

        Ok(response.json().await?)
    }

    /// Upload a music file
    pub async fn upload_music(&self, file: Vec<u8>) -> Result<()> {
        // 创建多部分表单
        let form = multipart::Form::new().part(
            "file",
            multipart::Part::bytes(file).file_name("default.mp3"),
        );

        // 发送请求
        let url = self.base_url.join("/upload")?;
        let response = self.client.post(url).multipart(form).send().await?;
        let status = response.status();
        if !status.is_success() {
            return Err(error::Error::Non2xxResponse(status, response.text().await?));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::signature::{Keypair, Signer};

    #[tokio::test]
    async fn get_challenge_and_authorize() {
        let client = Client::default();
        let keypair = Keypair::new();
        let challenge = client.get_challenge(keypair.pubkey()).await.unwrap();
        let signature = keypair.sign_message(&challenge.build_message());
        let request = AuthRequest {
            pub_key: keypair.pubkey(),
            signature,
            hmac: challenge.hmac,
            timestamp: challenge.timestamp,
            duration: 3600,
        };
        let response = client.authorize(&request).await.unwrap();
        assert_eq!(response.pub_key, keypair.pubkey());
    }
}
