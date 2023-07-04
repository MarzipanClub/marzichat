//! Module to generate a suggested username.

//! Module implement the username api.

use {
    crate::postgres,
    anyhow::{anyhow, Result},
    common::types::{validation::Validate, Username},
    names::{Generator, Name},
};

/// Generates a suggested username.
pub fn suggested() -> Result<Username> {
    let mut generator = Generator::with_naming(Name::Numbered);

    loop {
        let username = Username(
            generator
                .next()
                .ok_or(anyhow!("failed to generate a username"))?,
        );
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
