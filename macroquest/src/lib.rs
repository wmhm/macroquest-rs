#![cfg(target_os = "windows")]

pub use macroquest_macros::plugin;

#[doc(hidden)]
#[cfg(not(docrs))]
pub use macroquest_sys as ffi;

#[doc(hidden)]
#[cfg(docrs)]
pub mod ffi {
    pub mod eqlib {
        pub struct PlayerClient;
        pub struct EQGroundItem;
    }
}

pub use crate::pluginapi::{Plugin, PluginHandler};

pub mod eq;
mod pluginapi;
#[doc(hidden)]
pub mod windows;
