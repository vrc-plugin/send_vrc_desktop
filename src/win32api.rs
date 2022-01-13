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

pub mod clipboard {
  use std::num::NonZeroIsize;
  use anyhow::{bail, ensure, Result};

  use windows::Win32::Foundation::{GetLastError, HANDLE, NO_ERROR, PWSTR};
  use windows::Win32::Globalization::lstrcpyW;
  use windows::Win32::System::DataExchange::{
      CloseClipboard, EmptyClipboard, OpenClipboard, SetClipboardData,
  };
  use windows::Win32::System::Memory::{GlobalAlloc, GlobalFree, GlobalLock, GlobalUnlock, GHND, GLOBAL_ALLOC_FLAGS};
  use windows::Win32::System::SystemServices::CF_UNICODETEXT;

  type NonZeroHANDLE = NonZeroIsize;

  struct GStr {
      hmem: Option<NonZeroHANDLE>,
  }

  impl GStr {
      fn try_new(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> Result<Self> {
          let hmem = unsafe { GlobalAlloc(uflags, dwbytes) };
          ensure!(hmem != 0, "failed to allocate : {}", unsafe { GetLastError() });
          Ok(GStr { hmem: NonZeroHANDLE::new(hmem) })
      }

      fn try_from(s: &str) -> Result<Self> {
          let mut s: Vec<_> = s.encode_utf16().chain(Some(0)).collect();
          let gs = Self::try_new(GHND, s.len() * std::mem::size_of::<u16>())?;
          let result = unsafe { lstrcpyW(PWSTR(gs.lock()?), PWSTR(s.as_mut_ptr())) };
          ensure!(!result.is_null(), "failed to write : {}", unsafe { GetLastError() });
          gs.unlock()?;
          Ok(gs)
      }

      fn lock(&self) -> Result<*mut u16> {
          if let Some(hmem) = self.hmem {
              let result = unsafe { GlobalLock(hmem.get()) };
              ensure!(!result.is_null(), "failed to lock : {}", unsafe { GetLastError() } );
              Ok(result as _)
          } else {
              bail!("lock called on empty GStr")
          }
      }

      fn unlock(&self) -> Result<()> {
          if let Some(hmem) = self.hmem {
              let result = unsafe { GlobalUnlock(hmem.get()) };
              if !result.as_bool() {
                  let err = unsafe { GetLastError() };
                  ensure!(err == NO_ERROR, "failed to unlock : {}", err);
              }
              Ok(())
          } else {
              bail!("unlock called on empty GStr")
          }
      }
  }

  impl Drop for GStr {
      fn drop(&mut self) {
          if let Some(hmem) = self.hmem {
              let result = unsafe { GlobalFree(hmem.get()) };
              assert!(result != 0, "failed to free : {}", unsafe { GetLastError() });
          }
      }
  }

  struct Clipboard {
      _dummy: (),
  }

  impl Clipboard {
      fn open() -> Result<Self> {
          let result = unsafe { OpenClipboard(0) };
          ensure!(result.as_bool(), "failed to open clipboard : {}", unsafe { GetLastError() });
          Ok(Self { _dummy: () })
      }

      fn empty(&self) -> Result<()> {
          let result = unsafe { EmptyClipboard() };
          ensure!(result.as_bool(), "failed to initialize clipboard : {}", unsafe { GetLastError() });
          Ok(())
      }

      fn set(&self, s: &str) -> Result<()> {
          self.empty()?;
          let mut gs = GStr::try_from(s)?;
          let handle = unsafe { SetClipboardData(CF_UNICODETEXT, HANDLE(gs.hmem.unwrap().get())) };
          ensure!(!handle.is_invalid(), "failed to set data to clipboard : {}", unsafe { GetLastError() });
          gs.hmem = None;
          Ok(())
      }
  }

  impl Drop for Clipboard {
      fn drop(&mut self) {
          let result = unsafe { CloseClipboard() };
          assert!(result.as_bool(), "failed to close clipboard : {}", unsafe { GetLastError() });
      }
  }

  pub fn set_clipboard(value: &str) -> Result<()> {
      Clipboard::open()?.set(value)
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
