use {
    crate::types::validation::{Validate, Validator, Violations},
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    std::fmt,
    zeroize::{Zeroize, ZeroizeOnDrop},
};

const MIN_ENTROPY_SCORE: u8 = 3;

/// A password.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Serialize, Deserialize, From)]
#[from(forward)]
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct Password(pub String);

impl Password {
    pub const MAX_BYTES: usize = 64;
    pub const MIN_BYTES: usize = 8;
}

impl fmt::Debug for Password {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let redacted = "•".repeat(self.0.len());
        f.debug_tuple("Password").field(&redacted).finish()
    }
}

#[derive(thiserror::Error, Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Display, Hash)]
pub enum Violation {
    TooLong,
    TooShort,
    TooSimple,
}

impl Validate for Password {
    type Violation = Violation;

    fn validate(&self) -> Result<(), Violations<Self::Violation>> {
        let entropy_too_low = match zxcvbn::zxcvbn(&self.0, &[]) {
            Ok(entropy) => entropy.score() < MIN_ENTROPY_SCORE,
            Err(_) => false, // if we get any errors here, the entropy is still too low
        };
        Validator::new()
            .invalid_if(self.0.len() > Self::MAX_BYTES, Violation::TooLong)
            .invalid_if(self.0.len() < Self::MIN_BYTES, Violation::TooShort)
            .invalid_if(entropy_too_low, Violation::TooSimple)
            .into()
    }
}
