//! Http web server.
#![cfg(feature = "ssr")]

use {actix_files::Files, actix_web::*, leptos::*, leptos_actix::LeptosRoutes, marzichat::App};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::NoContent()
}

#[get("/info")]
async fn info() -> impl Responder {
    HttpResponse::Ok()
        .content_type(http::header::ContentType::plaintext())
        .body(marzichat::summary())
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

    // Generate the list of routes in your Leptos App
    let routes = leptos_actix::generate_route_list(|cx| view! { cx, <App/> });
    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(info)
            .service(favicon)
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .service(Files::new("/", &leptos_options.site_root))
            .wrap(crate::limiter::governor())
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
