use anyhow::Error;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

pub struct ApiError(Error);

impl From<Error> for ApiError {
    fn from(err: Error) -> ApiError {
        ApiError(err)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = StatusCode::BAD_REQUEST;
        let body = Json(ApiErrorResponse {
            error: format!("{}", self.0),
        });
        (status, body).into_response()
    }
}
