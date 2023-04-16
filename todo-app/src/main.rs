use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() -> hyper::Result<()> {
    let app = Router::new().route("/", get(root));
    axum::Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World"
}
