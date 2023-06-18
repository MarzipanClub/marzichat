//! Error types and implementations.

use serde::{Deserialize, Serialize};

/// The various error types that can be returned when interacting with the
/// datastore.
#[derive(thiserror::Error, Debug, Serialize, Deserialize)]
pub enum Error {
    #[error("invalid table name: {0}")]
    InvalidTableName(String),

    #[error("table not found: {0}")]
    TableNotFound(String),

    #[error("invalid key: {0}")]
    InvalidKey(String),

    #[error("operation failed to complete due to shutdown")]
    Exiting,

    #[error("internal error")]
    Internal,
}

impl From<redb::Error> for Error {
    fn from(error: redb::Error) -> Self {
        tracing::error!(?error, "error from redb");
        Self::Internal
    }
}

impl From<redb::TransactionError> for Error {
    fn from(error: redb::TransactionError) -> Self {
        tracing::error!(?error, "transaction error from redb");
        Self::Internal
    }
}

impl From<redb::TableError> for Error {
    fn from(error: redb::TableError) -> Self {
        tracing::error!(?error, "table error from redb");
        Self::Internal
    }
}

impl From<redb::StorageError> for Error {
    fn from(error: redb::StorageError) -> Self {
        tracing::error!(?error, "storage error from redb");
        Self::Internal
    }
}
