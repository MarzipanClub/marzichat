//! Postgres database connection and error handling.
#![cfg(feature = "ssr")]

use {
    crate::config::PostgresConfig,
    anyhow::Result,
    marzichat::{internationalization::Language, types::*},
    sqlx::{error::DatabaseError, Pool, Postgres},
    std::sync::OnceLock,
};

static POOL: OnceLock<Pool<Postgres>> = OnceLock::new();

/// Creates a postgres connection pool and runs all migrations.
#[deny(dead_code)]
pub async fn init(config: PostgresConfig) {
    let max_connections = config.max_connections.get();
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&config.url)
        .await
        .expect("unable to create postgres connection pool");

    let current_pool_size = pool.size();
    tracing::info!(current_pool_size, max_connections, "connected",);

    tracing::info!("running migrations");
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("failed to run migrations");

    POOL.set(pool)
        .expect("postgres connection pool already initialized");
}

/// Returns a reference to the postgres connection pool.
#[inline]
fn db() -> &'static Pool<Postgres> {
    POOL.get()
        .expect("postgres connection pool is not initialized")
}

/// A Postgres error.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Row not found: `{0:?}")]
    RowNotFound(sqlx::Error),

    #[error("Sqlx error: `{0:?}`")]
    Sqlx(sqlx::Error),

    #[error("Unique constraint violation: `{0:?}")]
    UniqueViolation(Box<(dyn DatabaseError)>),

    #[error("Foreign key constraint violation: `{0:?}")]
    ForeignKeyViolation(Box<(dyn DatabaseError)>),

    #[error("Integrity constraint violation: `{0:?}")]
    IntegrityViolation(Box<(dyn DatabaseError)>),

    #[error("Postgres error: `{0:?}`")]
    Other(Box<(dyn DatabaseError)>),
}

impl From<sqlx::Error> for Error {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => Error::RowNotFound(error),
            sqlx::Error::Database(error) => match error.code().as_deref() {
                // https://www.postgresql.org/docs/current/errcodes-appendix.html
                Some("23505") => Error::UniqueViolation(error),
                Some("23503") => Error::ForeignKeyViolation(error),
                Some("23000") => Error::IntegrityViolation(error),
                _ => Error::Other(error),
            },
            _ => Error::Sqlx(error),
        }
    }
}

/// Check whether the username is not associated with a user.
pub async fn is_username_available(username: &Username) -> Result<bool, Error> {
    Ok(sqlx::query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)",
        username.0
    )
    .fetch_one(db())
    .await?
    .exists
    .map_or(true, |value| !value))
}
/// Check whether the email address is not associated with a user.
pub async fn is_email_available(email: &Email) -> Result<bool, Error> {
    Ok(sqlx::query!(
        "SELECT EXISTS (SELECT 1 FROM users WHERE email = $1)",
        email.0
    )
    .fetch_one(db())
    .await?
    .exists
    .map_or(true, |value| !value))
}

/// Create a user.
pub async fn create_user(
    user_id: UserId,
    username: &Username,
    email: &Email,
    phc_string: &str,
    language: Language,
) -> Result<(), Error> {
    let now = chrono::Utc::now();
    sqlx::query!(
        "INSERT INTO users VALUES ($1, $2, $3, $4, $5, $6, $7)",
        user_id.0,
        now,
        now,
        username.0,
        email.0,
        phc_string,
        language as Language,
    )
    .execute(db())
    .await?;

    Ok(())
}
