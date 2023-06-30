//! The main entry point for the backend.

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

use anyhow::Result;

mod build;
mod config;
mod handlers;
mod logging;
mod postgres;
mod server;

#[tokio::main]
async fn main() -> Result<()> {
    color_backtrace::install();
    config::init()?;
    logging::init();
    postgres::init().await?;
    server::run().await?;
    Ok(())
}
