use near_jsonrpc_client::{JsonRpcClient, methods};
use near_primitives::types::{Finality, FunctionArgs};
use near_primitives::views::QueryRequest;
use serde_json::Value;
use std::sync::Arc;

use crate::qx_sender::QuerySender;

pub struct NearQxSender {
    query_sender: QuerySender,
    account_id_receiver: String,
    method_name: String,
    args: Value,
}

pub struct NearQxSenderBuilder {
    rpc_url: String,
    account_id_sender: Option<String>,
    account_id_receiver: Option<String>,
    method_name: Option<String>,
    args: Option<Value>,
}

impl NearQxSenderBuilder {
    pub fn new(rpc_url: &str) -> Self {
        Self {
            rpc_url: rpc_url.to_string(),
            account_id_sender: None,
            account_id_receiver: None,
            method_name: None,
            args: None,
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

    pub fn method_name(mut self, method_name: &str) -> Self {
        self.method_name = Some(method_name.to_string());
        self
    }

    pub fn args(mut self, args: Value) -> Self {
        self.args = Some(args);
        self
    }

    pub fn build(self) -> Result<NearQxSender, Box<dyn std::error::Error + Send + Sync>> {
        let client = Arc::new(JsonRpcClient::connect(&self.rpc_url));
        
        let account_id_receiver = self.account_id_receiver
            .ok_or("account_id_receiver is required")?;
        
        let method_name = self.method_name
            .ok_or("method_name is required")?;

        Ok(NearQxSender {
            query_sender: QuerySender::new(client),
            account_id_receiver,
            method_name,
            args: self.args.unwrap_or(Value::Null),
        })
    }
}

impl NearQxSender {
    pub fn builder(rpc_url: &str) -> NearQxSenderBuilder {
        NearQxSenderBuilder::new(rpc_url)
    }

    pub async fn send_query(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let request = methods::query::RpcQueryRequest {
            block_reference: near_primitives::types::BlockReference::Finality(Finality::Final),
            request: QueryRequest::CallFunction {
                account_id: self.account_id_receiver.parse()?,
                method_name: self.method_name.clone(),
                args: FunctionArgs::from(serde_json::to_vec(&self.args)?),
            },
        };

        self.query_sender.send_query(request).await
    }
} 