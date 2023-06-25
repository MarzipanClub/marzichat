//! Account handlers.

use {
    axum::{
        extract::{rejection::JsonRejection, Path},
        response::IntoResponse,
    },
    common::types::{
        account::{self, Account},
        DateTime,
    },
    hyper::StatusCode,
};

// We create our own rejection type
#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    message: String,
}

// We implement `From<JsonRejection> for ApiError`
impl From<JsonRejection> for ApiError {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status: rejection.status(),
            message: rejection.body_text(),
        }
    }
}

/// Handler for getting an account.
pub async fn get(Path(account_id): Path<account::Id>) -> impl IntoResponse {
    let account = Account {
        id: account_id,
        created: chrono::Utc::now(),
        updated: chrono::Utc::now(),
        username: "alice".into(),
        email: "email".into(),
        password_hash: "password_hahs".into(),
        language: "en".try_into().unwrap(),
    };
    (StatusCode::OK)
}
