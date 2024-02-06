fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=include/eqlib.h");
    println!("cargo:rerun-if-changed=src/eqlib.cc");
    println!("cargo:rerun-if-changed=include/mq.h");
    println!("cargo:rerun-if-changed=src/mq.cc");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // We can only be built for windows
    if target_os == "windows" {
        let config = macroquest_build_config::BuildConfig::load();

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
    }
}
