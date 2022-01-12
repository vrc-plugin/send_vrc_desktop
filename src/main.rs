mod win32api;

use anyhow::{anyhow, Result};

use win32api::{clipboard, input, window};

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .ok_or(anyhow!("argument is required"))?;

    clipboard::set_clipboard(&url)?;

    let hwnd = window::find_window_by_name("VRChat")?;
    window::set_foreground_window(hwnd)?;

    input::send_paste_input()?;
    input::send_enter_input()?;

    Ok(())
}
