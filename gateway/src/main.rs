//! The main entry point for the gateway.

// rustc lints
// https://doc.rust-lang.org/rustc/lints/index.html
#![forbid(unsafe_code, let_underscore_lock)]
#![deny(unused_extern_crates, unused_crate_dependencies)]
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
    app::{
        ui::{GetPost, ListPostMetadata, Main},
        ASSETS_PATH,
    },
    axum::routing::{get, post},
    const_format::formatcp,
    hyper::server::{accept::Accept, conn::AddrIncoming},
    leptos::{leptos_config::Env, view, LeptosOptions, ServerFn},
    leptos_axum::{generate_route_list, LeptosRoutes},
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        pin::Pin,
        task::{Context, Poll},
    },
    tower_governor::{governor::GovernorConfigBuilder, GovernorLayer},
    tower_http::{compression::CompressionLayer, services::fs::ServeDir},
};

mod config;
mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_backtrace::install();
    config::init()?;
    logging::init()?;

    async fn handler() -> String {
        "Hello, World!".to_string()
    }

    let leptos_options = LeptosOptions {
        output_name: String::from("app"),

        site_root: ".".into(),

        site_pkg_dir: ASSETS_PATH.into(),

        env: if cfg!(debug_assertions) {
            Env::DEV
        } else {
            Env::PROD
        },

        site_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 2506),

        reload_port: 0,
    };

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <Main/> }).await;

    GetPost::register().expect("failed to register GetPost");
    ListPostMetadata::register().expect("failed to register ListPostMetadata");

    // build our application with a single route
    let app = axum::Router::new()
        .route("/hello", get(handler))
        .route("/foo", get(|| async { "foobar" }))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options, routes, |cx| view! { cx, <Main/> })
        .nest_service(
            formatcp!("/{ASSETS_PATH}"),
            ServeDir::new("target/assets/debug").precompressed_br(),
        )
        .layer(
            tower::ServiceBuilder::new()
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
                                .per_second(
                                    crate::config::get().rate_limit_interval_per_second.into(),
                                )
                                .burst_size(crate::config::get().rate_limit_burst_size.into())
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
        );

    let server = axum::Server::builder(Listeners([
        AddrIncoming::bind(&SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 2506))?,
        AddrIncoming::bind(&SocketAddr::new(Ipv6Addr::LOCALHOST.into(), 2506))?,
    ]));

    tracing::debug!("ready");
    server
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    Ok(())
}

struct Listeners<const N: usize>([AddrIncoming; N]);

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
