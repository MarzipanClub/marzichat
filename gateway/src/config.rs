//! # Config module
//!
//! This module contains the configuration setup for the gateway.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::{Context, Result},
    hex::FromHex,
    serde::{de::DeserializeOwned, Deserialize, Deserializer},
    std::{
        num::{NonZeroU32, NonZeroU64},
        path::PathBuf,
        str::FromStr,
        sync::OnceLock,
    },
};

static CONFIG: OnceLock<Config> = OnceLock::new();

const COOKIE_SIGNING_KEY_LENGTH_BYTES: usize = 64;

pub type CookieSigningKey = [u8; COOKIE_SIGNING_KEY_LENGTH_BYTES];

/// The root configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// The logging directive to use.
    /// Acceptable directive must must follow
    /// [this][1] format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    pub logging_directive: String,

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
        .set(parse::<Config>(env!("CARGO_PKG_NAME"))?)
        .expect("config already initialized");
    Ok(())
}

/// Parse the given environment variable as a generic type T.
pub fn env_var<T>(name: &str) -> Result<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
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
pub fn parse<T: DeserializeOwned>(cargo_package_name: &str) -> Result<T> {
    let config_env_var_name = format!("{}_CONFIG", cargo_package_name.to_ascii_uppercase());

    let path = env_var::<PathBuf>(&config_env_var_name)?;

    let config = ron::from_str(
        &std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {path:?}"))?,
    )
    .context("failed to parse config file")?;
    Ok(config)
}

fn cookie_signing_key_from_str<'de, D>(deserializer: D) -> Result<CookieSigningKey, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: String = Deserialize::deserialize(deserializer)?;
    <CookieSigningKey>::from_hex(hex).map_err(serde::de::Error::custom)
}
