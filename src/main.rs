use anyhow::{anyhow, ensure, Result};

use windows::Win32::Foundation::{HANDLE, PWSTR};
use windows::Win32::Globalization::lstrcpyW;
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GHND};
use windows::Win32::System::SystemServices::CF_UNICODETEXT;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
    VK_LCONTROL, VK_RETURN, VK_V,
};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SetForegroundWindow};

fn find_window_by_name(name: &str) -> Result<isize> {
    let hwnd = unsafe { FindWindowA(None, name) };
    ensure!(hwnd != 0, "`{}` does not exists", name);
    Ok(hwnd)
}

fn set_foreground_window(hwnd: isize) -> Result<()> {
    let result = unsafe { SetForegroundWindow(hwnd) };
    ensure!(result.as_bool(), "failed to set foreground window");
    Ok(())
}

fn set_clipboard(value: &str) -> Result<()> {
    let mut value: Vec<u16> = value.encode_utf16().chain(Some(0)).collect();

    let result = unsafe { OpenClipboard(None) };
    ensure!(result.as_bool(), "failed to open clipboard");

    let result = unsafe { EmptyClipboard() };
    ensure!(result.as_bool(), "failed to initialize clipboard");

    let hmem = unsafe { GlobalAlloc(GHND, value.len() * std::mem::size_of::<u16>()) };
    ensure!(hmem != 0, "failed to allocate");

    let mem_ptr = unsafe { GlobalLock(hmem) } as *mut u16;
    ensure!(!mem_ptr.is_null(), "failed to lock");

    let pwstr = unsafe { lstrcpyW(PWSTR(mem_ptr), PWSTR(value.as_mut_ptr())) };
    ensure!(!pwstr.is_null(), "failed to lstrcpy");

    let result = unsafe { GlobalUnlock(hmem) };
    ensure!(!result.as_bool(), "failed to unlock");

    let handle = unsafe { SetClipboardData(CF_UNICODETEXT, HANDLE(hmem)) };
    ensure!(!handle.is_invalid(), "failed to set data to clipboard");

    let result = unsafe { CloseClipboard() };
    ensure!(result.as_bool(), "failed to close clipboard");

    Ok(())
}

fn send_input(inputs: &[INPUT]) -> Result<()> {
    let result = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    ensure!(result != 0);
    Ok(())
}

fn send_paste_input() -> Result<()> {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as u32, 0) } as u16,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: unsafe { MapVirtualKeyA(VK_V as u32, 0) } as u16,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: unsafe { MapVirtualKeyA(VK_V as u32, 0) } as u16,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_LCONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as u32, 0) } as u16,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    let result = send_input(&inputs);
    ensure!(result.is_ok(), "failed to send `Paste`(`Ctrl` + `V`) input");
    Ok(())
}

fn send_enter_input() -> Result<()> {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_RETURN,
                    wScan: unsafe { MapVirtualKeyA(VK_RETURN as u32, 0) } as u16,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_RETURN,
                    wScan: unsafe { MapVirtualKeyA(VK_RETURN as u32, 0) } as u16,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    let result = send_input(&inputs);
    ensure!(result.is_ok(), "failed to send `Enter` input");
    Ok(())
}

fn main() -> Result<()> {
    let url = std::env::args()
        .nth(1)
        .ok_or(anyhow!("argument is required"))?;

    let hwnd = find_window_by_name("VRChat")?;
    set_foreground_window(hwnd)?;

    set_clipboard(&url)?;
    send_paste_input()?;
    send_enter_input()?;

    Ok(())
}
