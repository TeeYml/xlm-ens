use crate::config::Network;
use xlm_ns_sdk::client::XlmNsClient;

pub fn run_resolve(network: Network, name: &str) {
    let client = XlmNsClient::new(match network {
        Network::Testnet => "https://soroban-testnet.example",
        Network::Mainnet => "https://soroban-mainnet.example",
    });

    let result = client
        .resolve(name)
        .expect("resolution should not fail in scaffold");
    println!("{} -> {:?}", result.name, result.address);
}
