//! # Logging module
//!
//! This module sets up logging for the server.

use {
    anyhow::Context,
    std::time::Duration,
    systemstat::{saturating_sub_bytes, Platform, System},
    tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

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

    log_machine_stats();
    Ok(())
}

fn log_machine_stats() {
    const INTERVAL_PERIOD: Duration = Duration::from_millis(500);
    const CPU_TEMP_WARN_THRESHOLD_CELSIUS: f32 = 50.0;
    const MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE: f64 = 0.4;
    const CPU_LOAD_WARN_LIMIT: f32 = 0.8;

    tokio::task::spawn(async {
        let mut interval = tokio::time::interval(INTERVAL_PERIOD);
        loop {
            interval.tick().await;
            let cpu_temp = System::new().cpu_temp().context("error getting cpu temp")?;
            if cpu_temp > CPU_TEMP_WARN_THRESHOLD_CELSIUS {
                tracing::warn!(
                    "cpu temp is above warning threshold of {}°C: {}°C",
                    CPU_TEMP_WARN_THRESHOLD_CELSIUS,
                    cpu_temp
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    tokio::task::spawn(async {
        let mut interval = tokio::time::interval(INTERVAL_PERIOD);
        loop {
            interval.tick().await;
            let memory = System::new()
                .memory()
                .context("error getting memory usage")?;
            let memory_usage = saturating_sub_bytes(memory.total, memory.free);
            let memory_usage_percentage =
                (memory_usage.as_u64() as f64) / (memory.total.as_u64() as f64);
            if memory_usage_percentage > MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE {
                tracing::warn!(
                    "memory usage is above warning threshold of {}%: {:.2}%",
                    MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE * 100.0,
                    memory_usage_percentage * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    tokio::task::spawn(async {
        let mut interval = tokio::time::interval(INTERVAL_PERIOD);
        loop {
            interval.tick().await;
            let cpu_load_aggregate = System::new()
                .cpu_load_aggregate()
                .context("error getting cpu load")?;

            tokio::time::sleep(Duration::from_secs(1)).await;
            let cpu_load_aggregate = cpu_load_aggregate.done()?;
            if cpu_load_aggregate.system > CPU_LOAD_WARN_LIMIT {
                tracing::warn!(
                    "cpu load is above warning threshold of {}%: {}%",
                    CPU_LOAD_WARN_LIMIT * 100.0,
                    cpu_load_aggregate.system * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
