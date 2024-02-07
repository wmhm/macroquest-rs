#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream;
use proc_macro_error::{abort_call_site, proc_macro_error};
use quote::quote;

mod plugin;

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

/// Defines the plugin hooks for an `impl Hooks` block.
///
/// Whenever implementing a `macroquest::plugin::Hooks` trait, decorating it
/// with the [`hooks`](`macro@plugin_hooks`) macro will cause all of the
/// implemented methods to emit the macroquest hook functions.
///
/// # Examples
///
/// Basic example of implementing a few `Hooks` methods.
/// ```
/// # use macroquest::eq;
/// # use macroquest::plugin::Hooks;
/// # use macroquest_proc_macros::plugin_hooks as hooks;
/// # use std::sync::OnceLock;
/// # static PLUGIN: OnceLock<MyPlugin> = OnceLock::new();
/// struct MyPlugin;
///
/// #[hooks]
/// impl Hooks for MyPlugin {
///     fn initialize(&self) {}
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn plugin_hooks(attr: TokenStream, item: TokenStream) -> TokenStream {
    if !attr.is_empty() {
        abort_call_site!("arguments are not supported")
    }

    let hooks = syn::parse_macro_input!(item as plugin::hooks::Hooks);

    quote! { #hooks }.into()
}
