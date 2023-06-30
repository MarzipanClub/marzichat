//! # Config module
//!
//! This module contains the configuration for the backend.
//! The config file should be written as a [ron](https://docs.rs/ron/latest/ron/index.html) file.

use {
    anyhow::{Context, Result},
    derive_more::Display,
    hex::FromHex,
    sentry::types::Dsn,
    serde::{de::DeserializeOwned, Deserialize, Deserializer},
    std::{
        error::Error,
        num::{NonZeroU32, NonZeroU64},
        path::PathBuf,
        str::FromStr,
        sync::{Mutex, OnceLock},
    },
    zeroize::{Zeroize, ZeroizeOnDrop},
};

pub const GRACEFUL_SHUTDOWN_TIMEOUT_SECONDS: u64 = 3;

const HMAC_KEY_LENGTH_BYTES: usize = 64;

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct HmacKey(pub [u8; HMAC_KEY_LENGTH_BYTES]);

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct PostgresConnectionUrl(pub String);

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize, Display, PartialEq, Eq)]
pub enum Environment {
    Local,
    Staging,
    Production,
}

/// The TLS configuration.
#[derive(Deserialize, Debug)]
pub struct TlsConfig {
    /// The filepath to the root certificate authorities.
    pub root_certificates_pem_path: PathBuf,

    /// The filepath to the server tls certificate.
    pub cert_path: PathBuf,

    /// The filepath to the server tls private key.
    pub cert_key_path: PathBuf,
}

/// The root configuration.
#[derive(Deserialize)]
pub struct Config {
    /// The environment in which the backend is running.
    pub environment: Environment,

    /// The logging directives to use.
    /// Acceptable directive must must follow
    /// [this][1] format.
    ///
    /// [1]: https://docs.rs/env_logger/latest/env_logger/index.html#enabling-logging
    pub logging_directives: String,

    /// The sentry DSN
    pub sentry_data_source_name: Option<Dsn>,

    /// The interval in seconds to log machine stats.
    pub machine_stats_logging_interval_seconds: NonZeroU64,

    /// The cpu temperature in celsius above which a warning is logged.
    pub cpu_temp_warn_threshold_celsius: f32,

    /// The memory usage percentage above which a warning is logged.
    pub memory_usage_warn_threshold_percentage: f64,

    /// The cpu load average above which a warning is logged.
    pub cpu_load_warn_limit: f32,

    /// The burst size for rate limiting.
    ///
    /// Sets the quota size that defines how many requests can occur before the
    /// governor middleware starts blocking requests from an IP address and
    /// clients have to wait until the elements of the quota are replenished.
    pub rate_limiter_burst_size: NonZeroU32,

    /// The replenish rate for rate limiting.
    ///
    /// Sets the interval after which one element of the quota is replenished in
    /// milliseconds.
    pub rate_limiter_replenish_rate_milliseconds: NonZeroU64,

    /// The path to the static assets to serve.
    /// This is where the favicons directory should be located.
    pub static_assets_path: PathBuf,

    /// The secret key to use for cryptographic hashing.
    ///
    /// Must be 64 bytes long and encoded as hex.
    /// Use `openssl rand -hex 64` to generate a new key.
    #[serde(deserialize_with = "hmac_key_from_str")]
    pub hmac_key: ReadOnce<HmacKey>,

    /// The maximum number of active postgres connections to pool.
    pub max_postgres_connection_pool_size: u32,

    /// The postgres connection url.
    #[serde(deserialize_with = "postgres_connection_url_from_str")]
    pub postgres_connection_url: ReadOnce<PostgresConnectionUrl>,

    pub tls_config: Option<TlsConfig>,
}

/// Initialize the config.
#[deny(dead_code)]
pub fn init() -> Result<()> {
    let config = parse::<Config>(env!("CARGO_PKG_NAME"))?;

    if cfg!(debug_assertions) && config.environment != Environment::Local {
        panic!("debug builds are not allowed to run in non-local environments");
    }

    CONFIG
        .set(config)
        // can't use expect because Config deliberately doesn't implement Debug.
        .unwrap_or_else(|_| panic!("config already initialized"));
    Ok(())
}

/// Returns the global configuration for the server.
#[inline]
pub fn get() -> &'static Config {
    CONFIG.get().expect("config not initialized")
}

pub struct ReadOnce<T: Zeroize + ZeroizeOnDrop> {
    value: Mutex<Option<T>>,
}

impl<T: Zeroize + ZeroizeOnDrop> ReadOnce<T> {
    /// Creates a new ReadOnce with the given value.
    pub fn new(value: T) -> Self {
        Self {
            value: Mutex::new(Some(value)),
        }
    }

    /// Returns the value if it has not already been read.
    /// ### Panicking Behavior
    /// Panics called twice.
    pub fn get_once(&self) -> T {
        self.value
            .lock()
            .expect("error acquiring lock")
            .take()
            .expect("value already read")
    }
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
) -> Result<ReadOnce<PostgresConnectionUrl>, D::Error>
where
    D: Deserializer<'de>,
{
    let url: String = Deserialize::deserialize(deserializer)?;
    Ok(ReadOnce::new(PostgresConnectionUrl(url)))
}

fn hmac_key_from_str<'de, D>(deserializer: D) -> Result<ReadOnce<HmacKey>, D::Error>
where
    D: Deserializer<'de>,
{
    let hex: String = Deserialize::deserialize(deserializer)?;
    let bytes = <[u8; HMAC_KEY_LENGTH_BYTES]>::from_hex(hex).map_err(serde::de::Error::custom)?;

    Ok(ReadOnce::new(HmacKey(bytes)))
}
