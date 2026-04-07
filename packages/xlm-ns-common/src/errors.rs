use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CommonError {
    #[error("name is too short")]
    NameTooShort,
    #[error("name is too long")]
    NameTooLong,
    #[error("name contains invalid characters")]
    InvalidCharacters,
    #[error("tld is not supported")]
    UnsupportedTld,
}
