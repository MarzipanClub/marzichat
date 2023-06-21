//! The main entry point for the gateway.

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
        body::Body as AxumBody,
        extract::{Path, RawQuery, State},
        http::{header::HeaderMap, Request},
        response::{IntoResponse, Response},
        routing::{get, post},
        Router,
    },
    axum_login::AuthLayer,
    axum_sessions::{
        async_session::CookieStore,
        extractors::{ReadableSession, WritableSession},
        SessionLayer,
    },
    common::types::{account, account::Account},
    const_format::formatcp,
    hyper::{
        server::{accept::Accept, conn::AddrIncoming},
        Server,
    },
    leptos::{leptos_config::Env, view, LeptosOptions},
    leptos_axum::{generate_route_list, handle_server_fns_with_context, LeptosRoutes},
    rand::{prelude::ThreadRng, Rng},
    std::{
        net::{Ipv4Addr, Ipv6Addr, SocketAddr},
        pin::Pin,
        task::{Context, Poll},
    },
    tower::ServiceBuilder,
    tower_governor::{governor::GovernorConfigBuilder, GovernorLayer},
    tower_http::{
        compression::CompressionLayer,
        services::{fs::ServeDir, ServeFile},
    },
};

mod config;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    color_backtrace::install();
    config::init()?;
    logging::init()?;

    async fn handler() -> String {
        "Hello, World!".to_string()
    }

    let config = crate::config::get();

    let session_layer =
        SessionLayer::new(CookieStore::new(), &config.cookie_signing_key).with_secure(false);

    // let user_store = AuthMemoryStore::new(&store);
    // let auth_layer = AuthLayer::new(user_store, &secret);

    let assets_path = &config.static_assets_path;
    let router = Router::new()
        .route("/hello", get(handler))
        .route("/foo", get(|| async { "foobar" }))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(
            LeptosOptions {
                output_name: "app".into(),
                site_root: ".".into(),
                site_pkg_dir: ASSETS_PATH.into(),
                env: Env::PROD, // no
                site_addr: SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 0),
                reload_port: 0, // not used
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
        )
        .layer({
            SessionLayer::new(CookieStore::new(), &config.cookie_signing_key)
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

use {axum::extract::FromRef, leptos::provide_context};

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This
/// is the only way to have more than one item in Axum's State. Leptos requires
/// you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Debug, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    // pub pool: SqlitePool,
}

// pub type AuthSession = axum_session_auth::AuthSession<User, i64,
// SessionSqlitePool, SqlitePool>

async fn server_fn_handler(
    State(app_state): State<AppState>,
    // auth_session: AuthSession,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            // provide_context(cx, auth_session.clone());
            // provide_context(cx, app_state.pool.clone());
        },
        request,
    )
    .await
}

// type AuthContext = axum_login::extractors::AuthContext<account::Id, Account,
// SqliteStore<User>>;

// async fn leptos_routes_handler(
//     // auth_session: AuthSession,
//     State(app_state): State<AppState>,
//     req: Request<AxumBody>,
// ) -> Response {
//     let handler = leptos_axum::render_app_to_stream_with_context(
//         app_state.leptos_options.clone(),
//         move |cx| {
//             // provide_context(cx, auth_session.clone());
//             // provide_context(cx, app_state.pool.clone());
//         },
//         |cx| view! { cx, <TodoApp/> },
//     );
//     handler(req).await.into_response()
// }
