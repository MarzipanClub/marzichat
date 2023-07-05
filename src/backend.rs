#![cfg(feature = "ssr")]

use {
    actix_files::Files,
    actix_web::*,
    leptos::*,
    leptos_actix::LeptosRoutes,
    marzichat::App,
    tracing_subscriber::{fmt::layer, layer::SubscriberExt, util::SubscriberInitExt},
};

#[get("/style.css")]
async fn css() -> impl Responder {
    actix_files::NamedFile::open_async("./style.css").await
}

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    actix_files::NamedFile::open_async("./target/site/favicon.ico").await
}

/// Run the backend server.
pub async fn run() -> anyhow::Result<()> {
    let log = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::builder()
                .with_env_var("LOG")
                .from_env_lossy(),
        )
        .with(sentry_tracing::layer());

    if cfg!(debug_assertions) {
        log.with(layer().without_time().with_line_number(true))
            .init();
    } else {
        log.with(layer().with_line_number(true))
            .with(tracing_journald::Layer::new().expect("failed to initialize journald layer"))
            .init();
    }

    {
        let release = sentry::release_name!().expect("error getting release name");

        let guard = sentry::init(sentry::ClientOptions {
            dsn: std::env::var("SENTRY_DSN")
                .ok()
                .map(|dsn| dsn.parse().ok())
                .flatten(),
            release: Some(release.to_owned()),
            environment: Some(
                gethostname::gethostname()
                    .to_string_lossy()
                    .to_string()
                    .into(),
            ),
            ..Default::default()
        });

        tracing::info!(is_enabled = guard.is_enabled(), ?release, "sentry");

        // keep the guard for the lifetime of the program
        std::mem::forget(guard);
    }

    let leptos_options = {
        let mut opt = leptos_config::get_config_from_env()
            .expect("failed to get leptos config")
            .leptos_options;
        opt.env = if cfg!(debug_assertions) {
            leptos_config::Env::DEV
        } else {
            leptos_config::Env::PROD
        };
        opt
    };

    let site_addr = leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = leptos_actix::generate_route_list(|cx| view! { cx, <App/> });
    let server = HttpServer::new(move || {
        let site_root = &leptos_options.site_root;
        App::new()
            .service(css)
            .service(favicon)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", site_root))
    })
    .bind(&site_addr)?;

    tracing::info!(socket_addresses = ?server.addrs(), "binding");
    tracing::info!("✅ ready");

    server.run().await?;
    Ok(())
}
