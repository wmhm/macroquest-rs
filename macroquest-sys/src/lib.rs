#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
#![cfg(target_os = "windows")]

pub const EQ_VERSION: &[u8; 21] = include!(concat!(env!("OUT_DIR"), "/eq_version.rs"));

#[cxx::bridge(namespace = "mqrust::eqlib")]
pub mod eqlib {
    unsafe extern "C++" {
        include!("macroquest-sys/include/eqlib.h");

        pub type PlayerClient;

        #[must_use]
        fn name(&self) -> &str;

    }

    unsafe extern "C++" {
        include!("macroquest-sys/include/eqlib.h");

        pub type EQGroundItem;

        #[must_use]
        fn name(&self) -> &str;
    }
}

#[cxx::bridge(namespace = "mqrust::mq")]
pub mod mq {
    unsafe extern "C++" {
        include!("macroquest-sys/include/mq.h");

        // Path Functions
        #[must_use]
        fn get_path_MQRoot() -> &'static str;

        #[must_use]
        fn get_path_Config() -> &'static str;

        #[must_use]
        fn get_path_MQini() -> &'static str;

        #[must_use]
        fn get_path_Macros() -> &'static str;

        #[must_use]
        fn get_path_Logs() -> &'static str;

        #[must_use]
        fn get_path_CrashDumps() -> &'static str;

        #[must_use]
        fn get_path_Plugins() -> &'static str;

        #[must_use]
        fn get_path_Resources() -> &'static str;

        #[must_use]
        fn get_path_EverQuest() -> &'static str;

        // General Functions
        fn write_chat_color(line: &str, color: i32);

        // MQPlugin
        pub type MQPlugin;

        #[must_use]
        fn plugin_name(&self) -> &str;
    }
}

unsafe impl Send for mq::MQPlugin {}
unsafe impl Sync for mq::MQPlugin {}
