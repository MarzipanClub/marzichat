//! Utilities for logging machine stats.

use {
    anyhow::Context,
    std::time::Duration,
    systemstat::{saturating_sub_bytes, Platform, System},
};

pub const INTERVAL_PERIOD: Duration = Duration::from_millis(500);
pub const CPU_TEMP_WARN_THRESHOLD_CELSIUS: f32 = 50.0;
pub const MEMORY_USAGE_WARN_THRESHOLD_PERCENTAGE: f64 = 0.4;
pub const CPU_LOAD_WARN_LIMIT: f32 = 0.8;

/// Logs cpu temperature, memory usage, and cpu load average on linux.
#[cfg(target_os = "linux")]
pub fn warn_machine_stats(
    interval: Duration,
    cpu_temp_warn_threshold_celsius: f32,
    memory_usage_warn_threshold: f64,
    cpu_load_warn_limit: f32,
) {
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(interval);
        loop {
            interval.tick().await;
            let cpu_temp = System::new().cpu_temp().context("error getting cpu temp")?;
            if cpu_temp > cpu_temp_warn_threshold_celsius {
                tracing::warn!(
                    "cpu temp is above warning threshold of {}°C: {}°C",
                    cpu_temp_warn_threshold_celsius,
                    cpu_temp
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(INTERVAL_PERIOD);
        loop {
            interval.tick().await;
            let memory = System::new()
                .memory()
                .context("error getting memory usage")?;
            let memory_usage = saturating_sub_bytes(memory.total, memory.free);
            let memory_usage_percentage =
                (memory_usage.as_u64() as f64) / (memory.total.as_u64() as f64);
            if memory_usage_percentage > memory_usage_warn_threshold {
                tracing::warn!(
                    "memory usage is above warning threshold of {}%: {:.2}%",
                    memory_usage_warn_threshold * 100.0,
                    memory_usage_percentage * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
    tokio::task::spawn(async move {
        let mut interval = tokio::time::interval(INTERVAL_PERIOD);
        loop {
            interval.tick().await;
            let cpu_load_aggregate = System::new()
                .cpu_load_aggregate()
                .context("error getting cpu load")?;

            tokio::time::sleep(Duration::from_secs(1)).await;
            let cpu_load_aggregate = cpu_load_aggregate.done()?;
            if cpu_load_aggregate.system > cpu_load_warn_limit {
                tracing::warn!(
                    "cpu load is above warning threshold of {}%: {}%",
                    cpu_load_warn_limit * 100.0,
                    cpu_load_aggregate.system * 100.0
                );
            }
        }
        // allow lint to annotate return type which cannot be inferred
        #[allow(unreachable_code)]
        Ok::<_, anyhow::Error>(())
    });
}
