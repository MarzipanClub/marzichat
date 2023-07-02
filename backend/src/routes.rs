//! Handlers for the various routes.

use {
    crate::websocket,
    actix_web::{http::header::ContentType, web, HttpResponse},
    common::routes::{ASSETS_PATH, HEALTH, INFO, WEBSOCKET},
    const_format::formatcp,
};

fn routes(config: &mut web::ServiceConfig) {
    let cfg = crate::config::get();

    config.service(
        actix_files::Files::new(formatcp!("/{ASSETS_PATH}"), &cfg.static_assets_path)
            .prefer_utf8(true),
    );

    config.service(web::resource(HEALTH).route(web::get().to(HttpResponse::NoContent)));

    config.service(web::resource(INFO).route(web::get().to(|| async {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(crate::build::summary())
    })));

    config.route(WEBSOCKET, actix_web::web::get().to(websocket::handler));

    // the favicon.ico file is served from the root
    // should be registered last otherwise other services won't be registered
    config.service(web::resource("/favicon.ico").route(web::get().to(|| async {
        actix_files::NamedFile::open_async(formatcp!("{ASSETS_PATH}/favicons/favicon.ico")).await
    })));
}

/// Adds all the backend's services to the config.
/// Note that actix may call this function multiple times.
pub fn config(config: &mut web::ServiceConfig) {
    // all services will use strict transport security
    // the empty string is the root scope
    #[allow(clippy::let_and_return)]
    let scope = actix_web::web::scope("").configure(routes);

    // enable strict transport security in release builds
    #[cfg(not(debug_assertions))]
    let scope = scope.wrap(actix_web_lab::middleware::RedirectHttps::with_hsts(
        actix_web_lab::header::StrictTransportSecurity::recommended(),
    ));

    config.service(scope);
}
