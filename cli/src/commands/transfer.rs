use crate::config::Network;

pub fn run_transfer(network: Network, name: &str, new_owner: &str) {
    let environment = match network {
        Network::Testnet => "testnet",
        Network::Mainnet => "mainnet",
    };

    println!("transfer {name} on {environment} to {new_owner}");
}
