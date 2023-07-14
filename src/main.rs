// rustc lints
// https://doc.rust-lang.org/rustc/lints/index.html
#![forbid(unsafe_code, let_underscore_lock)]
#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    let_underscore_drop,
    rust_2018_idioms,
    single_use_lifetimes,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences
)]

mod address;
mod config;
mod limiter;
mod logger;
mod postgres;
mod redirect;
mod server;

#[cfg(feature = "ssr")]
#[fncmd::fncmd]
#[tokio::main(flavor = "current_thread")]
async fn main(
    /// Config file path if not using CONFIG env var.
    #[opt(short, long)]
    config: Option<std::path::PathBuf>,

    /// Validate the config and exit without running the server.
    #[opt]
    dry_run: bool,

    /// Print info and exit.
    #[opt(short, long)]
    info: bool,
) -> anyhow::Result<()> {
    use anyhow::Context;

    if info {
        println!("{}", marzichat::summary());
        Ok(())
    } else {
        let config = match config {
            Some(config) => config,
            None => std::env::var_os("CONFIG")
                .context("CONFIG env var not set and --config not passed in as argument")?
                .into(),
        };
        let config = config::parse_from_file(&config).context("invalid config")?;
        if dry_run {
            sqlx::postgres::PgPool::connect(&config.postgres.url)
                .await
                .context("postgres unreachable")?;
            Ok(())
        } else {
            logger::init(config.logging);
            postgres::init(config.postgres).await;
            server::run(config.server).await
        }
    }
}

// no main function if we're not using ssr feature
// because wasm is loaded from the lib.rs using the `hydrate()`
// as the entry point
#[cfg(not(feature = "ssr"))]
fn main() {}
