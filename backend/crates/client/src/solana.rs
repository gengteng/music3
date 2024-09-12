//! # Solana client

use futures::future::BoxFuture;
use futures::stream::BoxStream;
use solana_client::client_error::ClientError;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::pubsub_client::PubsubClientError;
use solana_client::rpc_config::RpcProgramAccountsConfig;
use solana_client::rpc_response::RpcKeyedAccount;
use solana_client::tpu_client::TpuSenderError;
use solana_quic_client::{QuicConfig, QuicConnectionManager, QuicPool};
use solana_rpc_client_api::response::Response;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;

/// Solana client
pub struct SolanaClient {
    inner: solana_client::nonblocking::tpu_client::TpuClient<
        QuicPool,
        QuicConnectionManager,
        QuicConfig,
    >,
    pubsub: solana_client::nonblocking::pubsub_client::PubsubClient,
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
        let pub_sub = solana_client::nonblocking::pubsub_client::PubsubClient::new(
            "wss://api.devnet.solana.com",
        )
        .await?;
        Ok(Self {
            inner: solana_client,
            pubsub: pub_sub,
        })
    }

    /// Get account
    pub async fn get_account(&self, pub_key: &Pubkey) -> Result<Account, ClientError> {
        self.inner.rpc_client().get_account(pub_key).await
    }

    /// Subscribe to program account events.
    pub async fn program_subscribe(
        &self,
        pub_key: &Pubkey,
        config: impl Into<Option<RpcProgramAccountsConfig>>,
    ) -> Result<
        (
            BoxStream<'_, Response<RpcKeyedAccount>>,
            Box<dyn FnOnce() -> BoxFuture<'static, ()> + Send>,
        ),
        PubsubClientError,
    > {
        self.pubsub.program_subscribe(pub_key, config.into()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;
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

    #[tokio::test]
    async fn subscribe_program() {
        let client = SolanaClient::dev_net().await.unwrap();
        let (mut stream, cancel) = client
            .program_subscribe(
                &Pubkey::from_str("11111111111111111111111111111111").unwrap(),
                None,
            )
            .await
            .unwrap();
        if let Some(response) = stream.next().await {
            println!("{:?}", response.value);
        }
        cancel().await;
    }
}
