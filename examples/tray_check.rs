fn main() -> Result<(), toumei::Error> {
    match toumei::detect_tray_transparency()? {
        toumei::Mode::Transparent => println!("🎉 System tray transparency is enabled!"),
        toumei::Mode::Opaque => println!("🔒 System tray transparency is disabled"),
    }
    Ok(())
}
