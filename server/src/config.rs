//! # Config module
//!
//! This module contains the configuration setup for the server.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::{Context, Result},
    once_cell::sync::OnceCell,
    serde::Deserialize,
    std::{path::PathBuf, str::FromStr},
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
        std::env::var(name).with_context(|| format!("Missing environment variable: {name}"))?;
    value
        .parse()
        .with_context(|| format!("Invalid value for environment variable: {name}"))
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
