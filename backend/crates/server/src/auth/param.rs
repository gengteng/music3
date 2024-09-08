//! # Authorization parameters
//!

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

/// Hmac request
#[derive(Debug, Serialize, Deserialize)]
pub struct HmacRequest {
    /// Public key
    pub pub_key: Pubkey,
}

/// Nonce response
#[derive(Debug, Serialize, Deserialize)]
pub struct HmacResponse {
    /// HMAC
    pub hmac: String,
    /// Timestamp
    pub timestamp: usize,
}

/// Authorization request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Public key
    pub pub_key: Pubkey,
    /// Signature
    pub signature: Signature,
    /// HMAC
    pub hmac: String,
    /// Timestamp
    pub timestamp: usize,
    /// Duration
    pub duration: usize,
}

impl AuthRequest {
    /// Verify the request
    pub fn verify(&self) -> bool {
        let message = format!("{}{}", self.hmac, self.timestamp);
        self.signature
            .verify(self.pub_key.as_ref(), message.as_bytes())
    }
}

/// Authorization response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Public key
    pub pub_key: Pubkey,
    /// JWT
    pub jwt: String,
    /// Expiration
    pub exp: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::hmac::Hmac;
    use solana_sdk::signer::Signer;

    #[test]
    fn verify_user_signature() {
        let hmac = Hmac::try_from(b"secret").unwrap();
        let keypair = solana_sdk::signature::Keypair::new();
        let (hm, ts) = hmac.generate(&keypair.pubkey());
        let message = format!("{hm}{ts}");
        let signature = keypair.sign_message(message.as_bytes());
        let request = AuthRequest {
            pub_key: keypair.pubkey(),
            signature,
            hmac: hm.clone(),
            timestamp: ts,
            duration: 30,
        };
        let request_json = serde_json::to_string(&request).unwrap();
        let request = serde_json::from_str::<AuthRequest>(&request_json).unwrap();
        assert!(request.verify());
    }
}
