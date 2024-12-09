use near_tx_qx_builder::{NearTxSender, NearQuerySender};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    // Transactions 
    let tx_sender = NearTxSender::builder("https://rpc.testnet.near.org")
    .account_sender("sender_account_id.testnet")
    .use_private_key("ed25519:...")
    .account_receiver("receiver_account_id.testnet")
    .method_name("my_method")
    .args(json!({ "param1": "value" }))
    .build()?;

    let result = tx_sender.send_transaction().await?;
    println!("Transaction result: {:?}", result);

    // Queries
    let query_sender = NearQxSender::builder("https://rpc.testnet.near.org")
    .account_sender("sender_account_id.testnet")
    .account_receiver("receiver_account_id.testnet")
    .method_name("my_method")
    .build()?;

    let query_result = query_sender.query().await?;
    println!("Query result: {}", query_result);

    Ok(())
}