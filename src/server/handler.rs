use std::time::Duration;
use anyhow::{anyhow, Result};
use axum::Json;
use clipboard_win::set_clipboard_string;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

use crate::win32api::{input, window};

use super::error::ApiError;

#[derive(Deserialize)]
pub struct UrlRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct UrlResponse {
    pub message: String,
}

pub async fn url(Json(payload): Json<UrlRequest>) -> Result<Json<UrlResponse>, ApiError> {
    const INPUT_WAIT_DURATION: Duration = Duration::from_millis(500);
    let url = &payload.url;
    println!("received url: {url}");

    set_clipboard_string(url).map_err(|_| anyhow!("failed to set clipboard"))?;

    let hwnd = window::find_window_by_name("VRChat");
    window::set_foreground_window(hwnd)?;

    sleep(INPUT_WAIT_DURATION).await;
    input::send_dummy_input()?;
    window::set_foreground_window(hwnd)?;

    sleep(INPUT_WAIT_DURATION).await;
    input::send_paste_input().await?;
    input::send_enter_input()?;

    Ok(Json(UrlResponse {
        message: String::from("ok"),
    }))
}
