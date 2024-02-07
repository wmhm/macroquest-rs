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
