#![cfg(target_os = "windows")]

pub use macroquest_macros::plugin;
#[doc(hidden)]
pub use macroquest_sys as ffi;

pub use crate::pluginapi::{Plugin, PluginHandler};

pub mod eq;
mod pluginapi;
#[doc(hidden)]
pub mod windows;
