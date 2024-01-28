fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    macroquest_build_config::BuildConfig::load().emit();
}
