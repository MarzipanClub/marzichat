//! Config related utilities.

use {
    anyhow::{Context, Result},
    serde::de::DeserializeOwned,
    std::{path::PathBuf, str::FromStr},
};

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
