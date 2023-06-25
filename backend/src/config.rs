//! # Config module
//!
//! This module contains the configuration setup for the backend.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::{Context, Result},
    derive_more::From,
    hex::FromHex,
    serde::{de::DeserializeOwned, Deserialize, Deserializer},
    std::{
        error::Error,
        num::{NonZeroU32, NonZeroU64},
        path::PathBuf,
        str::FromStr,
        sync::OnceLock,
    },
    zeroize::{Zeroize, ZeroizeOnDrop},
};

static CONFIG: OnceLock<Config> = OnceLock::new();

const COOKIE_SIGNING_KEY_LENGTH_BYTES: usize = 64;
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct CookieSigningKey(pub [u8; COOKIE_SIGNING_KEY_LENGTH_BYTES]);

#[derive(Zeroize, ZeroizeOnDrop, From)]
#[from(forward)]
pub struct PostgresConnectionUrl(pub String);

/// The root configuration.
#[derive(Deserialize)]
pub struct Config {
    /// The logging directives to use.
    /// Acceptable directive must must follow
    /// [this][1] format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    pub logging_directives: String,

    /// The interval after which one element of the quota is replenished in
    /// seconds for each ip address.
    pub rate_limit_interval_per_second: NonZeroU64,

    /// The quota size that defines how many requests for a given ip address can
    /// occur before the governor middleware starts blocking requests from
    /// an IP address and clients have to wait until the elements of the
    /// quota are replenished.
    pub rate_limit_burst_size: NonZeroU32,

    /// The path to the static assets to serve.
    /// This is where the favicons directory and css file should be located.
    pub static_assets_path: PathBuf,

    /// The secret key to use for signing cookies.
    ///
    /// Must be 64 bytes long and encoded as hex.
    /// Use `openssl rand -hex 64` to generate a new key.
    #[serde(deserialize_with = "cookie_signing_key_from_str")]
    pub cookie_signing_key: CookieSigningKey,

    /// The maximum number of active postgres connections to pool.
    pub max_postgres_connection_pool_size: u32,

    /// The postgres connection url.
    #[serde(deserialize_with = "postgres_connection_url_from_str")]
    pub postgres_connection_url: PostgresConnectionUrl,
}

/// Returns the global configuration for the server.
#[inline]
pub fn get() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

/// Initialize the config.
#[deny(dead_code)]
pub fn init() -> Result<()> {
    CONFIG
        .set(parse(env!("CARGO_PKG_NAME"))?)
        .unwrap_or_else(|_| panic!("config already initialized"));
    Ok(())
}

/// Parse the given environment variable as a generic type T.
fn env_var<T>(name: &str) -> Result<T>
where
    T: FromStr,
    T::Err: Error + Send + Sync + 'static,
{
    let value =
        std::env::var(name).with_context(|| format!("missing environment variable: {name}"))?;
    value
        .parse()
        .with_context(|| format!("invalid value for environment variable: {name}"))
}

/// Returns a T from the config file path defined by the environmental variable:
/// `<cargo_package_name>_CONFIG` in all caps.
/// ```no_run
/// # use common::utils::config::parse;
/// # #[derive(serde::Deserialize)]
/// # struct Config;
/// # fn main() -> anyhow::Result<()> {
/// let config: Config = config_for_package(env!("CARGO_PKG_NAME"))?;
/// # Ok(())
/// # }
/// ```
fn parse<T: DeserializeOwned>(cargo_package_name: &str) -> Result<T> {
    let config_env_var_name = format!("{}_CONFIG", cargo_package_name.to_ascii_uppercase());

    let path = env_var::<PathBuf>(&config_env_var_name)?;

    let config = ron::from_str(
        &std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {path:?}"))?,
    )
    .context("failed to parse config file")?;
    Ok(config)
}

fn postgres_connection_url_from_str<'de, D>(
    deserializer: D,
) -> Result<PostgresConnectionUrl, D::Error>
where
    D: Deserializer<'de>,
{
    let url: String = Deserialize::deserialize(deserializer)?;
    Ok(PostgresConnectionUrl(url))
}

fn cookie_signing_key_from_str<'de, D>(deserializer: D) -> Result<CookieSigningKey, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: String = Deserialize::deserialize(deserializer)?;
    let bytes =
        <[u8; COOKIE_SIGNING_KEY_LENGTH_BYTES]>::from_hex(hex).map_err(serde::de::Error::custom)?;

    Ok(CookieSigningKey(bytes))
}
