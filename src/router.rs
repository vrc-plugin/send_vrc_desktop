use axum::{
    routing::{post, put},
    Router,
};

use crate::handle;

pub fn new() -> Router {
    Router::new()
        .route("/url", post(handle::url))
        .route("/url", put(handle::url))
}
