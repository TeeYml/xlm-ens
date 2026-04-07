use crate::config::Network;
use xlm_ns_sdk::client::XlmNsClient;
use xlm_ns_sdk::types::RegistrationRequest;

pub fn run_register(network: Network, label: &str, owner: &str) {
    let client = XlmNsClient::new(match network {
        Network::Testnet => "https://soroban-testnet.example",
        Network::Mainnet => "https://soroban-mainnet.example",
    });

    let _ = client.register(RegistrationRequest {
        label: label.into(),
        owner: owner.into(),
    });

    println!("submitted register request for {label}.xlm to {owner}");
}
