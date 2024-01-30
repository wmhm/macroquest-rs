#![cfg(target_os = "windows")]

#[cxx::bridge(namespace = "mqrust::eqlib")]
pub mod eqlib {
    unsafe extern "C++" {
        include!("macroquest-sys/include/eqlib.h");

        pub type PlayerClient;

        fn name(&self) -> &str;

    }

    unsafe extern "C++" {
        include!("macroquest-sys/include/eqlib.h");

        pub type EQGroundItem;

        fn name(&self) -> &str;
    }
}

#[cxx::bridge(namespace = "mqrust::mq")]
pub mod mq {
    unsafe extern "C++" {
        include!("macroquest-sys/include/mq.h");

        fn write_chat_color(line: &str, color: i32);
    }
}
