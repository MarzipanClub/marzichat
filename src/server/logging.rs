//! # Logging module
//!
//! This module sets up logging for the server.

use tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logging using the logging directive specified in the config file.
#[deny(dead_code)]
pub fn init() -> anyhow::Result<()> {
    let logging = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .parse(&crate::config::get().logging_directive)?,
        )
        .with(sentry_tracing::layer());

    if cfg!(debug_assertions) {
        logging.with(layer().without_time()).init();
    } else {
        logging
            .with(layer())
            .with(tracing_journald::Layer::new()?)
            .init();
    }

    Ok(())
}
