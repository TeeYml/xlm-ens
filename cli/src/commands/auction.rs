use crate::config::Network;
use xlm_ns_auction::bid::Bid;
use xlm_ns_auction::AuctionContract;

pub fn run_auction(network: Network, name: &str, reserve: u64) {
    let mut auction = AuctionContract::default();
    let bidder = match network {
        Network::Testnet => "testnet-bidder",
        Network::Mainnet => "mainnet-bidder",
    };

    auction.place_bid(
        name,
        Bid {
            bidder: bidder.into(),
            amount: reserve,
        },
    );

    println!("created auction placeholder for {name} with reserve {reserve}");
}
