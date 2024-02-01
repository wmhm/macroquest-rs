#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;

mod plugin;

/// Defines the ``PluginMain`` entry point for this plugin.
///
/// The ``PluginMain`` entry point is the first thing and last thing that will
/// be called (and in fact, is called by Windows, not MacroQuest) when the DLL
/// for this plugin is loaded and unloaded. It can be used to do any very basic
/// setup (such as creating the underlying plugin object, or creating static
/// data structures, etc) that needs to happen prior to any of the MacroQuest
/// functions being called.
///
/// The wrapped function must take a single parameter, a `Reason`, and can
/// return one of:
///
///   - `()`
///   - [`std::primitive::bool`]
///
/// If a [`false`](std::primitive::bool)-ey value is returned, then the DLL will
/// be immediately unloaded.
///
/// # Examples
///
/// A simple ``main`` function that can never fail and will always load the
/// DLL.
///
/// ```
/// # use macroquest::{log::trace, plugin::Reason};
/// # use macroquest_proc_macros::plugin_main as main;
/// #[main]
/// fn pmain(reason: Reason) {
///     match reason {
///         Reason::Load => trace!("module loaded"),
///         Reason::Unload => trace!("module unload"),
///     }
/// }
///
/// ```
///
/// A slightly more complex ``main`` function that could return
/// [`false`](std::primitive::bool) if it wasn't able to initialize the module
/// fully.
///
/// ```
/// # use macroquest::{log::trace, plugin::Reason};
/// # use macroquest_proc_macros::plugin_main as main;
/// # fn check_if_can_allocate() -> bool { false }
/// #[main]
/// fn pmain(reason: Reason) -> bool {
///     if let Reason::Load = reason {
///         trace!("module loaded");
///
///         // If this fails, we need to just unload the module
///         if !check_if_can_allocate() {
///             return false;
///         }
///     }
///
///     true
/// }
/// ```
///
#[doc(alias = "PluginMain")]
#[doc(alias = "DllMain")]
#[proc_macro_attribute]
#[proc_macro_error]
pub fn plugin_main(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let main = syn::parse_macro_input!(item as plugin::dllmain::PluginMain);

    quote! { #main }.into()
}
