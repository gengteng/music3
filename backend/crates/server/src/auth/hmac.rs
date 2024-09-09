//! # HMAC

use hmac::Mac;
use jsonwebtoken::get_current_timestamp;
use music3_common::utils::Base64;
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
    pub fn generate(mut self, pub_key: &Pubkey) -> (Base64, usize) {
        let timestamp = get_current_timestamp() as usize;
        let message = Self::build_message(pub_key, timestamp);
        self.hmac_sha256.update(&message);
        let result = self.hmac_sha256.finalize();
        let code_bytes = result.into_bytes();
        (Base64(code_bytes.to_vec()), timestamp as usize)
    }

    /// Verify the HMAC
    pub fn verify(mut self, pub_key: &Pubkey, hmac: &Base64, timestamp: usize) -> bool {
        let message = Self::build_message(pub_key, timestamp);
        self.hmac_sha256.update(&message);
        let result = self.hmac_sha256.finalize();
        let code_bytes = result.into_bytes();
        hmac.as_slice().eq(code_bytes.as_slice())
    }

    fn build_message(pub_key: &Pubkey, timestamp: usize) -> Vec<u8> {
        let mut vec = Vec::with_capacity(size_of::<Pubkey>() + size_of::<u64>());
        vec.extend_from_slice(pub_key.as_ref());
        vec.extend_from_slice(&timestamp.to_be_bytes());
        vec
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
