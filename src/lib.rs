// src/lib.rs

//! This crate is designed to provide a simple API to detect the current tray transparency setting

mod error;
mod mode;
mod platforms;

pub use error::Error;
pub use mode::Mode;
pub use platforms::detect_tray_transparency;

// Add this conditional test module inclusion
#[cfg(test)]
mod test;