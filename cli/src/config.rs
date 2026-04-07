#[derive(Debug, Clone, Copy)]
pub enum Network {
    Testnet,
    Mainnet,
}

impl Network {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "testnet" => Some(Self::Testnet),
            "mainnet" => Some(Self::Mainnet),
            _ => None,
        }
    }
}
