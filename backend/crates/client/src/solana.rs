//! # Solana client

use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::tpu_client::TpuSenderError;
use solana_quic_client::{QuicConfig, QuicConnectionManager, QuicPool};
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

/// Solana client
pub struct SolanaClient {
    inner: solana_client::nonblocking::tpu_client::TpuClient<
        QuicPool,
        QuicConnectionManager,
        QuicConfig,
    >,
}

impl SolanaClient {
    /// Create a new Solana client of the devnet
    pub async fn dev_net() -> Result<Self, TpuSenderError> {
        let rpc_client = RpcClient::new("https://api.devnet.solana.com".to_string());
        let solana_client = solana_client::nonblocking::tpu_client::TpuClient::new(
            "music3",
            rpc_client.into(),
            "wss://api.devnet.solana.com",
            Default::default(),
        )
        .await?;
        Ok(Self {
            inner: solana_client,
        })
    }

    /// Get account
    pub async fn get_account(&self, pub_key: &Pubkey) -> Result<Account, ClientError> {
        self.inner.rpc_client().get_account(pub_key).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[tokio::test]
    async fn get_account() {
        let client = SolanaClient::dev_net().await.unwrap();
        let account = client
            .get_account(&Pubkey::from_str("8GjmTC5Y2z1an9mGt3BeiSkshCh6nWeBQiaai5mmhrrS").unwrap())
            .await
            .unwrap();
        println!("{:?}", account);
    }
}
