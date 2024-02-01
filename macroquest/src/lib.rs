//!

#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
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
pub mod plugin;

mod pluginapi;

#[doc(hidden)]
pub mod windows;

mod macros {
    #[allow(missing_docs)]
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

/// Detects whether we're currently running on "MQNext".
///
/// This function is always going to return [`true`](std::primitive::bool) as we
/// only support MQNext (which is now generally known as MacroQuest).
///
/// This is most useful for future proofing the ``IsBuiltForNext`` symbol that
/// plugins need to export.
#[must_use]
pub const fn is_mq_next() -> bool {
    true
}

/// An EverQuest version (build date + time) with trailing null byte.
pub type EQVersion = [u8; 21];

/// The version of EverQuest that this crate is built against.
///
/// This returns a byte string that of the format ``Jan 02 2006 15:04:05``
/// followed by a null byte. This is the date and time that the ``eqgame.exe``
/// binary was built, which MacroQuest (and thus us) use as a stand in for a
/// version number for EverQuest itself.
///
/// This is most useful for the ``EverQuestVersion`` symbol that plugins need to
/// export to tell MacroQuest if they were compiled for a different version of
/// EverQuest.
#[must_use]
pub const fn eq_version() -> EQVersion {
    *include!(concat!(env!("OUT_DIR"), "/eq_version.rs"))
}
