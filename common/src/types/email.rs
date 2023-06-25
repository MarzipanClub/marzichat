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

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display, Hash)]
pub enum Violation {
    TooLong,
    Invalid,
}

impl Validate for Email {
    type Violation = Violation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        Validator::new()
            .invalid_if(self.0.len() > Self::MAX_BYTES, Violation::TooLong)
            .invalid_if(!mailchecker::is_valid(&self.0), Violation::Invalid)
            .into()
    }
}
