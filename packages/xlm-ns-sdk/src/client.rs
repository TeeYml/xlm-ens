use crate::errors::SdkError;
use crate::types::{RegistrationRequest, RenewalRequest, ResolutionResult};

#[derive(Debug, Clone)]
pub struct XlmNsClient {
    pub rpc_url: String,
}

impl XlmNsClient {
    pub fn new(rpc_url: impl Into<String>) -> Self {
        Self {
            rpc_url: rpc_url.into(),
        }
    }

    pub fn resolve(&self, name: &str) -> Result<ResolutionResult, SdkError> {
        Ok(ResolutionResult {
            name: name.to_string(),
            address: None,
        })
    }

    pub fn register(&self, request: RegistrationRequest) -> Result<(), SdkError> {
        if request.label.trim().is_empty() {
            return Err(SdkError::InvalidRequest("label must not be empty".into()));
        }

        Ok(())
    }

    pub fn renew(&self, request: RenewalRequest) -> Result<(), SdkError> {
        if request.additional_years == 0 {
            return Err(SdkError::InvalidRequest(
                "additional_years must be greater than zero".into(),
            ));
        }

        Ok(())
    }
}
