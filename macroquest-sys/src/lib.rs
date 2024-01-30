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

        // Path Functions
        fn get_path_MQRoot() -> &'static str;
        fn get_path_Config() -> &'static str;
        fn get_path_MQini() -> &'static str;
        fn get_path_Macros() -> &'static str;
        fn get_path_Logs() -> &'static str;
        fn get_path_CrashDumps() -> &'static str;
        fn get_path_Plugins() -> &'static str;
        fn get_path_Resources() -> &'static str;
        fn get_path_EverQuest() -> &'static str;

        // General Functions
        fn write_chat_color(line: &str, color: i32);
    }
}
