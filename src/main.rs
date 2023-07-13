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
fn main() -> anyhow::Result<()> {
    println!("{}", marzichat::summary());
    let args = std::env::args_os().collect::<Vec<std::ffi::OsString>>();

    match (args.get(1), args.get(2)) {
        (Some(validate), Some(config)) if validate == "validate" => {
            println!("validating config");
            match config::parse(&std::path::PathBuf::from(config)) {
                Ok(_) => {
                    println!("config is valid");
                    Ok(())
                }
                Err(error) => Err(anyhow::anyhow!("invalid config: {error}")),
            }
        }
        (Some(config), _) => {
            let config = config::parse(&std::path::PathBuf::from(config))?;
            logger::init(config.logging);
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(config.io_threads.get())
                .build()
                .expect("failed to build tokio runtime")
                .block_on(async {
                    postgres::init().await;
                    server::run().await
                })
        }
        _ => Err(anyhow::anyhow!("invalid arguments")),
    }
}

// no main function if we're not using ssr feature
#[cfg(not(feature = "ssr"))]
fn main() {}
