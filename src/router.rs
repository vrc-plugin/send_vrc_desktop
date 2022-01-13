use axum::{routing::post, Router};

use crate::handle;

pub fn new() -> Router {
    Router::new().route("/url", post(handle::url))
}
