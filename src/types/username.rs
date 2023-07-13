use {
    crate::types::validation::{Validate, Validator, Violations},
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
};

/// A username.

#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, From, Display,
)]
#[from(forward)]
pub struct Username(pub String);

impl Username {
    /// The maximum number of bytes a username can be.
    // Don't change without updating the accounts table contraints.
    pub const MAX_BYTES: usize = 24;
    /// The minimum number of bytes a username can be.
    //  Don't change without updating the accounts table contraints.
    pub const MIN_BYTES: usize = 5;
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display, Hash)]
pub enum Violation {
    TooLong,
    TooShort,
    Invalid,
}

/// Validate a username.
pub fn validate(username: &str) -> Result<(), Violations<Violation>> {
    let all_chars_valid = username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
    Validator::new()
        .invalid_if(username.len() > Username::MAX_BYTES, Violation::TooLong)
        .invalid_if(username.len() < Username::MIN_BYTES, Violation::TooShort)
        .invalid_if(!all_chars_valid, Violation::Invalid)
        .into()
}

impl Validate for Username {
    type Violation = Violation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        validate(&self.0)
    }
}
