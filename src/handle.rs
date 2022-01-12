use actix_web::web::Json;
use actix_web::{post, HttpResponse, Responder};
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

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[post("/url")]
pub async fn url(req: Json<UrlRequest>) -> impl Responder {
    let url = &req.url;

    if let Err(e) = clipboard::set_clipboard(url) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{}", e),
        });
    }

    let hwnd = match window::find_window_by_name("VRChat") {
        Ok(hwnd) => hwnd,
        Err(e) => {
            return HttpResponse::BadRequest().json(ErrorResponse {
                error: format!("{}", e),
            });
        }
    };

    if let Err(e) = input::send_dummy_input() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{}", e),
        });
    }

    if let Err(e) = window::set_foreground_window(hwnd) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{}", e),
        });
    }

    if let Err(e) = input::send_paste_input() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{}", e),
        });
    }

    if let Err(e) = input::send_enter_input() {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: format!("{}", e),
        });
    }

    HttpResponse::Ok().json(UrlResponse {
        message: String::from("ok"),
    })
}
