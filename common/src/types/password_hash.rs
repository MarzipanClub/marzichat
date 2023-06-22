use {
    derive_more::From,
    serde::{Deserialize, Serialize},
    zeroize::{Zeroize, ZeroizeOnDrop},
};

/// Bytes of a hash and salted password.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, From, Hash)]
#[from(forward)]
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PasswordHash(Vec<u8>);
