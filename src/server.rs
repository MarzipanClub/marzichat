//! Http web server.
#![cfg(feature = "ssr")]

use {
    actix_files::Files,
    actix_web::*,
    anyhow::Result,
    leptos::*,
    leptos_actix::LeptosRoutes,
    marzichat::{App, OUT_DIR},
    std::{
        env,
        fs::File,
        io::BufReader,
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

/// Run the backend server.
pub async fn run() -> Result<()> {
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
    tracing::debug!("site_root: {}", leptos_options.site_root);

    // Generate the list of routes in your Leptos App
    let routes = leptos_actix::generate_route_list(|cx| view! { cx, <App/> });
    let server = HttpServer::new(move || {
        App::new()
            .service(health)
            .service(info)
            .service(favicon)
            .service(Files::new(
                OUT_DIR,
                format!("{}/{OUT_DIR}", &leptos_options.site_root),
            ))
            .route("/api/{tail:.*}", leptos_actix::handle_server_fns())
            .leptos_routes(
                leptos_options.to_owned(),
                routes.to_owned(),
                |cx| view! { cx, <App/> },
            )
            .wrap(crate::limiter::governor())
            .wrap(middleware::Logger::new("%s for %U %a in %Ts"))
            .wrap(sentry_actix::Sentry::new())
            .wrap(middleware::Compress::default())
            .wrap(crate::redirect::HttpToHttps)
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .app_data(web::Data::new(leptos_options.to_owned()))
    });

    let server =
        if let (Some(cert), Some(key)) = (env::var_os("TLS_CERT"), env::var_os("TLS_CERT_KEY")) {
            let cert = File::open(cert).expect("error opening TLS_CERT");
            let key = File::open(key).expect("error opening TLS_CERT_KEY");
            let cert_chain = rustls_pemfile::certs(&mut BufReader::new(cert))
                .expect("couldn't parse cert")
                .into_iter()
                .map(rustls::Certificate)
                .collect();
            let key = {
                let mut keys = rustls_pemfile::pkcs8_private_keys(&mut BufReader::new(key))
                    .expect("couldn't parse cert key");

                anyhow::ensure!(!keys.is_empty(), "no cert key found");
                // get the private key
                rustls::PrivateKey(keys.swap_remove(0))
            };
            let config = rustls::ServerConfig::builder()
                .with_safe_defaults()
                .with_no_client_auth()
                .with_single_cert(cert_chain, key)
                .expect("couldn't set up certificate chain");

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
                .bind_rustls(https.as_ref(), config)
                .expect("couldn't bind to port 443")
        } else {
            tracing::warn!("TLS_CERT not set, tls disabled");
            server.bind(&site_addr).expect("couldn't bind port")
        };

    tracing::info!(socket_addresses = ?server.addrs(), "binding");
    tracing::info!("✅ ready");

    server.run().await?;
    Ok(())
}
