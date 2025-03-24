// src/platforms/windows.rs

use windows_sys::Win32::{
    Foundation::{S_OK},
    Graphics::Dwm::{ DwmGetWindowAttribute, DwmIsCompositionEnabled, DWMWA_COLOR_DEFAULT, DWMWA_SYSTEMBACKDROP_TYPE, },
    System::Diagnostics::ToolHelp::{ CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS, },
    UI::WindowsAndMessaging::{ FindWindowW, GetLayeredWindowAttributes, GetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, },
};

use crate::{Error, Mode};

// Constants for backdrop types
const DWMSBT_TRANSIENTWINDOW: u32 = 3;
const DWMSBT_TABBEDWINDOW: u32 = 4;


// Helper to create null-terminated UTF-16 strings
fn wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn is_translucenttb_running() -> bool {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        //if snapshot == !0 {
        if snapshot.is_null() {
            return false;
        }

        let mut entry: PROCESSENTRY32 = std::mem::zeroed();
        entry.dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        if Process32First(snapshot, &mut entry) == 0 {
            return false;
        }

        loop {
            let exe_name = std::ffi::CStr::from_ptr(entry.szExeFile.as_ptr())
                .to_string_lossy()
                .to_lowercase();

            if exe_name == "translucenttb.exe" {
                return true;
            }

            if Process32Next(snapshot, &mut entry) == 0 {
                break;
            }
        }

        false
    }
}



pub fn detect_tray_transparency() -> Result<Mode, Error> {
    // First check if TranslucentTB is running
    if is_translucenttb_running() {
        return Ok(Mode::Transparent);
    }

    // Then perform normal transparency checks
    let mut is_dwm_enabled = 0;
    let result = unsafe { DwmIsCompositionEnabled(&mut is_dwm_enabled) };
    if result != S_OK || is_dwm_enabled == 0 {
        return Ok(Mode::Opaque);
    }

    // Rest of the original transparency detection code...
    let primary_class = wide_string("Shell_TrayWnd");
    let secondary_class = wide_string("Shell_SecondaryTrayWnd");

    let hwnds = [
        unsafe { FindWindowW(primary_class.as_ptr(), std::ptr::null()) },
        unsafe { FindWindowW(secondary_class.as_ptr(), std::ptr::null()) },
    ];

    let mut has_transparency = false;
    let mut found_any_window = false;

    for hwnd in hwnds.iter().copied() {
        if hwnd.is_null() {
            continue;
        }
        found_any_window = true;

        // Check 1: Layered window attributes
        let ex_style = unsafe { GetWindowLongW(hwnd, GWL_EXSTYLE) } as u32;
        if (ex_style & WS_EX_LAYERED) != 0 {
            let mut alpha = 0;
            let mut flags = 0;
            let mut _color = 0;
            if unsafe { GetLayeredWindowAttributes(hwnd, &mut _color, &mut alpha, &mut flags) } != 0 {
                if alpha != 255 {
                    has_transparency = true;
                    break;
                }
            }
        }

        // Check 2: DWM color attribute
        let mut color: u32 = 0;
        let color_size = std::mem::size_of_val(&color) as u32;
        if unsafe { 
            DwmGetWindowAttribute(
                hwnd, 
                DWMWA_COLOR_DEFAULT, 
                &mut color as *mut _ as *mut _, 
                color_size
            ) 
        } == S_OK {
            let alpha = (color >> 24) & 0xFF;
            if alpha < 255 {
                has_transparency = true;
                break;
            }
        }

        // Check 3: DWM backdrop type (Windows 11)
        let mut backdrop_type: u32 = 0;
        let backdrop_size = std::mem::size_of_val(&backdrop_type) as u32;
        if unsafe { 
            DwmGetWindowAttribute(
                hwnd, 
                DWMWA_SYSTEMBACKDROP_TYPE.try_into().unwrap(), 
                &mut backdrop_type as *mut _ as *mut _, 
                backdrop_size
            ) 
        } == S_OK {
            if backdrop_type == DWMSBT_TRANSIENTWINDOW || backdrop_type == DWMSBT_TABBEDWINDOW {
                has_transparency = true;
                break;
            }
        }
    }

    if !found_any_window {
        return Err(Error::WindowNotFound);
    }

    Ok(if has_transparency {
        Mode::Transparent
    } else {
        Mode::Opaque
    })
}