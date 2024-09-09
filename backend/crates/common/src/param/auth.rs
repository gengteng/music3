//! # Authorization parameters
//!

use crate::utils::Base64;
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
    pub hmac: Base64,
    /// Timestamp
    pub timestamp: usize,
}

impl ChallengeResponse {
    /// Build the message to sign
    pub fn build_message(&self) -> Vec<u8> {
        build_message(&self.hmac, self.timestamp)
    }
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
    pub hmac: Base64,
    /// Timestamp
    pub timestamp: usize,
    /// Duration
    pub duration: usize,
}

impl AuthRequest {
    /// Build the message to verify
    pub fn build_message(&self) -> Vec<u8> {
        build_message(&self.hmac, self.timestamp)
    }
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

fn build_message(hmac: &Base64, timestamp: usize) -> Vec<u8> {
    let mut vec = Vec::with_capacity(hmac.len() + size_of::<u64>());
    vec.extend_from_slice(hmac.as_ref());
    vec.extend_from_slice(&timestamp.to_be_bytes());
    vec
}
