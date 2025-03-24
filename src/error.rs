// src/error.rs
use std::fmt::Display;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    XdgDesktopPortal(String),
    PersistentDomainFailed,
    WindowNotFound,
    UnsupportedPlatform,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::XdgDesktopPortal(e) => write!(f, "XDG Portal error: {e}"),
            Self::PersistentDomainFailed => write!(f, "Failed to get Apple domain"),
            Self::WindowNotFound => write!(f, "Window not found"),
            Self::UnsupportedPlatform => write!(f, "Unsupported platform"),
        }
    }
}

impl std::error::Error for Error {}