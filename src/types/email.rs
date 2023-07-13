use {
    crate::types::validation::{Validate, Validator, Violations},
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
};

/// An email.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, From, Display,
)]
#[from(forward)]
pub struct Email(pub String);

impl Email {
    pub const MAX_BYTES: usize = 64;
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display, Hash)]
pub enum Violation {
    TooLong,
    Invalid,
}

/// Validate an email.
pub fn validate(email: &str) -> Result<(), Violations<Violation>> {
    Validator::new()
        .invalid_if(email.len() > Email::MAX_BYTES, Violation::TooLong)
        .invalid_if(!mailchecker::is_valid(email), Violation::Invalid)
        .into()
}

impl Validate for Email {
    type Violation = Violation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        validate(&self.0)
    }
}
