use std::sync::Arc;

use axum::routing::{delete, get, patch, post};
use axum::{Extension, Router};

use crate::routes::root::root;
use crate::routes::todos::{create_todo, delete_todo, get_all_todo, get_single_todo, update_todo};
use crate::todos::repositories::TodoRepository;

pub fn create_route<T: TodoRepository>(repository: T) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/todos", post(create_todo::<T>).get(get_all_todo::<T>))
        .route(
            "/todos/:id",
            get(get_single_todo::<T>)
                .delete(delete_todo::<T>)
                .patch(update_todo::<T>),
        )
        .layer(Extension(Arc::new(repository)))
}

mod todos;

mod root;
