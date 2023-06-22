use {
    crate::types::validation::{Invalidities, Validate, Validator},
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
    pub const MAX_BYTES: usize = 24;
    pub const MIN_BYTES: usize = 5;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display)]
pub enum Invalidity {
    TooLong,
    TooShort,
    Invalid,
}

impl Validate for Username {
    type Invalidity = Invalidity;

    fn validate(&self) -> Result<(), Invalidities<Self::Invalidity>> {
        let all_chars_valid = self
            .0
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
        Validator::new()
            .invalid_if(self.0.len() > Self::MAX_BYTES, Invalidity::TooLong)
            .invalid_if(self.0.len() < Self::MIN_BYTES, Invalidity::TooShort)
            .invalid_if(!all_chars_valid, Invalidity::Invalid)
            .into()
    }
}
