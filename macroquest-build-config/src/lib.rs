#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

// NOTE: this has to be kept in sync with the BuildConfig located in build.rs
#[derive(Debug)]
pub struct BuildConfig {
    eq_version: String,
    mq_dir:     PathBuf,
    mq_profile: String,
    mq_arch:    String,
}

impl BuildConfig {
    #[must_use]
    #[allow(clippy::missing_panics_doc)]
    pub fn load() -> BuildConfig {
        let config_str = include_str!(concat!(env!("OUT_DIR"), "/config.txt"));
        let config_data: Vec<&str> = config_str.split('\n').collect();

        BuildConfig {
            eq_version: String::from(config_data[0]),
            mq_dir:     PathBuf::from(config_data[1]),
            mq_profile: String::from(config_data[2]),
            mq_arch:    String::from(config_data[3]),
        }
    }
}

impl BuildConfig {
    #[must_use]
    pub fn eq_version(&self) -> &str {
        self.eq_version.as_str()
    }

    #[must_use]
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
        .map(|s| self.mq_dir.join(s))
        .collect()
    }

    #[must_use]
    pub fn lib_dirs(&self) -> Vec<PathBuf> {
        vec![
            // $MACROQUEST/build/bin/$PROFILE/
            self.mq_dir.join("build/bin").join(&self.mq_profile),
            // $MACROQUEST/build/lib/$ARCH/$PROFILE
            self.mq_dir
                .join("build/lib")
                .join(&self.mq_arch)
                .join(&self.mq_profile),
            // $MACROQUEST/contrib/vcpkg/installed/$ARCH-windows-static/lib
            self.mq_dir
                .join("contrib/vcpkg/installed")
                .join(format!("{}-windows-static", self.mq_arch))
                .join("lib"),
            // $MACROQUEST/contrib/vcpkg/installed/$ARCH-windows/lib
            self.mq_dir
                .join("contrib/vcpkg/installed")
                .join(format!("{}-windows", self.mq_arch))
                .join("lib"),
        ]
    }
}
