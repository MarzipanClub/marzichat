use {
    crate::{
        internationalization::Language,
        types::{DateTime, Email, PasswordHash, Username},
    },
    derive_more::{Display, From},
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

/// An account id.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(transparent))]
#[derive(
    Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Serialize, Deserialize, From, Display, Hash,
)]
#[from(forward)]
pub struct Id(pub Uuid);

/// An account.
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, From)]
pub struct Account {
    pub id: Id,
    pub created: DateTime,
    pub updated: DateTime,
    pub username: Username,
    pub email: Email,
    pub password_hash: PasswordHash,
    pub language: Language,
}
