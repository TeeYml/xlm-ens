use crate::config::Network;
use xlm_ns_sdk::client::XlmNsClient;
use xlm_ns_sdk::types::RenewalRequest;

pub fn run_renew(network: Network, name: &str, years: u64) {
    let client = XlmNsClient::new(match network {
        Network::Testnet => "https://soroban-testnet.example",
        Network::Mainnet => "https://soroban-mainnet.example",
    });

    let _ = client.renew(RenewalRequest {
        name: name.into(),
        additional_years: years as u32,
    });

    println!("renewed {name} for {years} year(s)");
}
