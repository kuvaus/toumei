// src/test/mod.rs

use super::*;

#[test]
fn test_transparency_detection() {
    match detect_tray_transparency() {
        Ok(mode) => {
            if cfg!(target_os = "macos") {
                println!("macOS Tray Transparency: {:?}", mode);
            } else if cfg!(target_os = "windows") {
                println!("Windows Transparency: {:?}", mode);
            } else if cfg!(target_os = "linux") {
                println!("Linux Transparency: {:?}", mode);
            }
            assert!(true);  // Just verify we got a valid mode
        }
        Err(e) => {
            if cfg!(not(any(
                target_os = "macos",
                target_os = "windows",
                target_os = "linux"
            ))) {
                assert!(matches!(e, Error::UnsupportedPlatform));
            } else {
                panic!("Detection failed on supported platform: {}", e);
            }
        }
    }
}