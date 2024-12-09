# NEAR Transaction and Query Builder

A robust and flexible solution for building and sending transactions and queries to the NEAR Protocol blockchain, with support for both synchronous and asynchronous operations.

## Overview

NEAR Transaction and Query Builder is a Rust library that simplifies the interaction with NEAR Protocol smart contracts. It provides an intuitive builder pattern interface for constructing and sending transactions and queries, with comprehensive error handling and timeout management.

### Key Features

* **Transaction Building**: Easy-to-use builder pattern for transaction construction
* **Query Management**: Simplified interface for contract queries
* **Nonce Management**: Automatic nonce handling for transactions
* **Flexible Configuration**: Customizable timeouts and RPC endpoints
* **Error Handling**: Robust error management with custom error types
* **Multiple Environments**: Support for MainNet, TestNet, and local development
* **Asynchronous Operations**: Built on Tokio for efficient async/await patterns

## How it Works

### Core Components

1. **Transaction Management**:
   * Transaction building with `TxBuilder`
   * Automatic nonce handling via `NonceManager`
   * Transaction sending through `TxSender`

2. **Query Management**:
   * Query construction with `QueryBuilder`
   * Efficient query execution via `QuerySender`

3. **High-level Interfaces**:
   * `NearTxSender` for transaction operations
   * `NearQxSender` for query operations

## Usage

### Transaction Example

```rust
use near_tx_qx_builder::NearTxSender;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let tx_sender = NearTxSender::builder("https://rpc.testnet.near.org")
    .account_sender("sender.testnet")
    .account_receiver("receiver.testnet")
    .use_private_key("your-private-key")
    .method_name("some_method")
    .args(json!({
    "param1": "value1",
    "param2": "value2"
    }))
    .build()?;

    let result = tx_sender.send_transaction().await?;
    println!("Transaction result: {:?}", result);
    Ok(())
}

```

### Query Example

```rust
use near_tx_qx_builder::NearQxSender;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let qx_sender = NearQxSender::builder("https://rpc.testnet.near.org")
    .account_receiver("contract.testnet")
    .method_name("view_method")
    .args(json!({
    "param1": "value1"
    }))
    .build()?;
    let result = qx_sender.send_query().await?;
    println!("Query result: {}", result);
    Ok(())
}
```


## Technical Implementation

### Core Structures


```rust

pub struct NearTxSender {
    signer: Arc<InMemorySigner>,
    nonce_manager: NonceManager,
    tx_sender: TxSender,
    tx_builder: TxBuilder,
}

pub struct NearQxSender {
    query_sender: QuerySender,
    account_id_receiver: String,
    method_name: String,
    args: Value,
}
```

## Development

### Prerequisites

* Rust 1.54 or higher
* Tokio runtime
* NEAR SDK

### Building

```bash
cargo build
```

### Testing

```bash
cargo test
```

## Security Considerations

* Secure private key handling
* Proper error handling
* Transaction timeout management
* Nonce collision prevention
* RPC connection security

## Error Handling

The library implements comprehensive error handling for:
* RPC connection issues
* Transaction failures
* Query failures
* Invalid parameters
* Timeout scenarios

## Acknowledgements

This implementation was built to facilitate easier interaction with the NEAR Protocol blockchain and simplify the development of decentralized applications.