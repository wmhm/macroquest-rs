use std::path::PathBuf;

use serde::Deserialize;

// NOTE: this has to be kept in sync with the BuildConfig located in build.rs
#[derive(Deserialize, Debug)]
pub struct BuildConfig {
    eq_version: String,
    profile: String,
    root_dir: PathBuf,
    bin_dir: PathBuf,
}

impl BuildConfig {
    pub fn load() -> BuildConfig {
        let config_str = include_str!(concat!(env!("OUT_DIR"), "/config.toml"));
        let config: BuildConfig = toml::from_str(config_str).unwrap();
        config
    }

    pub fn emit(&self) {
        println!(
            "cargo:rustc-link-search={}",
            self.bin_dir.join(self.profile.as_str()).display()
        );
        println!(
            "cargo:rustc-link-search={}",
            self.root_dir
                .join("build/lib/x64")
                .join(self.profile.as_str())
                .display()
        );
        println!(
            "cargo:rustc-link-search={}",
            self.root_dir
                .join(r"contrib\vcpkg\installed\x64-windows-static\lib")
                .display()
        );
        println!(
            "cargo:rustc-link-search={}",
            self.root_dir
                .join(r"contrib\vcpkg\installed\x64-windows\lib")
                .display()
        );

        println!("cargo:rustc-link-lib=MQ2Main");
        println!("cargo:rustc-link-lib=eqlib");
        println!("cargo:rustc-link-lib=pluginapi");
    }
}

impl BuildConfig {
    pub fn include_dirs(&self) -> Vec<PathBuf> {
        [
            "include",
            "src",
            "contrib",
            r"contrib\imgui",
            r"contrib\vcpkg\installed\x64-windows-static\include",
            r"contrib\vcpkg\installed\x64-windows\include",
        ]
        .iter()
        .map(|s| self.root_dir.join(s))
        .collect()
    }

    pub fn eqlib_dir(&self) -> PathBuf {
        self.root_dir.join(r"src\eqlib")
    }

    pub fn eq_version(&self) -> &str {
        self.eq_version.as_str()
    }
}
