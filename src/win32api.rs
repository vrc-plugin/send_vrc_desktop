pub mod window {
    use anyhow::{ensure, Result};
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SetForegroundWindow};

    use crate::win32api::input::send_dummy_input;

    pub fn find_window_by_name(name: &str) -> Result<isize> {
        let hwnd = unsafe { FindWindowA(None, name) };
        ensure!(hwnd != 0, "`{}` does not exists", name);
        Ok(hwnd)
    }

    pub fn set_foreground_window(hwnd: isize) -> Result<()> {
        let result = unsafe { SetForegroundWindow(hwnd) };
        if !result.as_bool() {
            send_dummy_input()?;

            let result = unsafe { SetForegroundWindow(hwnd) };
            ensure!(result.as_bool(), "failed to set foreground window");
        }

        Ok(())
    }
}

pub mod input {
    use anyhow::{ensure, Result};

    use windows::Win32::UI::Input::KeyboardAndMouse::{
        MapVirtualKeyA, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYEVENTF_KEYUP,
        VK_LCONTROL, VK_RETURN, VK_V,
    };

    fn send_input(inputs: &[INPUT]) -> Result<()> {
        let result = unsafe {
            SendInput(
                inputs.len() as _,
                inputs.as_ptr(),
                std::mem::size_of::<INPUT>() as _,
            )
        };
        ensure!(result != 0);
        Ok(())
    }

    pub fn send_dummy_input() -> Result<()> {
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_LCONTROL,
                        wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as _, 0) } as _,
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
                        wVk: VK_LCONTROL,
                        wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as _, 0) } as _,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    },
                },
            },
        ];
        let result = send_input(&inputs);
        ensure!(result.is_ok(), "failed to send dummy (`Ctrl`) input");
        Ok(())
    }

    pub fn send_paste_input() -> Result<()> {
        let inputs = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: VK_LCONTROL,
                        wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as _, 0) } as _,
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
                        wScan: unsafe { MapVirtualKeyA(VK_V as _, 0) } as _,
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
                        wScan: unsafe { MapVirtualKeyA(VK_V as _, 0) } as _,
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
                        wScan: unsafe { MapVirtualKeyA(VK_LCONTROL as _, 0) } as _,
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
        let result = send_input(&inputs);
        ensure!(result.is_ok(), "failed to send `Enter` input");
        Ok(())
    }
}
