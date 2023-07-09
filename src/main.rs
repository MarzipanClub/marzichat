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
mod limiter;
mod logger;
mod postgres;
mod redirect;
mod server;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    postgres::init().await;
    server::run().await
}

// no main function if we're not using ssr feature
#[cfg(not(feature = "ssr"))]
fn main() {}
