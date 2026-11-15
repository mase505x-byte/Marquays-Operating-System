use std::env;
use std::fs;
use std::path::Path;

fn main() {
	let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set by Cargo");
	let dest_path = Path::new(&out_dir).join("built.rs");
	
	let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
	let profile = env::var("PROFILE").unwrap_or_else(|_| "unknown".to_string());
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());

    let generated = format!(
        r#"
        pub const PKG_VERSION: &str = "{version}";
        pub const TARGET_TRIPLE: &str = "{target}";
        pub const BUILD_PROFILE: &str = "{profile}";
        "#
    );

    fs::write(&dest_path, generated).expect("Failed to write built.rs");
    println!("cargo:rerun-if-changed=build.rs");
}