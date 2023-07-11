//! Logging module.
#![cfg(feature = "ssr")]

use tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt};

/// Sets up logging.
#[deny(dead_code)]
pub fn init() {
    let log = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .with(sentry_tracing::layer());

    // show line numbers and hide timestamps in debug builds
    #[cfg(debug_assertions)]
    let log = log.with(layer().without_time().with_line_number(true));

    // journal is a linux-only feature
    #[cfg(target_os = "linux")]
    let log =
        log.with(tracing_journald::Layer::new().expect("failed to initialize journald layer"));

    log.init();

    let release = sentry::release_name!().expect("error getting release name");

    let guard = sentry::init(sentry::ClientOptions {
        dsn: std::env::var("SENTRY_DSN")
            .ok()
            .map(|dsn| dsn.parse().ok())
            .flatten(),
        release: Some(release.to_owned()),
        ..Default::default()
    });

    tracing::info!(%release);
    if !guard.is_enabled() {
        tracing::warn!("no SENTRY_DSN found, sentry is disabled");
    }

    // keep the guard for the lifetime of the program
    std::mem::forget(guard);
}
