use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::{env, fs};

struct MQConfig {
    dir:     PathBuf,
    profile: String,
    arch:    String,
}

impl MQConfig {
    fn from_env() -> Self {
        let dir = PathBuf::from(
            env::var_os("MACROQUEST_DIR")
                .expect("Must set MACROQUEST_DIR to the root of a MacroQuest checkout"),
        );
        let profile =
            env::var("MACROQUEST_PROFILE").unwrap_or_else(|_| "release".into());
        let arch = env::var("MACROQUEST_ARCH").unwrap_or_else(|_| "x64".into());

        MQConfig { dir, profile, arch }
    }

    #[must_use]
    fn bin_dir(&self) -> PathBuf {
        self.dir.join("build/bin").join(self.profile.as_str())
    }

    #[must_use]
    fn lib_dirs(&self) -> Vec<PathBuf> {
        vec![
            // $MACROQUEST/build/bin/$PROFILE/
            self.dir.join("build/bin").join(&self.profile),
            // $MACROQUEST/build/lib/$ARCH/$PROFILE
            self.dir
                .join("build/lib")
                .join(&self.arch)
                .join(&self.profile),
            // $MACROQUEST/contrib/vcpkg/installed/$ARCH-windows-static/lib
            self.dir
                .join("contrib/vcpkg/installed")
                .join(format!("{}-windows-static", self.arch))
                .join("lib"),
            // $MACROQUEST/contrib/vcpkg/installed/$ARCH-windows/lib
            self.dir
                .join("contrib/vcpkg/installed")
                .join(format!("{}-windows", self.arch))
                .join("lib"),
        ]
    }

    #[must_use]
    fn include_dirs(&self) -> Vec<PathBuf> {
        [
            "include",
            "src",
            "contrib",
            r"contrib\imgui",
            r"contrib\vcpkg\installed\x64-windows-static\include",
            r"contrib\vcpkg\installed\x64-windows\include",
        ]
        .iter()
        .map(|s| self.dir.join(s))
        .collect()
    }
}

// This is a bit of a hack, ideally we'd just use our bindings to access this
// information, however that doesn't seem to be working due to a number of
// different interacting issues. So this small hack will let us work around
// that, and should overall be pretty harmless.
fn eq_version<P>(dir: P) -> String
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();

    let current_path = env::var_os("PATH").unwrap();
    let mut new_path = current_path.clone();
    new_path.push(";");
    new_path.push(dir);
    env::set_var("PATH", new_path);

    unsafe {
        let lib = libloading::Library::new(dir.join("MQ2Main.dll")).unwrap();

        let version_ptr: libloading::Symbol<*const c_char> =
            lib.get(b"gszVersion\0").unwrap();
        let version = CStr::from_ptr(*version_ptr).to_str().unwrap();

        let time_ptr: libloading::Symbol<*const c_char> =
            lib.get(b"gszTime\0").unwrap();
        let time = CStr::from_ptr(*time_ptr).to_str().unwrap();

        format!("{} {}", version, time)
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=MACROQUEST_DIR");
    println!("cargo:rerun-if-env-changed=MACROQUEST_PROFILE");
    println!("cargo:rerun-if-env-changed=MACROQUEST_ARCH");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/eqlib.h");
    println!("cargo:rerun-if-changed=src/eqlib.cc");
    println!("cargo:rerun-if-changed=include/mq.h");
    println!("cargo:rerun-if-changed=src/mq.cc");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // We can only be built for windows
    if target_os == "windows" {
        let config = MQConfig::from_env();

        // Emit the directories to search for linkable libraries
        for libdir in config.lib_dirs() {
            println!("cargo:rustc-link-search={}", libdir.to_string_lossy());
        }

        // Emit the libraries we actually need to link against
        println!("cargo:rustc-link-lib=MQ2Main");
        println!("cargo:rustc-link-lib=eqlib");
        println!("cargo:rustc-link-lib=pluginapi");

        // Build our bridge between C++ and Rust
        cxx_build::bridge("src/lib.rs")
            .std("c++17")
            .includes(config.include_dirs())
            .define("NOMINMAX", None)
            .files(["src/eqlib.cc", "src/mq.cc"])
            .compile("mqrust");

        // Write out the EQVersion string
        let out_dir = env::var_os("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("eq_version.rs");
        fs::write(
            dest_path,
            format!("b\"{}\\0\"", eq_version(config.bin_dir())),
        )
        .unwrap();
    }
}
