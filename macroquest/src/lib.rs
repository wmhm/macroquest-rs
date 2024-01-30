#![cfg(target_os = "windows")]

pub use macroquest_macros::plugin;

#[cfg(not(docsrs))]
#[doc(hidden)]
pub use macroquest_sys as ffi;

#[cfg(docsrs)]
#[doc(hidden)]
pub mod ffi {
    pub mod eqlib {
        pub struct PlayerClient;
        pub struct EQGroundItem;
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
