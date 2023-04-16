use axum::routing::{get, post};
use axum::Router;

use crate::routes::create_user::create_user;
use crate::routes::root::root;

pub fn create_route() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", post(create_user))
}

mod create_user;
mod root;
