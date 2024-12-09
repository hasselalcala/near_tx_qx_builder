use near_crypto::InMemorySigner;
use near_jsonrpc_client::{methods, JsonRpcClient};
use near_jsonrpc_primitives::types::transactions::RpcTransactionResponse;
use serde_json::Value;
use std::sync::Arc;
use std::time::Duration;
use near_primitives::views::TxExecutionStatus;

use crate::nonce_manager::NonceManager;
use crate::tx_builder::TxBuilder;
use crate::tx_sender::TxSender;

pub struct NearTxSender {
    signer: Arc<InMemorySigner>,
    nonce_manager: NonceManager,
    tx_sender: TxSender,
    tx_builder: TxBuilder,
}

pub struct NearTxSenderBuilder {
    rpc_url: String,
    account_id_sender: Option<String>,
    account_id_receiver: Option<String>,
    method_name: Option<String>,
    args: Option<Value>,
    private_key: Option<String>,
    timeout: Duration,
}

impl NearTxSenderBuilder {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            account_id_sender: None,
            account_id_receiver: None,
            method_name: None,
            args: None,
            private_key: None,
            timeout: Duration::from_secs(60),
        }
    }

    pub fn account_sender(mut self, account_id: &str) -> Self {
        self.account_id_sender = Some(account_id.to_string());
        self
    }

    pub fn account_receiver(mut self, account_id: &str) -> Self {
        self.account_id_receiver = Some(account_id.to_string());
        self
    }

    pub fn use_private_key(mut self, private_key: &str) -> Self {
        self.private_key = Some(private_key.to_string());
        self
    }

    pub fn method_name(mut self, method_name: &str) -> Self {
        self.method_name = Some(method_name.to_string());
        self
    }

    pub fn args(mut self, args: Value) -> Self {
        self.args = Some(args);
        self
    }

    pub fn build(self) -> Result<NearTxSender, Box<dyn std::error::Error + Send + Sync>> {
        let client = Arc::new(JsonRpcClient::connect(&self.rpc_url));

        let account_id_sender = self
            .account_id_sender
            .ok_or("account_id_sender is required")?;

        let account_id_receiver = self
            .account_id_receiver
            .ok_or("account_id_receiver is required")?;

        let signer = if let Some(private_key) = self.private_key {
            Arc::new(InMemorySigner::from_secret_key(
                account_id_sender.parse()?,
                private_key.parse()?,
            ))
        } else {
            return Err("private_key is required for transactions".into());
        };

        let nonce_manager = NonceManager::new(client.clone(), signer.clone());
        let tx_sender = TxSender::new(client.clone(), self.timeout);

        let mut tx_builder = TxBuilder::new(
            signer.as_ref().clone(),
            account_id_receiver.parse()?,
        );

        if let Some(method_name) = self.method_name {
            tx_builder.with_method_name(&method_name);
        }

        if let Some(args) = self.args {
            tx_builder.with_args(args);
        }

        Ok(NearTxSender {
            //client,
            signer,
            nonce_manager,
            tx_sender,
            tx_builder,
        })
    }
}

impl NearTxSender {
    pub fn builder(rpc_url: &str) -> NearTxSenderBuilder {
        NearTxSenderBuilder::new(rpc_url)
    }

    pub async fn send_transaction(
        &self,
    ) -> Result<RpcTransactionResponse, Box<dyn std::error::Error + Send + Sync>> {
        let (nonce, block_hash) = self.nonce_manager.get_nonce_and_tx_hash().await?;

        let (transaction, _) = self.tx_builder.build(nonce, block_hash);
        let signed_transaction = transaction.sign(self.signer.as_ref());

        let request = methods::send_tx::RpcSendTransactionRequest {
            signed_transaction,
            wait_until: TxExecutionStatus::Final,
        };

        self.tx_sender.send_transaction(request).await
    }
}
