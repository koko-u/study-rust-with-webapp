use std::env;

use tracing::info;

use todo_app::routes::create_route;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let log_level = env::var("RUST_LOG").unwrap_or("INFO".into());
    env::set_var("RUST_LOG", log_level);
    tracing_subscriber::fmt::init();

    let routes = create_route();
    let app = axum::Server::bind(&([127, 0, 0, 1], 8080).into()).serve(routes.into_make_service());

    info!("Listening on {}", app.local_addr());

    app.await?;

    Ok(())
}
