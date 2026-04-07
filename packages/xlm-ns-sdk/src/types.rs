#[derive(Debug, Clone)]
pub struct RegistrationRequest {
    pub label: String,
    pub owner: String,
}

#[derive(Debug, Clone)]
pub struct RenewalRequest {
    pub name: String,
    pub additional_years: u32,
}

#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub name: String,
    pub address: Option<String>,
}
