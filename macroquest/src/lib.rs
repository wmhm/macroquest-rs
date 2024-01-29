pub use macroquest_macros::plugin;
pub use pluginapi::{Plugin, PluginHandler};

mod pluginapi;
pub mod types;
#[doc(hidden)]
pub mod windows;
