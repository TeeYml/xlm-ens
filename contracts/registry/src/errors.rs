use core::fmt;
use xlm_ns_common::CommonError;

#[derive(Debug)]
pub enum RegistryError {
    AlreadyRegistered,
    NotFound,
    Validation(CommonError),
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered => f.write_str("name is already registered"),
            Self::NotFound => f.write_str("name was not found"),
            Self::Validation(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for RegistryError {}

impl From<CommonError> for RegistryError {
    fn from(value: CommonError) -> Self {
        Self::Validation(value)
    }
}
