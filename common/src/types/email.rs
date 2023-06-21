use {
    crate::types::validation::{Invalidities, Validate, Validator},
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
};

/// An email.
#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Clone,
    Hash,
    Serialize,
    Deserialize,
    From,
    Display,
)]
#[from(forward)]
pub struct Email(pub String);

impl Email {
    pub const MAX_BYTES: usize = 64;
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display)]
pub enum Invalidity {
    TooLong,
    Invalid,
}

impl Validate for Email {
    type Invalidity = Invalidity;

    fn validate(&self) -> Result<(), Invalidities<Self::Invalidity>> {
        Validator::new()
            .invalid_if(self.0.len() > Self::MAX_BYTES, Invalidity::TooLong)
            .invalid_if(!mailchecker::is_valid(&self.0), Invalidity::Invalid)
            .into()
    }
}
