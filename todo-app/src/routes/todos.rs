use std::sync::Arc;

use axum::extract::{Extension, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;

use crate::todos::models::{CreateTodo, UpdateTodo};
use crate::todos::repositories::TodoRepository;

/// POST /todos
pub async fn create_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Json(payload): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = repository.create(payload);

    (StatusCode::CREATED, Json(todo))
}

/// GET /todos/:id
pub async fn get_single_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<u64>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = repository.find(id);

    Ok(StatusCode::OK)
}

/// Get /todos
pub async fn get_all_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
) -> impl IntoResponse {
    let todos = repository.all();

    (StatusCode::OK, Json(todos))
}

/// PATCH /todos/:id
pub async fn update_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<u64>,
    Json(payload): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    Ok(StatusCode::OK)
}

/// DELETE /todos/:id
pub async fn delete_todo<T: TodoRepository>(
    Extension(repository): Extension<Arc<T>>,
    Path(id): Path<u64>,
) -> StatusCode {
    StatusCode::OK
}
