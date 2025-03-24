// build.rs
fn main() {
    // Only link Foundation framework on macOS
    if std::env::var("TARGET").unwrap().contains("apple") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=objc");
    }
}
