pub mod routes;
pub mod users;

#[cfg(test)]
mod tests {
    use crate::routes::create_route;
    use crate::users::User;
    use assert2::check;
    use axum::http::{header, Method, Request};
    use hyper::Body;
    use std::error::Error;
    use tower::ServiceExt;

    #[tokio::test]
    async fn should_return_hello_world() -> Result<(), Box<dyn Error>> {
        let req = Request::builder().uri("/").body(Body::empty())?;
        let res = create_route().oneshot(req).await?;
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;

        check!(body == "Hello, World");

        Ok(())
    }

    #[tokio::test]
    async fn should_return_user() -> Result<(), Box<dyn Error>> {
        let req = Request::builder()
            .uri("/users")
            .method(Method::POST)
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                r#"
            {
                "username": "山田 太郎"
            }
            "#,
            ))?;
        let res = create_route().oneshot(req).await?;
        let bytes = hyper::body::to_bytes(res.into_body()).await?;
        let body = String::from_utf8(bytes.to_vec())?;
        let user = serde_json::from_str::<User>(&body)?;

        check!(user == User::new(1337, "山田 太郎"));

        Ok(())
    }
}
