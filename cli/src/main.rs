mod commands;
mod config;

use std::env;

use commands::{
    auction::run_auction, register::run_register, renew::run_renew, resolve::run_resolve,
    transfer::run_transfer,
};
use config::Network;

fn main() {
    let args: Vec<String> = env::args().collect();
    let network = args
        .windows(2)
        .find(|pair| pair[0] == "--network")
        .and_then(|pair| Network::parse(&pair[1]))
        .unwrap_or(Network::Testnet);

    let command_index = args
        .iter()
        .position(|arg| !arg.starts_with("--") && arg != &args[0])
        .unwrap_or(0);

    if command_index == 0 {
        print_usage();
        return;
    }

    match args[command_index].as_str() {
        "register" if args.len() > command_index + 2 => {
            run_register(network, &args[command_index + 1], &args[command_index + 2])
        }
        "resolve" if args.len() > command_index + 1 => {
            run_resolve(network, &args[command_index + 1])
        }
        "transfer" if args.len() > command_index + 2 => {
            run_transfer(network, &args[command_index + 1], &args[command_index + 2])
        }
        "renew" if args.len() > command_index + 2 => {
            let years = args[command_index + 2].parse::<u64>().unwrap_or(1);
            run_renew(network, &args[command_index + 1], years)
        }
        "auction" if args.len() > command_index + 2 => {
            let reserve = args[command_index + 2].parse::<u64>().unwrap_or(0);
            run_auction(network, &args[command_index + 1], reserve)
        }
        _ => print_usage(),
    }
}

fn print_usage() {
    println!(
        "usage: xlm-ns [--network testnet|mainnet] <register|resolve|transfer|renew|auction> ..."
    );
}
