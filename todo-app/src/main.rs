use axum::routing::get;
use axum::Router;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let log_level = env::var("RUST_LOG").unwrap_or("INFO".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let routes = Router::new().route("/", get(root));
    let app = axum::Server::bind(&([127, 0, 0, 1], 8080).into()).serve(routes.into_make_service());

    info!("Listening on {}", app.local_addr());

    app.await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World"
}
