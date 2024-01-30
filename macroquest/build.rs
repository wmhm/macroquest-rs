fn main() {
    let mq_config = macroquest_build_config::BuildConfig::load();

    cxx_build::bridge("src/ffi/mod.rs")
        .std("c++17")
        .includes(mq_config.include_dirs())
        .define("NOMINMAX", None)
        .file("src/ffi/eqlib.cc")
        .file("src/ffi/mq.cc")
        .compile("mqrust");

    mq_config.emit();

    // Emit for all of the headers/files in eqlib
    for entry in walkdir::WalkDir::new(mq_config.eqlib_dir())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let filename = entry.file_name().to_string_lossy();
            if filename.ends_with(".h") || filename.ends_with(".cc") {
                println!("cargo:rerun-if-changed={}", entry.path().display())
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/eqlib.h");
    println!("cargo:rerun-if-changed=include/mq.h");
    println!("cargo:rerun-if-changed=src/ffi/mod.rs");
    println!("cargo:rerun-if-changed=src/ffi/eqlib.cc");
    println!("cargo:rerun-if-changed=src/ffi/mq.cc");
}
