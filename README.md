
<div align="center">
    <img src="resources/logo.png" width="200"/>
    <h1>toumei</h1>
    <p>透明 【とうめい】tōmei, meaning: transparent; clear.</p>
    <p>Rust crate to detect if system tray is transparent or opaque</p>
    <a href="https://crates.io/crates/toumei"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/toumei?style=for-the-badge"></a>
    <br>
    <br>
</div>

Supports macOS, Windows, and Linux.

  * On **macOS** it detects transparency of the **System Tray (Top Bar)** is transparent or opaque. It checks the transparency by seeing if the Reduce Transparency setting in Universal Access is turned on or off.
  * On **Windows** it detects the transparency of the **Windows Taskbar**. It first checks if TranslucentTB is changing the Taskbar transparency. It also checks if either the Layered window attributes, DWM color attribute, or DWM backdrop type (Windows 11) is changing the Taskbar transparency.
  * On **Linux** it detects the transparency of the **Top Bar**. It checks if XDG Desktop Portal has set the Transparency. On GNOME it also checks if gnome-extension transparent-top-bar is changing the Top Bar transparency.


[API Documentation](https://docs.rs/toumei/)

## Usage

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

## License

This project is licensed under the MIT [License](https://github.com/kuvaus/toumei/blob/main/LICENSE)

