//! Module to generate a suggested username.

//! Module implement the username api.

use {
    crate::postgres,
    anyhow::Result,
    common::types::{
        username,
        validation::{Validate, Violations},
        Username,
    },
    names::{Generator, Name},
};

/// A username error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to generate username")]
    GenerateFailure,

    #[error("username is invalid: {0}")]
    Invalid(#[from] Violations<username::Violation>),
}
/// Generates a suggested username.
pub fn suggested() -> Result<Username, Error> {
    let mut generator = Generator::with_naming(Name::Numbered);

    loop {
        let username = Username(generator.next().ok_or(Error::GenerateFailure)?);
        if username.validate().is_ok() {
            break Ok(username);
        }
    }
}
/// Checks if a username is available.
pub async fn is_available(username: &Username) -> Result<bool> {
    username.validate()?;
    Ok(postgres::is_username_available(username).await?)
}
