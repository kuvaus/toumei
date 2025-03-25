#[cfg(feature = "capi")]
extern crate cbindgen;
#[cfg(feature = "capi")]
use std::env;
#[cfg(feature = "capi")]
use std::path::PathBuf;

// build.rs
fn main() {
    // Only link Foundation framework on macOS
    if std::env::var("TARGET").unwrap().contains("apple") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=objc");
    }
    // These are for building the optional c bindings
    #[cfg(feature = "capi")]
    {
        let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let package_name = env::var("CARGO_PKG_NAME").unwrap();
        let output_file = PathBuf::from(&crate_dir)
            .join("include")
            .join(format!("{}.h", package_name))
            .display()
            .to_string();
    
        let config = cbindgen::Config {
            language: cbindgen::Language::C,
            include_guard: Some("TOUMEI_H_".into()),
            ..Default::default()
        };
    
        cbindgen::generate_with_config(&crate_dir, config)
            .unwrap()
            .write_to_file(&output_file);
    }
}