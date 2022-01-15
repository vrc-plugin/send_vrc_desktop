use axum::{
    routing::{post, put},
    Router,
};

use super::handle;

pub fn new() -> Router {
    Router::new()
        .route("/url", post(handle::url))
        .route("/url", put(handle::url))
}
