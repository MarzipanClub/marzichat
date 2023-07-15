//! Http web server.
#![cfg(feature = "ssr")]

use {
    crate::config::ServerConfig,
    actix_files::Files,
    actix_web::*,
    anyhow::Result,
    leptos::*,
    leptos_actix::LeptosRoutes,
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        path::Path,
        time::Duration,
    },
};

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
async fn favicon(leptos_options: web::Data<LeptosOptions>) -> impl Responder {
    let leptos_options = leptos_options.into_inner();
    actix_files::NamedFile::open(Path::new(&leptos_options.site_root).join("favicon.ico"))
}

/// Run the backend server with the given configs.
pub async fn run(config: ServerConfig) -> Result<()> {
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
    let routes = leptos_actix::generate_route_list(marzichat::App);
    let output_dir = marzichat::OUT_DIR;
    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(info)
            .service(favicon)
            .service(Files::new(
                output_dir,
                format!("{}/{output_dir}", &leptos_options.site_root),
            ))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(leptos_options.to_owned(), routes.to_owned(), marzichat::App)
            .wrap(crate::limiter::layer(config.rate_limiter.clone()))
            .wrap(middleware::Logger::new("%s for %U %a in %Ts"))
            .wrap(sentry_actix::Sentry::new())
            .wrap(middleware::Compress::default())
            .wrap(crate::redirect::HttpToHttps)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(leptos_options.to_owned()))
    });

    let server = if let Some(tls) = config.tls {
        let http = [
            SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 80),
            SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 80),
        ];
        let https = [
            SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 443),
            SocketAddr::new(Ipv6Addr::UNSPECIFIED.into(), 443),
        ];
        server
            .bind(http.as_ref())
            .expect("couldn't bind to port 80")
            .bind_rustls(https.as_ref(), tls)
            .expect("couldn't bind to port 443")
    } else {
        tracing::warn!("Tls not configured");
        server.bind(&site_addr).expect("couldn't bind port")
    };

    let workers = config.os_threads_per_bind_address.get();
    tracing::info!(socket_addrs = ?server.addrs(), threads_per_addr = workers, "binding");

    server
        .workers(workers)
        .client_disconnect_timeout(Duration::from_millis(
            config.client_disconnect_timeout_milliseconds.get(),
        ))
        .client_request_timeout(Duration::from_millis(
            config.client_request_timeout_milliseconds.get(),
        ))
        .shutdown_timeout(config.shutdown_timeout_seconds.get())
        .run()
        .await?;

    Ok(())
}
