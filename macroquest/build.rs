use std::path::Path;
use std::{env, fs};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // We can only be built for windows
    if target_os == "windows" {
        let config = macroquest_build_config::BuildConfig::load();

        // Write out the EQVersion string
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("eq_version.rs");
        fs::write(dest_path, format!("b\"{}\\0\"", config.eq_version())).unwrap();
    }
}
