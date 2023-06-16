//! # Config module
//!
//! This module contains the configuration setup for the server.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::{Context, Result},
    once_cell::sync::OnceCell,
    serde::Deserialize,
    std::{
        num::{NonZeroU32, NonZeroU64},
        path::PathBuf,
        str::FromStr,
    },
};

static CONFIG: OnceCell<Config> = OnceCell::new();

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
}

/// Returns the global configuration for the server.
#[inline]
pub fn get() -> &'static Config {
    CONFIG.get().expect("config not initialized")
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

/// Initialize the config.
#[deny(dead_code)]
pub fn init() -> Result<()> {
    let config_env_var_name = format!("{}_CONFIG", env!("CARGO_PKG_NAME").to_ascii_uppercase());

    let path = env_var::<PathBuf>(&config_env_var_name)?;

    let config = ron::from_str(
        &std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {path:?}"))?,
    )
    .context("failed to parse config file")?;

    CONFIG.set(config).expect("config already initialized");
    Ok(())
}
