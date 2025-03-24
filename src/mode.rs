// src/mode.rs

#[derive(Copy, Clone, PartialEq, Eq, Debug)]  // <-- Already present in your code
pub enum Mode {
    Transparent,
    Opaque,
}

impl From<bool> for Mode {
    fn from(value: bool) -> Self {
        if value {
            Mode::Transparent
        } else {
            Mode::Opaque
        }
    }
}
