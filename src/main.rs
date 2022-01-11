use windows::Win32::UI::Input::KeyboardAndMouse::{
    MapVirtualKeyA, SendInput, INPUT, INPUT_0, KEYBDINPUT,
};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowA, SetForegroundWindow};

fn main() {
    let hwnd = unsafe { FindWindowA(None, "VRChat") };
    let result = unsafe { SetForegroundWindow(hwnd) };
    dbg!(result.as_bool());

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

    let result = unsafe {
        SendInput(
            inputs.len() as u32,
            inputs.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    dbg!(result);
}
