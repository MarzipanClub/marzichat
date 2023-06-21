use {
    crate::{
        internationalization::Language,
        types::{DateTime, Email, Username},
    },
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
};

/// An account id.
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize, From, Display, Hash,
)]
#[from(forward)]
pub struct Id(pub uuid::Uuid);

/// An account.
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, From, Hash)]
pub struct Account {
    pub id: Id,
    pub created: DateTime,
    pub updated: DateTime,
    pub username: Username,
    pub email: Email,
    pub language: Language,
}
