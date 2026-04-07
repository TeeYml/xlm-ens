use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum CommonError {
    NameTooShort,
    NameTooLong,
    InvalidCharacters,
    InvalidLabelBoundary,
    UnsupportedTld,
    MissingTld,
    InvalidName,
    EmptyOwner,
    InvalidRegistrationPeriod,
    EmptyChainName,
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NameTooShort => "name is too short",
            Self::NameTooLong => "name is too long",
            Self::InvalidCharacters => "name contains invalid characters",
            Self::InvalidLabelBoundary => "name label cannot start or end with a hyphen",
            Self::UnsupportedTld => "tld is not supported",
            Self::MissingTld => "name must include a supported tld",
            Self::InvalidName => "name is malformed",
            Self::EmptyOwner => "owner must not be empty",
            Self::InvalidRegistrationPeriod => "registration period is outside the supported range",
            Self::EmptyChainName => "chain name must not be empty",
        };

        f.write_str(message)
    }
}

impl std::error::Error for CommonError {}
