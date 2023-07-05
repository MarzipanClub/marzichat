//! Http web server.
#![cfg(feature = "ssr")]

use {actix_files::Files, actix_web::*, leptos::*, leptos_actix::LeptosRoutes, marzichat::App};

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

    // The interval after which one element of the quota is replenished in
    // milliseconds.
    let replenish_rate_milliseconds = std::env::var("REPLENISH_RATE_MILLISECONDS")
        .expect("REPLENISH_RATE_MILLISECONDS not set")
        .parse()
        .expect("REPLENISH_RATE_MILLISECONDS is not a number");

    let burst_size = std::env::var("BURST_SIZE")
        .expect("BURST_SIZE not set")
        .parse()
        .expect("BURST_SIZE is not a number");

    tracing::info!(replenish_rate_milliseconds, burst_size);

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
            .wrap(crate::limiter::new_governor(
                replenish_rate_milliseconds,
                burst_size,
            ))
            .wrap(middleware::Logger::new("%s for %U %a in %Ts"))
            .wrap(sentry_actix::Sentry::new())
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
    })
    .bind(&site_addr)?;

    tracing::info!(socket_addresses = ?server.addrs(), "binding");
    tracing::info!("✅ ready");

    server.run().await?;
    Ok(())
}
