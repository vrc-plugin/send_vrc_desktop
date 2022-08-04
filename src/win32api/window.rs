use anyhow::{anyhow, Result};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{FindWindowA, SetForegroundWindow},
    },
};

use crate::win32api::input::send_dummy_input;

pub fn find_window_by_name(name: &str) -> HWND {
    unsafe { FindWindowA(None, PCSTR(name.as_ptr())) }
}

pub fn set_foreground_window(hwnd: HWND) -> Result<()> {
    match unsafe { SetForegroundWindow(hwnd) }.ok() {
        Ok(_) => Ok(()),
        Err(_) => {
            send_dummy_input()?;

            unsafe { SetForegroundWindow(hwnd) }
                .ok()
                .map_err(|_| anyhow!("faile to set foreground window"))
        }
    }
}
