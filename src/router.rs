use axum::{routing::put, Router};

use crate::handle;

pub fn new() -> Router {
    Router::new().route("/url", put(handle::url))
}
