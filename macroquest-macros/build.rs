use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let config = macroquest_build_config::BuildConfig::load();

    // Write out the EQVersion string
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("eq_version.txt");
    fs::write(dest_path, config.eq_version()).unwrap();
}
