// src/capi.rs
use crate::{Mode as RustMode, Error as RustError};

// This is a traditional C style FFI wrapper for Toumei
#[repr(C)]
pub enum ToumeiResult {
    Ok,
    ErrUnsupportedPlatform,
    ErrWindowNotFound,
    ErrOther,
}

#[repr(C)]
pub enum ToumeiMode {
    Transparent,
    Opaque,
}

#[unsafe(no_mangle)]
pub extern "C" fn detect_tray_transparency(
    mode: *mut ToumeiMode,
) -> ToumeiResult {
    
    if mode.is_null() {
        return ToumeiResult::ErrOther;
    }
    
    let result = crate::detect_tray_transparency();
    
    match result {
        Ok(m) => {
            unsafe {
                *mode = match m {
                    RustMode::Transparent => ToumeiMode::Transparent,
                    RustMode::Opaque => ToumeiMode::Opaque,
                };
            }
            ToumeiResult::Ok
        }
        Err(e) => match e {
            RustError::UnsupportedPlatform => ToumeiResult::ErrUnsupportedPlatform,
            RustError::WindowNotFound => ToumeiResult::ErrWindowNotFound,
            _ => ToumeiResult::ErrOther,
        },
    }
}
