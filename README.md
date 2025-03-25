
<div align="center">
    <img src="resources/logo.png" width="200"/>
    <h1>toumei</h1>
    <p>Rust crate to detect if system tray is transparent or opaque</p>
    <a href="https://crates.io/crates/toumei"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/toumei?style=for-the-badge"></a>
    <br>
    <br>
</div>

## Info

Supports **macOS**, **Windows**, and **Linux**.

  * On **macOS** it detects transparency of the **System Tray (Top Bar)**. The setting detects if it is transparent or opaque. It checks the transparency by seeing if the Reduce Transparency setting in Universal Access is turned on or off.
  * On **Windows** it detects the transparency of the **Windows Taskbar**. It first checks if TranslucentTB is changing the Taskbar transparency. It also checks if either the Layered window attributes, DWM color attribute, or DWM backdrop type (Windows 11) is changing the Taskbar transparency.
  * On **Linux** it detects the transparency of the **Top Bar**. It checks if XDG Desktop Portal has set the Transparency. On GNOME it also checks if gnome-extension transparent-top-bar is changing the Top Bar transparency.


[API Documentation](https://docs.rs/toumei/)

<small>**Toumei:** 透明 【とうめい】tōmei, meaning: transparent; clear.</small>

## Why use this library?

Let's say you are making an application with an icon in system tray, or an app with tray notifications or widgets. It is common to have a light icon for dark theme (so the icon stands out) and a dark icon on light theme, with the icons changing dynamically. But when the user has set their system tray to be transparent, most likely the themed icons and widgets get buried depending on the wallpaper color. And at least on certain versions of MacOS the system tray is set to be semi-transparent by default. In those cases when one is using the Light theme, it is sometimes better to use the (bright) dark theme icon. This is why it is worth to  find out the transparency setting of the system tray dynamically. This crate provides a single function `detect_tray_transparency()` to see the current transparency setting of the system tray. There are also C bindings for using this library with other programming languages.


## Rust Usage

Add to your project:

```bash
cargo add toumei
```

### Detect tray transparency
You can detect the tray transparency mode by using the `detect_tray_transparency` function. This function returns a `Mode` value.
```rust
fn main() -> Result<(), toumei::Error> {
    match toumei::detect_tray_transparency()? {
        toumei::Mode::Transparent => println!("🎉 System tray transparency is enabled!"),
        toumei::Mode::Opaque => println!("🔒 System tray transparency is disabled"),
    }
    Ok(())
}
```

## C API Usage

Build the C library
```bash
cargo build --release --features capi
```

Call the functions from C/C++. The header file is in `include/toumei.h`.
```c
// Toumei has a C implementation in toumei.h
// This part is a traditional C style FFI wrapper for Toumei
#include "toumei.h"

ToumeiMode mode;
ToumeiResult result = detect_tray_transparency(&mode);

if (result == Ok) {
    if (mode == Transparent) { printf("🎉 System tray transparency is enabled!\n"); }
    if (mode == Opaque)      { printf("🔒 System tray transparency is disabled\n"); }
} else {
    // Handle error case
    printf("Error: ");
    // ... error handling code ...
}
```

Compile:
```bash
gcc your_program.c -ltoumei -Ltarget/release -Iinclude
```

## Examples
Check the `examples` folder for a `tray_check.rs` `rust` and `tray_check.c` `C` example codes.

## Author

**kuvaus**

## License

This project is licensed under the MIT [License](https://github.com/kuvaus/toumei/blob/main/LICENSE)
