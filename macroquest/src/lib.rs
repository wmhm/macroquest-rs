pub use macroquest_macros::plugin;

pub use crate::pluginapi::{Plugin, PluginHandler};
pub use crate::types::{ChatColor, GameState};

mod pluginapi;
pub mod types;
#[doc(hidden)]
pub mod windows;
