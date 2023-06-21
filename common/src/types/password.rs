use {
    crate::types::validation::{Invalidities, Validate, Validator},
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    zeroize::{Zeroize, ZeroizeOnDrop},
};

const MIN_ENTROPY_SCORE: u8 = 3;

/// A password.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, From)]
#[from(forward)]
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Password(pub String);

impl Password {
    pub const MAX_BYTES: usize = 64;
    pub const MIN_BYTES: usize = 8;
}

impl std::fmt::Debug for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let redacted = "•".repeat(self.0.len());
        f.debug_tuple("Password").field(&redacted).finish()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Deserialize, Display)]
pub enum Invalidity {
    TooLong,
    TooShort,
    TooSimple,
}

impl Validate for Password {
    type Invalidity = Invalidity;

    fn validate(&self) -> Result<(), Invalidities<Self::Invalidity>> {
        let entropy_too_low = match zxcvbn::zxcvbn(&self.0, &[]) {
            Ok(entropy) => entropy.score() < MIN_ENTROPY_SCORE,
            Err(_) => false, // if we get any errors here, the entropy is still too low
        };
        Validator::new()
            .invalid_if(self.0.len() > Self::MAX_BYTES, Invalidity::TooLong)
            .invalid_if(self.0.len() < Self::MIN_BYTES, Invalidity::TooShort)
            .invalid_if(entropy_too_low, Invalidity::TooSimple)
            .into()
    }
}
