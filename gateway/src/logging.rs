//! # Logging module
//!
//! This module sets up logging for the server.

use {
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
    const CPU_TEMP_WARN_THRESHOLD_CELSIUS: f32 = 50.0;
    const MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE: f64 = 0.4;

    tokio::task::spawn(async {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            let system_stats = System::new();
            match system_stats.cpu_temp() {
                Ok(cpu_temp) => {
                    if cpu_temp > CPU_TEMP_WARN_THRESHOLD_CELSIUS {
                        tracing::warn!(
                            "cpu temp is above warning threshold of {}°C: {}°C",
                            CPU_TEMP_WARN_THRESHOLD_CELSIUS,
                            cpu_temp
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("error getting cpu temp: {}", e);
                    break;
                }
            }
            match system_stats.memory() {
                Ok(memory) => {
                    let memory_usage = saturating_sub_bytes(memory.total, memory.free);
                    let memory_usage_percentage =
                        (memory_usage.as_u64() as f64) / (memory.total.as_u64() as f64);
                    if memory_usage_percentage > MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE {
                        tracing::warn!(
                            "memory usage is above warning threshold of {}%: {}%",
                            MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE * 100.0,
                            memory_usage_percentage * 100.0
                        );
                    }
                }
                Err(e) => {
                    tracing::error!("error getting memory usage: {}", e);
                    break;
                }
            }
        }
    });
}
