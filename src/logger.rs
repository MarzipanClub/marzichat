//! Logging module.
#![cfg(feature = "ssr")]

use {
    crate::config::LoggingConfig,
    anyhow::Context,
    std::time::Duration,
    systemstat::{saturating_sub_bytes, Platform, System},
    tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

/// Sets up logging.
#[deny(dead_code)]
pub fn init(config: LoggingConfig) {
    let log = tracing_subscriber::registry()
        .with(config.directives)
        .with(sentry_tracing::layer());

    // show line numbers and hide timestamps in debug builds
    #[cfg(debug_assertions)]
    let log = log.with(layer().without_time().with_line_number(true));

    // journald is a linux-only feature
    #[cfg(target_os = "linux")]
    let log =
        log.with(tracing_journald::Layer::new().expect("failed to initialize journald layer"));

    log.init();

    let release = sentry::release_name!().expect("error getting release name");

    let guard = sentry::init(sentry::ClientOptions {
        dsn: config.sentry_data_source_name,
        release: Some(release.to_owned()),
        ..Default::default()
    });

    tracing::info!(%release);
    if !guard.is_enabled() {
        tracing::warn!("no SENTRY_DSN found, sentry is disabled");
    }

    // keep the guard for the lifetime of the program
    std::mem::forget(guard);

    let period = Duration::from_secs(config.machine_stats_interval_seconds.into());
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(period);
        loop {
            interval.tick().await;
            let cpu_temp = System::new().cpu_temp().context("error getting cpu temp")?;
            if cpu_temp > config.cpu_temp_warn_threshold_celsius {
                tracing::warn!(
                    "cpu temp is above warning threshold of {}°C: {}°C",
                    config.cpu_temp_warn_threshold_celsius,
                    cpu_temp
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(period);
        loop {
            interval.tick().await;
            let memory = System::new()
                .memory()
                .context("error getting memory usage")?;
            let memory_usage = saturating_sub_bytes(memory.total, memory.free);
            let memory_usage_percentage =
                (memory_usage.as_u64() as f64) / (memory.total.as_u64() as f64);
            if memory_usage_percentage > config.system_memory_usage_warn_threshold_percentage {
                tracing::warn!(
                    "memory usage is above warning threshold of {}%: {:.2}%",
                    config.system_memory_usage_warn_threshold_percentage * 100.0,
                    memory_usage_percentage * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });

    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(period);
        loop {
            interval.tick().await;
            let cpu_load_aggregate = System::new()
                .cpu_load_aggregate()
                .context("error getting cpu load")?;

            tokio::time::sleep(Duration::from_secs(1)).await;
            let cpu_load_aggregate = cpu_load_aggregate.done()?;
            if cpu_load_aggregate.system > config.cpu_load_warn_limit {
                tracing::warn!(
                    "cpu load is above warning threshold of {}%: {}%",
                    config.cpu_load_warn_limit * 100.0,
                    cpu_load_aggregate.system * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
