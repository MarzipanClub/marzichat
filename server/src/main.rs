use axum::{routing::get, Router};

mod config;
mod logging;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // set up nice backtraces
    color_backtrace::install();

    config::init()?;
    logging::init()?;

    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
