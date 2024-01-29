#[cxx::bridge(namespace = "mqrust::eqlib")]
pub mod eqlib {
    unsafe extern "C++" {
        include!("macroquest/include/eqlib.h");

        pub type EQGroundItem;

        fn name(&self) -> &str;
    }
}
