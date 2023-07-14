//! Backend configuration module. This module contains all the config parameters
//! for the backend in one place.
//!
//! The configuration is parsed from `.ron` file. Ron is to Rust what JSON is to
//! JavaScript.
#![cfg(feature = "ssr")]

use {
    anyhow::{Context, Result},
    sentry::types::Dsn,
    serde::{Deserialize, Deserializer},
    std::{
        fs::File,
        io::BufReader,
        num::{NonZeroU32, NonZeroU64, NonZeroUsize},
        path::{Path, PathBuf},
    },
    tracing_subscriber::EnvFilter,
};

pub type TlsConfig = rustls::ServerConfig;

/// The root configuration.
// dont' derive Debug to avoid leaking secrets
#[derive(Deserialize)]
pub struct Config {
    pub logging: LoggingConfig,
    pub postgres: PostgresConfig,
    pub server: ServerConfig,
    pub io_threads: NonZeroUsize,
    pub cpu_threads: NonZeroUsize,
}

/// Logging filters and system metric warning configuration.
#[derive(Deserialize)]
pub struct LoggingConfig {
    /// Logging directives; acceptable directive must must follow [this][1]
    /// format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    #[serde(deserialize_with = "parse_env_filter")]
    pub directives: EnvFilter,

    /// The sentry dsn.
    pub sentry_data_source_name: Option<Dsn>,

    /// The interval in seconds to log machine stats.
    pub machine_stats_interval_seconds: NonZeroU64,

    /// The cpu temperature in celsius above which a warning is logged.
    pub cpu_temp_warn_threshold_celsius: f32,

    /// The memory usage percentage above which a warning is logged.
    pub system_memory_usage_warn_threshold_percentage: f64,

    /// The cpu load average above which a warning is logged.
    pub cpu_load_warn_limit: f32,
}

/// The postgres configuration.
#[derive(Deserialize)]
pub struct PostgresConfig {
    /// Set the maximum number of connections that the postgres pool should
    /// maintain.
    pub max_connections: NonZeroU32,

    /// The postgres database url to connect to.
    pub url: String,
}

/// The server configuration.
#[derive(Deserialize)]
pub struct ServerConfig {
    pub rate_limiter: RateLimiterConfig,
    #[serde(deserialize_with = "parse_tls_config")]
    pub tls: Option<TlsConfig>,
}

/// The rate limiter configuration.
#[derive(Deserialize, Clone)]
pub struct RateLimiterConfig {
    /// The burst size for rate limiting.
    ///
    /// Sets the quota size that defines how many requests can occur before the
    /// governor middleware starts blocking requests from an IP address and
    /// clients have to wait until the elements of the quota are replenished.
    pub burst_size: NonZeroU32,

    /// The replenish rate for rate limiting.
    ///
    /// Sets the interval (in seconds) after which one element of the quota is
    /// replenished.
    pub replenish_interval_seconds: NonZeroU64,
}

/// The paths to the tls certificate and private key.
#[derive(Deserialize)]
pub struct TlsCertPaths {
    /// The filepath to the tls certificate.
    pub cert: PathBuf,

    /// The filepath to the tls private key.
    pub cert_key: PathBuf,
}

/// Parses the config.
pub fn parse(config: &Path) -> Result<Config> {
    Ok(ron::from_str(
        &std::fs::read_to_string(config)
            .with_context(|| format!("failed to read config file {config:?}"))?,
    )
    .context("failed to parse config file")?)
}

fn parse_env_filter<'de, D>(deserializer: D) -> Result<EnvFilter, D::Error>
where
    D: Deserializer<'de>,
{
    let directives: String = Deserialize::deserialize(deserializer)?;
    Ok(EnvFilter::builder()
        .parse(directives)
        .map_err(serde::de::Error::custom)?)
}

fn parse_tls_config<'de, D>(deserializer: D) -> Result<Option<TlsConfig>, D::Error>
where
    D: Deserializer<'de>,
{
    let tls: Option<TlsCertPaths> = Deserialize::deserialize(deserializer)?;
    tls.map(|tls| create_tls(&tls.cert, &tls.cert_key).map_err(serde::de::Error::custom))
        .transpose()
}

fn create_tls(cert: &Path, cert_key: &Path) -> Result<TlsConfig> {
    let open = |path| File::open(path).with_context(|| format!("error opening {path:?}"));
    let cert_chain = rustls_pemfile::certs(&mut BufReader::new(open(cert)?))
        .context("couldn't parse cert")?
        .into_iter()
        .map(rustls::Certificate)
        .collect();
    let key = {
        let mut keys = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(open(cert_key)?))
            .context("couldn't parse cert key")?;
        anyhow::ensure!(!keys.is_empty(), "no cert key found");
        rustls::PrivateKey(keys.swap_remove(0))
    };
    TlsConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .context("couldn't create certificate chain")
}
