use axum::{
    routing::{post, put},
    Router,
};

use super::handler;

pub fn new() -> Router {
    Router::new().route("/url", post(handler::url).put(handler::url))
}
