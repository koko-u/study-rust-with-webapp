use std::error::Error;

use assert2::check;
use axum::http::Request;
use hyper::Body;
use tower::ServiceExt;

use todo_app::routes::create_route;
use todo_app::todos::repositories::in_memory::TodoRepositoryInMemory;

#[tokio::test]
async fn should_return_hello_world() -> Result<(), Box<dyn Error>> {
    let req = Request::builder().uri("/").body(Body::empty())?;
    let res = create_route(TodoRepositoryInMemory::new())
        .oneshot(req)
        .await?;
    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(bytes.to_vec())?;

    check!(body == "Hello, World");

    Ok(())
}
