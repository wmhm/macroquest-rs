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

/// Emit the symbols that mark this crate as a MacroQuest plugin.
///
/// Every MacroQuest plugin requires two symbols:
///
///   - A ``IsBuiltForNext`` ([`std::primitive::bool`]) symbol to indicate that
///     this plugin has been built for "MQNext" (aka MacroQuest) rather than the
///     now defunct "MQ2" (aka MacroQuest2).
///   - A ``EverQuestVersion`` (`const c_char[]`) symbol to indicate what
///     version of EverQuest this plugin has been built against.
///
/// This macro emits the ``pub static`` variables for these symbols, with the
/// correct values, to allow this crate to be loaded as a MacroQuest plugin.

// NOTE: It's kind of silly to have this as a proc macro when a regular macro
//       would work just as fine. However, we can't export a regular macro from
//       a proc macro crate, and we need a crate to hold the macro so it doesn't
//       end up at the root of the crate.
#[proc_macro]
pub fn plugin_preamble(_item: TokenStream) -> TokenStream {
    quote! {
        #[no_mangle]
        pub static IsBuiltForNext: bool = ::macroquest::is_mq_next();

        #[no_mangle]
        pub static EverQuestVersion: ::macroquest::EQVersion = ::macroquest::eq_version();
    }
    .into()
}

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

/// Defines an implementation of a MacroQuest plugin hook.
///
/// MacroQuest plugins work by implementing functions with particular names and
/// function signatures. To implement this, we expose the
/// [`hook`](`macro@plugin_hook`) macro which decorates a function and exposes
/// it using the proper name. Each hook has a specific function signature that
/// it expects.
///
/// The general rule for function signature is that the C/C++ signature that
/// MacroQuest expects has been translated into the most logical primitive type
/// supported by rust where possible, or a custom type from the
/// ``macroquest::eq`` crate otherwise.
///
/// # Examples
///
/// Register a hook for the ``InitializePlugin`` hook.
///
/// ```
/// # use macroquest::log::trace;
/// # use macroquest_proc_macros::plugin_hook as hook;
/// // The hook name does not need imported or defined.
/// #[hook(InitializePlugin)]
/// fn my_init_hook() {
///     trace!("plugin initializing")
/// }
/// ```
///
/// Register a hook for the ``SetGameState`` hook.
///
/// ```
/// # use macroquest::log::trace;
/// # use macroquest_proc_macros::plugin_hook as hook;
/// #[hook(SetGameState)]
/// fn my_set_game_state_hook(state: macroquest::eq::GameState) {
///     trace!("game state updated")
/// }
/// ```
///
/// Register a hook for the ``OnWriteChatColor`` hook.
///
/// ```
/// # use macroquest::log::trace;
/// # use macroquest_proc_macros::plugin_hook as hook;
/// #[hook(OnWriteChatColor)]
/// fn my_write_chat_color_hook(line: &str, color: macroquest::eq::ChatColor) {
///     trace!(?line, ?color, "WriteChatColor called")
/// }
/// ```
///
/// Register a hook for the ``OnAddSpawn`` hook.
///
/// ```
/// # use macroquest::log::trace;
/// # use macroquest_proc_macros::plugin_hook as hook;
/// #[hook(OnAddSpawn)]
/// fn my_add_spawn_hook(spawn: &macroquest::eq::Spawn) {
///     trace!(?spawn, "new spawn")
/// }
/// ```
///
/// Register a hook for the ``OnAddGroundItem`` hook.
///
/// ```
/// # use macroquest::log::trace;
/// # use macroquest_proc_macros::plugin_hook as hook;
/// #[hook(OnAddGroundItem)]
/// fn my_add_ground_item_hook(item: &macroquest::eq::GroundItem) {
///     trace!(?item, "new ground spawn")
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn plugin_hook(attr: TokenStream, item: TokenStream) -> TokenStream {
    match plugin::hook::Hook::parse(attr, item) {
        Ok(hook) => quote! { #hook }.into(),
        Err(e) => e.into_compile_error().into(),
    }
}
