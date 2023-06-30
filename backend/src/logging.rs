//! Logging module.

use {
    anyhow::Context,
    common::PRODUCT_NAME,
    std::time::Duration,
    systemstat::{saturating_sub_bytes, Platform, System},
    tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter},
};

/// Initialize logging using the logging directive specified in the config file.
#[deny(dead_code)]
pub fn init() {
    let cfg = crate::config::get();

    let logging = tracing_subscriber::registry()
        .with(
            EnvFilter::builder()
                .parse(&cfg.logging_directives)
                .expect("invalid logging directives"),
        )
        .with(sentry_tracing::layer());

    if cfg!(debug_assertions) {
        logging
            .with(layer().without_time().with_line_number(true))
            .init();
    } else {
        logging
            .with(layer().with_line_number(true))
            .with(tracing_journald::Layer::new().expect("failed to initialize journald layer"))
            .init();
    }
    tracing::info!(logging_directives = cfg.logging_directives);

    let release = format!("{}@{}", PRODUCT_NAME.to_lowercase(), crate::build::VERSION);

    let guard = sentry::init(sentry::ClientOptions {
        dsn: crate::config::get().sentry_data_source_name.to_owned(),
        release: cfg!(not(debug_assertions)).then(|| release.clone().into()),
        environment: Some(cfg.environment.to_string().into()),
        ..Default::default()
    });

    tracing::info!(is_enabled = guard.is_enabled(), release, "sentry");

    // keep the guard for the lifetime of the program
    std::mem::forget(guard);

    warn_machine_stats();
}

/// Logs cpu temperature, memory usage, and cpu load average.
pub fn warn_machine_stats() {
    let cfg = crate::config::get();

    let period = Duration::from_secs(cfg.machine_stats_logging_interval_seconds.into());
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(period);
        loop {
            interval.tick().await;
            let cpu_temp = System::new().cpu_temp().context("error getting cpu temp")?;
            if cpu_temp > cfg.cpu_temp_warn_threshold_celsius {
                tracing::warn!(
                    "cpu temp is above warning threshold of {}°C: {}°C",
                    cfg.cpu_temp_warn_threshold_celsius,
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
            if memory_usage_percentage > cfg.memory_usage_warn_threshold_percentage {
                tracing::warn!(
                    "memory usage is above warning threshold of {}%: {:.2}%",
                    cfg.memory_usage_warn_threshold_percentage * 100.0,
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
            if cpu_load_aggregate.system > cfg.cpu_load_warn_limit {
                tracing::warn!(
                    "cpu load is above warning threshold of {}%: {}%",
                    cfg.cpu_load_warn_limit * 100.0,
                    cpu_load_aggregate.system * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
