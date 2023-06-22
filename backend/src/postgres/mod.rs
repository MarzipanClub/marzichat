use {
    anyhow::{Context, Result},
    sqlx::error::DatabaseError,
    std::sync::OnceLock,
};

type Pool = sqlx::Pool<sqlx::Postgres>;

static POOL: OnceLock<Pool> = OnceLock::new();

/// Creates a postgres connection pool
#[deny(dead_code)]
pub async fn init() -> Result<()> {
    let config = crate::config::get();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_postgres_connection_pool_size)
        .connect(&config.postgres_connection_url)
        .await
        .context("unable to create postgres connection pool")?;

    let current_connection_count = pool.size();
    POOL.set(pool)
        .expect("postgres connection pool already initialized");

    tracing::info!(
        "Started {} connection{} to Postgres.",
        current_connection_count,
        if current_connection_count == 1 {
            ""
        } else {
            "s"
        }
    );
    Ok(())
}

pub async fn create_pool() -> Result<Pool> {
    let config = crate::config::get();

    Ok(sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_postgres_connection_pool_size)
        .connect(&config.postgres_connection_url)
        .await
        .context("unable to create postgres connection pool")?)
}

/// Returns a reference to the postgres connection pool.
pub fn pool() -> &'static Pool {
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
