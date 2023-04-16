use crate::users::{CreateUser, User};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

pub async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User::new(1337, &payload.username);

    (StatusCode::CREATED, Json(user))
}
