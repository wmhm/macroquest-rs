use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

// NOTE: this has to be kept in sync with the BuildConfig located in srcs/lib.rs
#[derive(Debug, Serialize)]
struct BuildConfig {
    profile: String,
    root_dir: PathBuf,
    bin_dir: Option<PathBuf>,
}

fn main() {
    // We need to rerun if a number of things change, so mark them all.
    println!("cargo:rerun-if-env-changed=MACROQUEST_BUILD_PROFILE");
    println!("cargo:rerun-if-env-changed=MACROQUEST_DIR");
    println!("cargo:rerun-if-env-changed=MACROQUEST_BUILD_BIN_DIR");
    println!("cargo:rerun-if-changed=build.rs");

    // Compute our Build Configuration
    let mq_profile = env::var("MACROQUEST_BUILD_PROFILE").unwrap_or_else(|_| "release".into());
    let mq_root_dir = PathBuf::from(
        env::var_os("MACROQUEST_DIR")
            .expect("Must set MACROQUEST_DIR to the root of a MacroQuest checkout"),
    );
    let mq_bin_dir = env::var_os("MACROQUEST_BUILD_BIN_DIR").map(PathBuf::from);

    let config = BuildConfig {
        root_dir: mq_root_dir,
        profile: mq_profile,
        bin_dir: mq_bin_dir,
    };

    // Actually write out our configuration file so that our crate can read it
    // at compile time.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.toml");
    fs::write(dest_path, toml::to_string(&config).unwrap()).unwrap();
}
