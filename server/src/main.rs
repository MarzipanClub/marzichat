use axum::routing::get;

mod config;
mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up nice backtraces
    color_backtrace::install();

    config::init()?;
    logging::init()?;

    async fn handler() -> String {
        "Hello, World!".to_string()
    }

    // build our application with a single route
    let app = axum::Router::new()
        .route("/", get(handler))
        .route("/foo", get(|| async { "foo" }))
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
                    tower_governor::GovernorLayer {
                        config: Box::leak(Box::new(
                            // allow bursts with up to rate_limit_interval_per_second requests per
                            // ip address and replenishes one element
                            // every rate_limit_burst_size seconds
                            tower_governor::governor::GovernorConfigBuilder::default()
                                .per_second(
                                    crate::config::get().rate_limit_interval_per_second.into(),
                                )
                                .burst_size(crate::config::get().rate_limit_burst_size.into())
                                .finish()
                                .ok_or(anyhow::anyhow!("invalid rate limiting configuration"))?,
                        )),
                    },
                )
                .layer(tower_http::compression::CompressionLayer::new()),
        );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service_with_connect_info::<std::net::SocketAddr>())
        .await?;

    Ok(())
}
