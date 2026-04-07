use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistrationRequest {
    pub label: String,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewalRequest {
    pub name: String,
    pub additional_years: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolutionResult {
    pub name: String,
    pub address: Option<String>,
}
