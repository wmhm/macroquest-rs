#![cfg(target_os = "windows")]

pub use macroquest_macros::plugin;

#[cfg(not(docsrs))]
#[doc(hidden)]
pub use macroquest_sys as ffi;

#[cfg(docsrs)]
#[doc(hidden)]
// I hate that we have to maintain this in order to get doc building to work
// correctly on docs.rs, but such is life.
pub mod ffi {
    pub mod eqlib {
        pub struct PlayerClient;
        pub struct EQGroundItem;
    }

    pub mod mq {
        pub fn get_path_MQRoot() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_Config() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_MQini() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_Macros() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_Logs() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_CrashDumps() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_Plugins() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_Resources() -> &'static str {
            unimplemented!()
        }

        pub fn get_path_EverQuest() -> &'static str {
            unimplemented!()
        }

        pub fn write_chat_color(line: &str, color: i32) {
            unimplemented!()
        }
    }
}

pub use crate::pluginapi::{Plugin, PluginHandler};

pub mod eq;
pub mod log;
pub mod mq;

mod pluginapi;

#[doc(hidden)]
pub mod windows;

mod macros {
    #[macro_export]
    macro_rules! println {
        () => {};

        ($($arg:tt)*) => {
            ::macroquest::mq::write_chat_color(
                format!($($arg)*),
                ::macroquest::eq::ChatColor::ChatChannel,
            );
        };
    }
}
