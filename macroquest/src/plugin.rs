//! The MacroQuest Plugin API and related types and functions.
//!
//! The plugin API exposed by MacroQuest is a series of module level functions
//! with specific, well known names and type signatures that it will call into
//! whenever certain events occur. Plugins may implement any number of these
//! functions to implement their functionality.
//!
//! The following hooks are supported:
//!
//!   - ``PluginMain``: Called when the DLL is either loaded or unloaded from
//!     memory (see [`main`] for more information).
//!
//! # Examples
//!
//! Use the low level API to create a basic, but useless, plugin.
//!
//! ```
//! # use std::sync::OnceLock;
//! # use macroquest::log::trace;
//! # use macroquest::plugin::Reason;
//! static DATA: OnceLock<String> = OnceLock::new();
//!
//! #[macroquest::plugin::main]
//! fn pmain(reason: Reason) {
//!     match reason {
//!         Reason::Load => {
//!             trace!("module loaded");
//!
//!             DATA.set(String::new());
//!         }
//!         Reason::Unload => trace!("module unloaded"),
//!     };
//! }
//! ```

use num_enum::TryFromPrimitive;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[doc(inline)]
pub use macroquest_proc_macros::plugin_main as main;

/// Describes the reason that the plugin ``main`` function is being called.
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Reason {
    /// The DLL is being loaded into memory
    Load = DLL_PROCESS_ATTACH,
    /// The DLL is being unloaded from memory
    Unload = DLL_PROCESS_DETACH,
}

/// Helper type that is used to allow the [`main`] macro to support multiple
/// return types. Functionally acts as an adapter from Any supported type into
/// [`std::primitive::bool`].
///
/// Each variant represents another possible return type that we support for the
/// decorated ``main`` function.
///
/// This type is technically an implementation detail, but needs to be exposed
/// as pub because the [`main`] macro will generate code that uses it within the
/// user's own crate.
#[doc(hidden)]
pub enum MainResult {
    Unit,
    Bool(bool),
}

/// Adapt a given [`MainResult`] into a bool for return to the OS when
/// Windows calls the ``DllMain`` function.
///
/// If this returns [`false`](std::primitive::bool) then the module will be
/// unloaded immediately.
impl From<MainResult> for bool {
    fn from(value: MainResult) -> Self {
        match value {
            MainResult::Unit => true,
            MainResult::Bool(b) => b,
        }
    }
}

impl From<()> for MainResult {
    #[allow(clippy::ignored_unit_patterns)]
    fn from(_: ()) -> Self {
        MainResult::Unit
    }
}

impl From<bool> for MainResult {
    fn from(value: bool) -> Self {
        MainResult::Bool(value)
    }
}
