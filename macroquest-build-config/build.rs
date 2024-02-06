use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::{env, fs};

// NOTE: this has to be kept in sync with the BuildConfig located in srcs/lib.rs
#[derive(Debug)]
struct BuildConfig {
    eq_version: String,
    mq_dir:     PathBuf,
    mq_profile: String,
    mq_arch:    String,
}

impl BuildConfig {
    fn serialize(&self) -> String {
        [
            self.eq_version.as_str(),
            self.mq_dir.to_str().expect("invalid path; not valid utf8"),
            self.mq_profile.as_str(),
            self.mq_arch.as_str(),
        ]
        .join("\n")
    }
}

fn eq_version(dir: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let current_path = env::var_os("PATH").unwrap();
    let mut new_path = current_path.clone();
    new_path.push(";");
    new_path.push(dir);
    env::set_var("PATH", new_path);

    Ok(unsafe {
        let lib = libloading::Library::new(dir.join("MQ2Main.dll"))?;

        let version_ptr: libloading::Symbol<*const c_char> =
            lib.get(b"gszVersion\0")?;
        let version = CStr::from_ptr(*version_ptr).to_str()?;

        let time_ptr: libloading::Symbol<*const c_char> = lib.get(b"gszTime\0")?;
        let time = CStr::from_ptr(*time_ptr).to_str()?;

        format!("{} {}", version, time)
    })
}

fn main() {
    // We need to rerun if a number of things change, so mark them all.
    println!("cargo:rerun-if-env-changed=MACROQUEST_DIR");
    println!("cargo:rerun-if-env-changed=MACROQUEST_PROFILE");
    println!("cargo:rerun-if-env-changed=MACROQUEST_ARCH");
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    let config = if std::env::var("DOCS_RS").is_ok() {
        // If we're building on docs.rs then we synthesize a build configuration
        BuildConfig {
            eq_version: String::from("docs build"),
            mq_dir:     PathBuf::from("docs build"),
            mq_profile: String::from("docs build"),
            mq_arch:    String::from("docs build"),
        }
    }
    else if target_os != "windows" {
        // If we're building for a non windows platform, then we synthesize a
        // build configuration
        BuildConfig {
            eq_version: String::from("non windows build"),
            mq_dir:     PathBuf::from("non windows build"),
            mq_profile: String::from("non windows build"),
            mq_arch:    String::from("non windows build"),
        }
    }
    else {
        // Compute our Build Configuration
        let mq_dir = PathBuf::from(
            env::var_os("MACROQUEST_DIR")
                .expect("Must set MACROQUEST_DIR to the root of a MacroQuest checkout"),
        );
        let mq_profile =
            env::var("MACROQUEST_PROFILE").unwrap_or_else(|_| "release".into());
        let mq_arch = env::var("MACROQUEST_ARCH").unwrap_or_else(|_| "x64".into());

        // Determine what version of EverQuest we're building against
        let eq_version =
            eq_version(mq_dir.join("build/bin").join(mq_profile.as_str()).as_path())
                .unwrap();

        BuildConfig {
            eq_version,
            mq_dir,
            mq_profile,
            mq_arch,
        }
    };

    // Actually write out our configuration file so that our crate can read it
    // at compile time.
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("config.txt");
    fs::write(dest_path, config.serialize()).unwrap();
}
