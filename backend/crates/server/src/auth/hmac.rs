//! # HMAC

use base64::Engine;
use hmac::Mac;
use jsonwebtoken::get_current_timestamp;
use sha2::Sha256;
use solana_sdk::pubkey::Pubkey;

type HmacSha256 = hmac::Hmac<Sha256>;

/// HMAC
#[derive(Clone)]
pub struct Hmac {
    hmac_sha256: HmacSha256,
}

impl TryFrom<&[u8]> for Hmac {
    type Error = anyhow::Error;
    fn try_from(secret: &[u8]) -> anyhow::Result<Self> {
        let hmac_sha256 = HmacSha256::new_from_slice(secret)?;
        Ok(Self { hmac_sha256 })
    }
}

impl<const L: usize> TryFrom<&[u8; L]> for Hmac {
    type Error = anyhow::Error;
    fn try_from(secret: &[u8; L]) -> anyhow::Result<Self> {
        let hmac_sha256 = HmacSha256::new_from_slice(secret)?;
        Ok(Self { hmac_sha256 })
    }
}

impl Hmac {
    /// Create a new HMAC and a timestamp
    pub fn generate(mut self, pub_key: &Pubkey) -> (String, usize) {
        let timestamp = get_current_timestamp();
        let message = format!("{}{}", pub_key, timestamp);
        self.hmac_sha256.update(message.as_bytes());
        let result = self.hmac_sha256.finalize();
        let code_bytes = result.into_bytes();
        (
            base64::engine::general_purpose::STANDARD.encode(code_bytes),
            timestamp as usize,
        )
    }

    /// Verify the HMAC
    pub fn verify(mut self, pub_key: &Pubkey, hmac: &str, timestamp: usize) -> bool {
        let message = format!("{}{}", pub_key, timestamp);
        self.hmac_sha256.update(message.as_bytes());
        let result = self.hmac_sha256.finalize();
        let code_bytes = result.into_bytes();
        let Ok(code) = base64::engine::general_purpose::STANDARD.decode(hmac.as_bytes()) else {
            return false;
        };
        code == *code_bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_hmac_and_verify() {
        let secret = b"music3-hmac-secret";
        let hmac = Hmac::try_from(secret).unwrap();
        let pub_key = Pubkey::new_unique();
        let (hmac_code, ts) = hmac.clone().generate(&pub_key);
        assert!(hmac.verify(&pub_key, &hmac_code, ts));
    }
}
