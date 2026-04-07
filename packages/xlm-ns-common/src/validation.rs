use crate::constants::{MAX_NAME_LENGTH, MIN_NAME_LENGTH};
use crate::errors::CommonError;

pub fn validate_label(label: &str) -> Result<(), CommonError> {
    let len = label.len();

    if len < MIN_NAME_LENGTH {
        return Err(CommonError::NameTooShort);
    }

    if len > MAX_NAME_LENGTH {
        return Err(CommonError::NameTooLong);
    }

    if !label
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
    {
        return Err(CommonError::InvalidCharacters);
    }

    Ok(())
}
