use std::str::FromStr;

use proc_macro_error::abort;
use quote::{format_ident, quote, ToTokens};
use strum::EnumString;
use syn::fold::Fold;
use syn::parse::{Parse, ParseStream};
use syn::{ImplItemFn, ItemImpl};

#[derive(Debug, PartialEq, EnumString, strum::Display)]
enum Kind {
    #[strum(serialize = "initialize", to_string = "InitializePlugin")]
    InitializePlugin,
    #[strum(serialize = "shutdown", to_string = "ShutdownPlugin")]
    ShutdownPlugin,
    #[strum(serialize = "clean_ui", to_string = "OnCleanUI")]
    OnCleanUI,
    #[strum(serialize = "reload_ui", to_string = "OnReloadUI")]
    OnReloadUI,
    #[strum(serialize = "draw_hud", to_string = "OnDrawHUD")]
    OnDrawHUD,
    #[strum(serialize = "game_state", to_string = "SetGameState")]
    SetGameState,
    #[strum(serialize = "pulse", to_string = "OnPulse")]
    OnPulse,
    #[strum(serialize = "write_chat", to_string = "OnWriteChatColor")]
    OnWriteChatColor,
    #[strum(serialize = "incoming_chat", to_string = "OnIncomingChat")]
    OnIncomingChat,
    #[strum(serialize = "add_spawn", to_string = "OnAddSpawn")]
    OnAddSpawn,
    #[strum(serialize = "remove_spawn", to_string = "OnRemoveSpawn")]
    OnRemoveSpawn,
    #[strum(serialize = "add_ground_item", to_string = "OnAddGroundItem")]
    OnAddGroundItem,
    #[strum(serialize = "remove_ground_item", to_string = "OnRemoveGroundItem")]
    OnRemoveGroundItem,
    #[strum(serialize = "begin_zone", to_string = "OnBeginZone")]
    OnBeginZone,
    #[strum(serialize = "end_zone", to_string = "OnEndZone")]
    OnEndZone,
    #[strum(serialize = "zoned", to_string = "OnZoned")]
    OnZoned,
    #[strum(serialize = "update_imgui", to_string = "OnUpdateImGui")]
    OnUpdateImGui,
    #[strum(serialize = "macro_start", to_string = "OnMacroStart")]
    OnMacroStart,
    #[strum(serialize = "macro_stop", to_string = "OnMacroStop")]
    OnMacroStop,
    #[strum(serialize = "plugin_load", to_string = "OnLoadPlugin")]
    OnLoadPlugin,
    #[strum(serialize = "plugin_unload", to_string = "OnUnloadPlugin")]
    OnUnloadPlugin,
}

pub(crate) struct Hooks {
    body:        ItemImpl,
    implemented: Vec<ImplItemFn>,
}

impl Parse for Hooks {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let plugin_impl: ItemImpl = input.parse()?;
        let mut hooks = Hooks {
            body:        plugin_impl.clone(),
            implemented: vec![],
        };

        hooks.body = hooks.fold_item_impl(plugin_impl);

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
            let Ok(kind) = Kind::from_str(hook.sig.ident.to_string().as_str())
            else {
                abort!(hook, "The hook must be a supported MacroQuest hook");
            };
            let hook_kind = format_ident!("{}", kind.to_string());

            quote! {
                macroquest::plugin::hook!(#hook_kind(PLUGIN));
            }
            .to_tokens(tokens);
        }
    }
}
