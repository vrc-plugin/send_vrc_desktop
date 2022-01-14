use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub struct ApiError {
    err: anyhow::Error,
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> ApiError {
        ApiError { err }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = (StatusCode::BAD_REQUEST, format!("{}", self.err));
        let body = Json(ErrorResponse { error: message });
        (status, body).into_response()
    }
}
