//! The main entry point for the backend.

// rustc lints
// https://doc.rust-lang.org/rustc/lints/index.html
#![forbid(unsafe_code, let_underscore_lock)]
#![deny(unused_extern_crates)]
#![warn(
    future_incompatible,
    let_underscore_drop,
    rust_2018_idioms,
    single_use_lifetimes,
    unused_import_braces,
    unused_lifetimes,
    unused_macro_rules,
    unused_qualifications,
    unused_tuple_struct_fields,
    variant_size_differences
)]

use {
    anyhow::Result,
    app::{App, ASSETS_PATH},
    axum::{
        routing::{get, post},
        Router,
    },
    axum_sessions::{async_session::CookieStore, SessionLayer},
    const_format::formatcp,
    hyper::{
        server::{accept::Accept, conn::AddrIncoming},
        Server,
    },
    leptos::{leptos_config::Env, view, LeptosOptions},
    leptos_axum::{generate_route_list, LeptosRoutes},
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        pin::Pin,
        task::{Context, Poll},
    },
    tokio::signal,
    tower::ServiceBuilder,
    tower_governor::{governor::GovernorConfigBuilder, GovernorLayer},
    tower_http::{
        compression::CompressionLayer,
        services::{fs::ServeDir, ServeFile},
    },
};

mod api;
mod config;
mod handler;
mod logging;
mod postgres;

#[tokio::main]
async fn main() -> Result<()> {
    color_backtrace::install();

    config::init()?;
    logging::init()?;
    postgres::init().await?;

    let assets_path = &config::get().static_assets_path;

    const API: &str = "/api";

    let router = Router::new()
        .route("/foo", get(|| async { "foobar" }))
        .route(formatcp!("{API}/account/:id"), post(handler::account::get))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(
            LeptosOptions {
                output_name: "app".into(),
                site_root: ".".into(),
                site_pkg_dir: ASSETS_PATH.into(),
                // we're not using cargo-leptos so PROD here has no effect
                // PROD removes the websocket for reloading code in leptos
                env: Env::PROD,
                // not used; filler value
                site_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0),
                // not used; filler value
                reload_port: 0,
            },
            generate_route_list(|cx| view! { cx, <App/> }).await,
            |cx| view! { cx, <App/> },
        )
        .nest_service(formatcp!("/{ASSETS_PATH}"), ServeDir::new(assets_path))
        .nest_service(
            formatcp!("/favicon.ico"),
            ServeFile::new(assets_path.join("favicons").join("favicon.ico")),
        )
        .nest_service(
            formatcp!("/{ASSETS_PATH}/favicons"),
            ServeDir::new(assets_path.join("favicons")),
        )
        .layer(
            ServiceBuilder::new()
                .layer(
                    // logging layer
                    tower_http::trace::TraceLayer::new_for_http(),
                )
                .layer(
                    // layer to handler errors in subsequent middleware layers
                    axum::error_handling::HandleErrorLayer::new(|e: axum::BoxError| async move {
                        tower_governor::errors::display_error(e)
                    }),
                )
                .layer(
                    // rate limiting layer
                    GovernorLayer {
                        config: Box::leak(Box::new(
                            // allow bursts with up to rate_limit_interval_per_second requests per
                            // ip address and replenishes one element
                            // every rate_limit_burst_size seconds
                            GovernorConfigBuilder::default()
                                .per_second(config::get().rate_limit_interval_per_second.into())
                                .burst_size(config::get().rate_limit_burst_size.into())
                                .finish()
                                .ok_or(anyhow::anyhow!("invalid rate limiting configuration"))?,
                        )),
                    },
                )
                .layer(if cfg!(debug_assertions) {
                    // disable brotli compression in debug mode to use source view in Firefox
                    // (Cmd + U)
                    CompressionLayer::new().no_br()
                } else {
                    CompressionLayer::new()
                }),
        )
        .layer({
            SessionLayer::new(CookieStore::new(), &config::get().cookie_signing_key.0)
                .with_secure(
                    // set secure cookie attributes on release builds.
                    // secure cookies are only sent via https
                    cfg!(not(debug_assertions)),
                )
                .with_http_only(
                    // http_only makes a cookie inaccessible to Javascript and thus
                    // unusable for XSS attacks
                    true,
                )
                .with_cookie_name("session-id")
                .with_session_ttl(Some(std::time::Duration::from_secs({
                    const COOKIE_SESSION_DURATION_DAYS: u64 = 60;
                    60 * 60 * 24 * COOKIE_SESSION_DURATION_DAYS
                })))
        });

    let listeners = Listeners([
        AddrIncoming::bind(&SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 2506))?,
        AddrIncoming::bind(&SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 2506))?,
    ]);

    tracing::info!(?listeners, "ready");

    Server::builder(listeners)
        .serve(router.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            let ctrl_c = async {
                signal::ctrl_c()
                    .await
                    .expect("failed to install Ctrl+C handler");
            };

            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("failed to install signal handler")
                    .recv()
                    .await;
            };

            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }

            tracing::info!("shutting down");
        })
        .await?;

    Ok(())
}

struct Listeners<const N: usize>([AddrIncoming; N]);

impl<const N: usize> std::fmt::Debug for Listeners<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut list = f.debug_list();
        for addr_incoming in &self.0 {
            list.entry(&addr_incoming.local_addr());
        }
        list.finish()
    }
}

impl<const N: usize> Accept for Listeners<N> {
    type Conn = <AddrIncoming as Accept>::Conn;
    type Error = <AddrIncoming as Accept>::Error;

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        for listener in &mut self.0 {
            if let Poll::Ready(Some(conn)) = Pin::new(listener).poll_accept(cx) {
                return Poll::Ready(Some(conn));
            }
        }

        Poll::Pending
    }
}
