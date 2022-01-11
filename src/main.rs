use anyhow::{anyhow, ensure, Result};

use windows::Win32::Foundation::{HANDLE, PSTR};
use windows::Win32::Globalization::lstrcpyA;
use windows::Win32::System::DataExchange::{
    CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
};
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_0, KEYBDINPUT,
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
    let result = unsafe { OpenClipboard(None) };
    ensure!(result.as_bool(), "failed to open clipboard");

    let result = unsafe { EmptyClipboard() };
    ensure!(result.as_bool(), "failed to initialize clipboard");

    const GHND: u32 = 0x0042;
    let hmem = unsafe { GlobalAlloc(GHND, value.len() + 1) };

    let lpstring1 = PSTR(unsafe { GlobalLock(hmem) as _ });
    unsafe { lstrcpyA(lpstring1, value) };
    let result = unsafe { GlobalUnlock(hmem) };
    ensure!(!result.as_bool(), "failed to unlock memory");

    const CF_TEXT: u32 = 1;
    let handle = unsafe { SetClipboardData(CF_TEXT, HANDLE(hmem)) };
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
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0xA2, // VK_LCONTROL
                    wScan: unsafe { MapVirtualKeyA(0xA2, 0) } as u16,
                    dwFlags: 0x0000, // NONE
                    time: 0,
                    dwExtraInfo: 0x0000,
                },
            },
        },
        INPUT {
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0x56, // VK_V
                    wScan: unsafe { MapVirtualKeyA(0x56, 0) } as u16,
                    dwFlags: 0x0000, // NONE
                    time: 0,
                    dwExtraInfo: 0x0000,
                },
            },
        },
        INPUT {
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0x56, // VK_V
                    wScan: unsafe { MapVirtualKeyA(0x56, 0) } as u16,
                    dwFlags: 0x0002, // KEYEVENTF_KEYUP
                    time: 0,
                    dwExtraInfo: 0x0000,
                },
            },
        },
        INPUT {
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0xA2, // VK_LCONTROL
                    wScan: unsafe { MapVirtualKeyA(0xA2, 0) } as u16,
                    dwFlags: 0x0002, // KEYEVENTF_KEYUP
                    time: 0,
                    dwExtraInfo: 0x0000,
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
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0x0D, // VK_RETURN
                    wScan: unsafe { MapVirtualKeyA(0x0D, 0) } as u16,
                    dwFlags: 0x0000, // NONE
                    time: 0,
                    dwExtraInfo: 0x0000,
                },
            },
        },
        INPUT {
            r#type: 0x0001, // INPUT_KEYBOARD
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: 0x0D, // VK_RETURN
                    wScan: unsafe { MapVirtualKeyA(0x0D, 0) } as u16,
                    dwFlags: 0x0002, // KEYEVENTF_KEYUP
                    time: 0,
                    dwExtraInfo: 0x0000,
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
