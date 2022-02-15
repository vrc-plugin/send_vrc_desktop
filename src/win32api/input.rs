use std::mem::size_of;
use std::time::Duration;

use anyhow::{anyhow, Result};
use tokio::time::sleep;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_EXTENDEDKEY,
    KEYEVENTF_KEYUP, VK_CONTROL, VK_RETURN, VK_V,
};

fn send_input(inputs: &[INPUT]) -> Result<(), ()> {
    match unsafe { SendInput(inputs.len() as _, inputs.as_ptr(), size_of::<INPUT>() as _) } {
        0 => Err(()),
        _ => Ok(()),
    }
}

pub fn send_dummy_input() -> Result<()> {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
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
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    send_input(&inputs).map_err(|_| anyhow!("failed to send dummy (`Ctrl`) input"))?;

    Ok(())
}

pub async fn send_paste_input() -> Result<()> {
    const ERROR_MESSAGE: &str = "failed to send `Paste` (`Ctrl` + `V`) input";
    const DURATION: Duration = Duration::from_millis(100);

    let inputs = [
        // Left Ctrl Down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Right Ctrl Down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
                    dwFlags: KEYEVENTF_EXTENDEDKEY,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    send_input(&inputs).map_err(|_| anyhow!(ERROR_MESSAGE))?;

    sleep(DURATION).await;

    let inputs = [
        // V Down
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: unsafe { MapVirtualKeyA(VK_V as _, 0) } as _,
                    dwFlags: 0,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // V Up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_V,
                    wScan: unsafe { MapVirtualKeyA(VK_V as _, 0) } as _,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    send_input(&inputs).map_err(|_| anyhow!(ERROR_MESSAGE))?;

    sleep(DURATION).await;

    let inputs = [
        // Left Ctrl Up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
        // Right Ctrl Up
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_CONTROL,
                    wScan: unsafe { MapVirtualKeyA(VK_CONTROL as _, 0) } as _,
                    dwFlags: KEYEVENTF_KEYUP | KEYEVENTF_EXTENDEDKEY,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    send_input(&inputs).map_err(|_| anyhow!(ERROR_MESSAGE))?;

    Ok(())
}

pub fn send_enter_input() -> Result<()> {
    let inputs = [
        INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VK_RETURN,
                    wScan: unsafe { MapVirtualKeyA(VK_RETURN as _, 0) } as _,
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
                    wScan: unsafe { MapVirtualKeyA(VK_RETURN as _, 0) } as _,
                    dwFlags: KEYEVENTF_KEYUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        },
    ];
    send_input(&inputs).map_err(|_| anyhow!("failed to send `Enter` input"))?;

    Ok(())
}
