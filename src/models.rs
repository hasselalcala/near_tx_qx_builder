#[derive(Clone)]
pub enum Network {
    Mainnet,
    Testnet,
}

impl Network {
    pub fn from_rpc_url(url: &str) -> Self {
        if url.contains("testnet") {
            Network::Testnet
        } else {
            Network::Mainnet
        }
    }
}
