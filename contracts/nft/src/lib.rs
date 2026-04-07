pub mod mint;
pub mod test;
pub mod transfer;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRecord {
    pub owner: String,
    pub approved: Option<String>,
    pub metadata_uri: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NftError {
    AlreadyMinted,
    NotFound,
    Unauthorized,
}

impl core::fmt::Display for NftError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::AlreadyMinted => f.write_str("token is already minted"),
            Self::NotFound => f.write_str("token was not found"),
            Self::Unauthorized => f.write_str("caller is not authorized for this token"),
        }
    }
}

impl std::error::Error for NftError {}

#[derive(Debug, Default)]
pub struct NftContract {
    tokens: HashMap<String, TokenRecord>,
}

impl NftContract {
    pub fn owner_of(&self, token_id: &str) -> Option<&str> {
        self.tokens.get(token_id).map(|record| record.owner.as_str())
    }

    pub fn token(&self, token_id: &str) -> Option<&TokenRecord> {
        self.tokens.get(token_id)
    }
}
