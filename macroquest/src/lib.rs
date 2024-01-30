#![cfg(target_os = "windows")]

pub use macroquest_macros::plugin;

pub use crate::pluginapi::{Plugin, PluginHandler};

pub mod eq;
#[doc(hidden)]
pub mod ffi;
mod pluginapi;
#[doc(hidden)]
pub mod windows;
