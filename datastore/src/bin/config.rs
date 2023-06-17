//! # Config module
//!
//! This module contains the configuration setup for the gateway.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::Result,
    common::utils::config::parse,
    serde::Deserialize,
    std::{
        num::{NonZeroU32, NonZeroU64},
        sync::OnceLock,
    },
};

static CONFIG: OnceLock<Config> = OnceLock::new();

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

/// Initialize the config.
#[deny(dead_code)]
pub fn init() -> Result<()> {
    CONFIG
        .set(parse::<Config>(env!("CARGO_PKG_NAME"))?)
        .expect("config already initialized");
    Ok(())
}
