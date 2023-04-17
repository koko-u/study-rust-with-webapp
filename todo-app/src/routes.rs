use std::sync::Arc;

use axum::{Extension, Router};
use axum::routing::{get, post};

use crate::routes::create_todo::create_todo;
use crate::routes::root::root;
use crate::todos::repositories::TodoRepository;

pub fn create_route<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>))
        .layer(Extension(Arc::new(repository)))
}

mod create_todo;
mod root;
