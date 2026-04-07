use serde::{Deserialize, Serialize};

pub type NameHash = [u8; 32];

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Tld {
    Xlm,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NameRecord {
    pub label: String,
    pub tld: Tld,
    pub owner: String,
    pub resolver: Option<String>,
    pub ttl_seconds: u64,
}

impl NameRecord {
    pub fn fqdn(&self) -> String {
        format!("{}.{}", self.label, self.tld.as_str())
    }
}

impl Tld {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Xlm => "xlm",
        }
    }
}
