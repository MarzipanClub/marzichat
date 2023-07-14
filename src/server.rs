//! Http web server.
#![cfg(feature = "ssr")]

use {
    crate::config::{RateLimiterConfig, ServerConfig, TlsConfig},
    actix_files::Files,
    actix_web::*,
    anyhow::Result,
    leptos::*,
    leptos_actix::LeptosRoutes,
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        path::Path,
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
        tracing::warn!("No tls config found. Running in http mode instead.");
        server.bind(&site_addr).expect("couldn't bind port")
    };

    tracing::info!(socket_addresses = ?server.addrs(), "binding");
    tracing::info!("✅ ready");

    server
        .workers(config.os_threads_per_bind_address.get())
        .run()
        .await?;

    Ok(())
}
