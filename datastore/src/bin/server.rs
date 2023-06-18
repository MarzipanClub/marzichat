//! A key-value store

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

use {
    anyhow::Result,
    datastore::Datastore,
    futures::{future, StreamExt},
    tarpc::{
        context,
        server::{incoming::Incoming, Channel},
        tokio_serde::formats::Bincode,
    },
};

mod config;

// This is the type that implements the generated World trait. It is the
// business logic and is used to start the server.
#[derive(Clone)]
struct Handler;

#[tarpc::server]
impl Datastore for Handler {
    async fn get(self, _: context::Context, key: String) -> Option<String> {
        tracing::info!("Received request for key: {}", key);
        Some("value".into())
    }

    async fn set(self, _: context::Context, key: String, value: String) -> Option<String> {
        tracing::info!("Received request to set key: {} to value: {}", key, value);
        None
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_backtrace::install();
    config::init()?;
    common::utils::logging::init(&crate::config::get().logging_directive)?;

    let listener =
        tarpc::serde_transport::tcp::listen(&crate::config::get().socket_address, Bincode::default)
            .await?;

    tracing::info!("listening on {}", listener.local_addr());

    listener
        // ignore accept errors
        .filter_map(|r| future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        // limit channels to 1 per ip address
        // .peer_addr() shouldn't normally fail so safe to unwrap
        .max_channels_per_key(1, |channel| channel.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            channel.execute(Handler.serve())
        })
        // max 100 channels
        .buffer_unordered(100)
        // run stream to completion
        .for_each(|_| async {})
        .await;

    Ok(())
}
