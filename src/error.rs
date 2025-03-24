// src/error.rs

use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    XdgDesktopPortal(String),
    Timeout,
    PersistentDomainFailed,
    WindowNotFound,
    MediaQueryFailed,
    MediaQueryNotSupported,
    WindowsApi(String),
    UnsupportedPlatform,
    CssCheck(String),  // Add this new variant
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::XdgDesktopPortal(e) => write!(f, "XDG Portal error: {}", e),
            Error::Timeout => write!(f, "Timeout"),
            Error::PersistentDomainFailed => write!(f, "Failed to get Apple domain"),
            Error::WindowNotFound => write!(f, "Window not found"),
            Error::MediaQueryFailed => write!(f, "Media query failed"),
            Error::MediaQueryNotSupported => write!(f, "Media query unsupported"),
            Error::WindowsApi(e) => write!(f, "Windows API error: {}", e),
            Error::UnsupportedPlatform => write!(f, "Unsupported platform"),
            Error::CssCheck(e) => write!(f, "CSS check error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}