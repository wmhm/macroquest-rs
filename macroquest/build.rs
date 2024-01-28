fn main() {
    macroquest_build_config::BuildConfig::load().emit();

    println!("cargo:rerun-if-changed=build.rs");
}
