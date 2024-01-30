fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/eqlib.h");
    println!("cargo:rerun-if-changed=include/mq.h");
    println!("cargo:rerun-if-changed=src/ffi/mod.rs");
    println!("cargo:rerun-if-changed=src/ffi/eqlib.cc");

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    // We can only be built for windows
    if target_os == "windows" {
        macroquest_build_config::BuildConfig::load().emit();
    }
}
