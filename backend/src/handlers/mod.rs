//! Handlers for the various routes.

use {
    actix_web::{get, web::ServiceConfig, Responder},
    common::routes::ASSETS_PATH,
    const_format::formatcp,
};

#[get("/favicon.ico")]
async fn favicon() -> impl Responder {
    actix_files::NamedFile::open_async(formatcp!("{ASSETS_PATH}/favicons/favicon.ico")).await
}

/// Configure the endpoints.
/// Note that actix may call this function multiple times.
pub fn routes(config: &mut ServiceConfig) {
    let cfg = crate::config::get();

    config.service(actix_files::Files::new(
        ASSETS_PATH,
        &cfg.static_assets_path,
    ));

    // the favicon.ico file is served from the root
    // should be registered last otherwise other services won't be registered
    config.service(favicon);
}
