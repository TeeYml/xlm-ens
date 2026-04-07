use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CommonError {
    NameTooShort,
    NameTooLong,
    InvalidCharacters,
    UnsupportedTld,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NameTooShort => "name is too short",
            Self::NameTooLong => "name is too long",
            Self::InvalidCharacters => "name contains invalid characters",
            Self::UnsupportedTld => "tld is not supported",
        };

        f.write_str(message)
    }
}

impl std::error::Error for CommonError {}
