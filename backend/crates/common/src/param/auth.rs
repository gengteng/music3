//! # Authorization parameters
//!

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;

/// Hmac request
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeRequest {
    /// Public key
    #[serde(with = "crate::utils::serde_str")]
    pub pub_key: Pubkey,
}

/// Nonce response
#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResponse {
    /// HMAC
    pub hmac: String,
    /// Timestamp
    pub timestamp: usize,
}

/// Authorization request
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Public key
    #[serde(with = "crate::utils::serde_str")]
    pub pub_key: Pubkey,
    /// Signature
    #[serde(with = "crate::utils::serde_str")]
    pub signature: Signature,
    /// HMAC
    pub hmac: String,
    /// Timestamp
    pub timestamp: usize,
    /// Duration
    pub duration: usize,
}

/// Authorization response
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Public key
    #[serde(with = "crate::utils::serde_str")]
    pub pub_key: Pubkey,
    /// JWT
    pub jwt: String,
    /// Expiration
    pub exp: usize,
}
