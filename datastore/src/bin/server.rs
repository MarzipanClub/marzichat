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
    crate::handler::{Command, CommandContext, Context},
    anyhow::Result,
    datastore::Datastore,
    futures::{future, SinkExt, StreamExt},
    std::any,
    tarpc::{
        context,
        server::{incoming::Incoming, Channel},
        tokio_serde::formats::Bincode,
    },
    tokio::sync::mpsc::UnboundedSender,
};

mod config;
mod handler;

#[tokio::main]
async fn main() -> Result<()> {
    color_backtrace::install();
    config::init()?;
    common::utils::logging::init(&crate::config::get().logging_directive)?;

    let listener =
        tarpc::serde_transport::tcp::listen(&crate::config::get().socket_address, Bincode::default)
            .await?;
    tracing::info!("listening on {}", listener.local_addr());

    let (main_sender, mut main_receiver): (UnboundedSender<CommandContext>, _) =
        tokio::sync::mpsc::unbounded_channel();

    let (shutdown_sender, mut shutdown_receiver) = tokio::sync::oneshot::channel();
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to set signal handler");
        shutdown_sender
            .send(())
            .expect("failed to send shutdown signal");
    });

    let database = redb::Database::create(&crate::config::get().database_file_path)?;

    tokio::spawn(async move {
        // while there is an error receiving the shutdown signal keep processing
        // commands
        while shutdown_receiver.try_recv().is_err() {
            match main_receiver.recv().await {
                Some(command_context) => command_context.handle(&database),
                None => break,
            };
        }

        Ok::<_, anyhow::Error>(())
    });

    listener
        // ignore accept errors
        .filter_map(|r| future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        // limit channels to 1 per ip address
        // .peer_addr() shouldn't normally fail so safe to unwrap
        .max_channels_per_key(1, |channel| channel.transport().peer_addr().unwrap().ip())
        .map(|channel| {
            channel.execute(Context::new(main_sender.clone()).serve())
        })
        // max 100 channels
        .buffer_unordered(100)
        // run stream to completion
        .for_each(|_| async {})
        .await;

    Ok(())
}
