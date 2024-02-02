use std::str::FromStr;

use proc_macro_error::abort;
use quote::{format_ident, quote, ToTokens};
use strum::EnumString;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ImplItemFn, ItemImpl};

#[derive(Debug, PartialEq, EnumString, strum::Display)]
enum Kind {
    #[strum(serialize = "initialize", to_string = "InitializePlugin")]
    InitializePlugin,
    #[strum(serialize = "shutdown", to_string = "ShutdownPlugin")]
    ShutdownPlugin,
    #[strum(serialize = "on_clean_ui", to_string = "OnCleanUI")]
    OnCleanUI,
    #[strum(serialize = "on_reload_ui", to_string = "OnReloadUI")]
    OnReloadUI,
    #[strum(serialize = "on_draw_hud", to_string = "OnDrawHUD")]
    OnDrawHUD,
    #[strum(serialize = "on_set_game_state", to_string = "SetGameState")]
    SetGameState,
    #[strum(serialize = "on_pulse", to_string = "OnPulse")]
    OnPulse,
    #[strum(serialize = "on_write_chat_color", to_string = "OnWriteChatColor")]
    OnWriteChatColor,
    #[strum(serialize = "on_incoming_chat", to_string = "OnIncomingChat")]
    OnIncomingChat,
    #[strum(serialize = "on_add_spawn", to_string = "OnAddSpawn")]
    OnAddSpawn,
    #[strum(serialize = "on_remove_spawn", to_string = "OnRemoveSpawn")]
    OnRemoveSpawn,
    #[strum(serialize = "on_add_ground_item", to_string = "OnAddGroundItem")]
    OnAddGroundItem,
    #[strum(serialize = "on_remove_ground_item", to_string = "OnRemoveGroundItem")]
    OnRemoveGroundItem,
    #[strum(serialize = "on_begin_zone", to_string = "OnBeginZone")]
    OnBeginZone,
    #[strum(serialize = "on_end_zone", to_string = "OnEndZone")]
    OnEndZone,
    #[strum(serialize = "on_zoned", to_string = "OnZoned")]
    OnZoned,
    #[strum(serialize = "on_update_imgui", to_string = "OnUpdateImGui")]
    OnUpdateImGui,
    #[strum(serialize = "on_macro_start", to_string = "OnMacroStart")]
    OnMacroStart,
    #[strum(serialize = "on_macro_stop", to_string = "OnMacroStop")]
    OnMacroStop,
    #[strum(serialize = "on_plugin_load", to_string = "OnLoadPlugin")]
    OnLoadPlugin,
    #[strum(serialize = "on_plugin_unload", to_string = "OnUnloadPlugin")]
    OnUnloadPlugin,
}

pub(crate) struct Hooks {
    body: ItemImpl,
    implemented: Vec<ImplItemFn>,
}

impl Parse for Hooks {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let plugin_impl: ItemImpl = input.parse()?;
        let mut hooks = Hooks {
            body: plugin_impl.clone(),
            implemented: vec![],
        };

        hooks.fold_item_impl(plugin_impl);

        Ok(hooks)
    }
}

impl Fold for Hooks {
    fn fold_impl_item_fn(&mut self, method: ImplItemFn) -> ImplItemFn {
        self.implemented.push(method.clone());
        method
    }
}

impl ToTokens for Hooks {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.body.to_tokens(tokens);

        for hook in &self.implemented {
            let plugin_hook_name = &hook.sig.ident;
            let Ok(kind) = Kind::from_str(hook.sig.ident.to_string().as_str()) else {
                abort!(hook, "The hook must be a supported MacroQuest hook");
            };

            match kind {
                kind @ (Kind::InitializePlugin
                | Kind::ShutdownPlugin
                | Kind::OnCleanUI
                | Kind::OnDrawHUD
                | Kind::OnReloadUI
                | Kind::OnPulse
                | Kind::OnBeginZone
                | Kind::OnEndZone
                | Kind::OnZoned
                | Kind::OnUpdateImGui) => Hooks::to_tokens_simple_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ Kind::SetGameState => Hooks::to_tokens_gamestate_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ Kind::OnWriteChatColor => Hooks::to_tokens_writechatcolor_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ Kind::OnIncomingChat => Hooks::to_tokens_incomingchat_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ (Kind::OnMacroStart
                | Kind::OnMacroStop
                | Kind::OnLoadPlugin
                | Kind::OnUnloadPlugin) => Hooks::to_tokens_str_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ (Kind::OnAddSpawn | Kind::OnRemoveSpawn) => Hooks::to_tokens_spawn_hook(
                    plugin_hook_name,
                    &format_ident!("{}", kind.to_string()),
                    tokens,
                ),
                kind @ (Kind::OnAddGroundItem | Kind::OnRemoveGroundItem) => {
                    Hooks::to_tokens_grounditem_hook(
                        plugin_hook_name,
                        &format_ident!("{}", kind.to_string()),
                        tokens,
                    );
                }
            };
        }
    }
}

impl Hooks {
    fn to_tokens_simple_hook(name: &Ident, hook: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name() {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_gamestate_hook(name: &Ident, hook: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(state: ::macroquest::eq::GameState) {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(state),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_writechatcolor_hook(
        name: &Ident,
        hook: &Ident,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(line: &str, color: ::macroquest::eq::ChatColor) {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(line, color),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_incomingchat_hook(
        name: &Ident,
        hook: &Ident,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(line: &str, color: ::macroquest::eq::ChatColor) -> bool {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(line, color),
                    None => {
                        ::macroquest::log::error!("plugin never set");
                        false
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_str_hook(name: &Ident, hook: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(value: &str) {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(value),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_spawn_hook(name: &Ident, hook: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(spawn: &::macroquest::eq::Spawn) {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(spawn),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_grounditem_hook(
        name: &Ident,
        hook: &Ident,
        tokens: &mut proc_macro2::TokenStream,
    ) {
        let outer_name = format_ident!("__plugin_{}", name);
        let hook_name = format_ident!("{}", hook.to_string());
        quote! {
            #[::macroquest::plugin::hook(#hook_name)]
            fn #outer_name(item: &::macroquest::eq::GroundItem) {
                match PLUGIN.get() {
                    Some(plugin) => plugin.#name(item),
                    None => ::macroquest::log::error!("plugin never set"),
                }
            }
        }
        .to_tokens(tokens);
    }
}
