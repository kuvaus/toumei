// src/platforms/unsupported.rs

use crate::{Error, Mode};

pub fn detect_tray_transparency() -> Result<Mode, Error> {
    Err(Error::UnsupportedPlatform)
}
