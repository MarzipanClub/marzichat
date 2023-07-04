use {
    crate::postgres,
    anyhow::{anyhow, Result},
    argon2::{
        password_hash::SaltString, Algorithm, Argon2, ParamsBuilder, PasswordHasher, Version,
    },
    common::{
        internationalization::Language,
        types::{validation::Validate, AccountId, Email, Password, Username},
    },
    rand::rngs::OsRng,
    serde::{Deserialize, Serialize},
    uuid::Uuid,
};

/// Creates an account.
pub async fn create_account(
    username: Username,
    email: Email,
    password: Password,
    language: Language,
) -> Result<AccountId> {
    username.validate()?;
    email.validate()?;
    password.validate()?;

    let account_id = Uuid::new_v4().into();

    let phc_string = phc_string(
        password,
        AssociatedData {
            account_id,
            username: username.clone(),
            email: email.clone(),
        },
    )
    .map_err(|error| {
        tracing::error!(
            email = email.0,
            %error,
            "failed to create phc_string"
        );
        anyhow!("failed to create phc_string")
    })?;

    postgres::create_account(account_id, &username, &email, &phc_string, language).await?;
    Ok(account_id)
}

/// The associated data for hashing a password.
/// Binds the password to the account id, username and email.
#[derive(Debug, Serialize, Deserialize)]
struct AssociatedData {
    account_id: AccountId,
    username: Username,
    email: Email,
}

/// Hash the password as a [phc string](https://github.com/P-H-C/phc-string-format/blob/master/phc-sf-spec.md)
fn phc_string(password: Password, associated_data: AssociatedData) -> Result<String> {
    let data = argon2::AssociatedData::new(&bincode::serialize(&associated_data)?)
        .map_err(|error| anyhow!("failed to create password associated data: {error:?}"))?;

    // params based on https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html
    // for less memory but more cpu usage
    let params = ParamsBuilder::new()
        .m_cost(7 * 1024)
        .t_cost(5)
        .p_cost(1)
        .data(data)
        .build()
        .map_err(|error| anyhow!("failed to create password hashing params: {error:?}"))?;

    let password_phc = Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
        .hash_password(password.0.as_bytes(), &SaltString::generate(&mut OsRng))
        .map_err(|error| anyhow!("failed to hash password: {error:?}"))?
        .to_string();

    Ok(password_phc)
}
