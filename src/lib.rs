// src/lib.rs
//! Detect system tray transparency across different platforms
//!
//! This crate provides functionality to detect if the system tray (or equivalent)
//! is using transparent or opaque styling. Supported platforms include:
//! - macOS
//! - Windows
//! - Linux/BSD (via XDG Desktop Portal and GNOME extensions)
//!
//! # Example
//! ```
//! fn main() -> Result<(), toumei::Error> {
//!     match toumei::detect_tray_transparency()? {
//!         toumei::Mode::Transparent => println!("Transparent tray detected!"),
//!         toumei::Mode::Opaque => println!("Opaque tray detected"),
//!     }
//!     Ok(())
//! }
//! ```

mod error;
mod mode;
mod platforms;

pub use error::Error;
pub use mode::Mode;
pub use platforms::detect_tray_transparency;

#[cfg(feature = "capi")]
pub mod capi;

#[cfg(test)]
mod test;

