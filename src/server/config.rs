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
const CONFIG_ENV_VAR_NAME: &str = "SERVER_CONFIG";

/// The root configuration.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Enable custom logging; acceptable directive must must follow
    /// [this][1] format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    pub logging_directive: String,

    /// Set the interval after which one element of the quota is replenished in
    /// seconds for each ip address.
    pub rate_limit_interval_per_second: NonZeroU64,

    /// Set quota size that defines how many requests for a given ip address can
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
    let path = env_var::<PathBuf>(CONFIG_ENV_VAR_NAME)?;

    let config = ron::from_str(
        &std::fs::read_to_string(&path)
            .with_context(|| format!("failed to read config file {path:?}"))?,
    )
    .context("failed to parse config file")?;

    CONFIG.set(config).expect("config already initialized");
    Ok(())
}
