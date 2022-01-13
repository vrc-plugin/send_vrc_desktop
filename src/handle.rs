use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::win32api::{clipboard, input, window};

#[derive(Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub message: String,
}

pub async fn url(Json(payload): Json<UrlRequest>) -> impl IntoResponse {
    let url = &payload.url;

    if let Err(e) = clipboard::set_clipboard(url) {
        return (
            StatusCode::BAD_REQUEST,
            Json(UrlResponse {
                message: format!("{}", e),
            }),
        );
    }

    let hwnd = match window::find_window_by_name("VRChat") {
        Ok(hwnd) => hwnd,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(UrlResponse {
                    message: format!("{}", e),
                }),
            );
        }
    };

    if let Err(e) = input::send_dummy_input() {
        return (
            StatusCode::BAD_REQUEST,
            Json(UrlResponse {
                message: format!("{}", e),
            }),
        );
    }

    if let Err(e) = window::set_foreground_window(hwnd) {
        return (
            StatusCode::BAD_REQUEST,
            Json(UrlResponse {
                message: format!("{}", e),
            }),
        );
    }

    if let Err(e) = input::send_paste_input() {
        return (
            StatusCode::BAD_REQUEST,
            Json(UrlResponse {
                message: format!("{}", e),
            }),
        );
    }

    if let Err(e) = input::send_enter_input() {
        return (
            StatusCode::BAD_REQUEST,
            Json(UrlResponse {
                message: format!("{}", e),
            }),
        );
    }

    (
        StatusCode::CREATED,
        Json(UrlResponse {
            message: String::from("ok"),
        }),
    )
}
