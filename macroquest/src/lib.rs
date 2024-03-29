//!

#![warn(missing_docs)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg(target_os = "windows")]

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

        pub struct MQPlugin;
    }
}

pub mod eq;
pub mod log;
pub mod mq;
pub mod plugin;

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
#[doc(alias = "IsBuiltForNext")]
#[must_use]
pub const fn is_mq_next() -> bool {
    true
}

/// An EverQuest version (build date + time) with trailing null byte.
#[repr(transparent)]
pub struct EQVersion([u8; 21]);

impl EQVersion {
    /// Return the build date portion of the [`EQVersion`]
    ///
    /// # Panics
    ///
    /// Panics if the [`EQVersion`] is not valid utf8.
    #[must_use]
    pub fn build_date(&self) -> &str {
        std::str::from_utf8(&self.0[0..11]).unwrap()
    }

    /// Return the build time portion of the [`EQVersion`]
    ///
    /// # Panics
    ///
    /// Panics if the [`EQVersion`] is not valid utf8.
    #[must_use]
    pub fn build_time(&self) -> &str {
        std::str::from_utf8(&self.0[12..20]).unwrap()
    }
}

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
#[doc(alias = "EverQuestVersion")]
#[must_use]
pub const fn eq_version() -> EQVersion {
    EQVersion(*ffi::EQ_VERSION)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eq_version_build_date() {
        assert_eq!(
            EQVersion(*b"Jan 02 2006 15:04:05\0").build_date(),
            "Jan 02 2006"
        );
    }

    #[test]
    fn test_eq_version_build_time() {
        assert_eq!(
            EQVersion(*b"Jan 02 2006 15:04:05\0").build_time(),
            "15:04:05"
        );
    }
}
