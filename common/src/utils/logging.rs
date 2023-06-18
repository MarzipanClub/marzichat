//! Utilities for logging.

use {
    crate::utils::stats::{
        CPU_LOAD_WARN_LIMIT, CPU_TEMP_WARN_THRESHOLD_CELSIUS, INTERVAL_PERIOD,
        MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE,
    },
    std::time::Duration,
    tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

/// Initialize logging using the logging directive specified in the config file.
pub fn init(logging_directive: &str) -> anyhow::Result<()> {
    let logging = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::builder().parse(logging_directive)?)
        .with(sentry_tracing::layer());

    if cfg!(debug_assertions) {
        logging.with(layer().without_time()).init();
    } else {
        logging
            .with(layer())
            .with(tracing_journald::Layer::new()?)
            .init();
    }

    #[cfg(target_os = "linux")]
    crate::utils::stats::warn_machine_stats(
        INTERVAL_PERIOD,
        CPU_TEMP_WARN_THRESHOLD_CELSIUS,
        MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE,
        CPU_LOAD_WARN_LIMIT,
    );
    Ok(())
}
