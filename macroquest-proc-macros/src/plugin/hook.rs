use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro_error::abort;
use quote::{format_ident, quote, ToTokens};
use strum::EnumString;
use syn::parse::{Parse, ParseStream};
use syn::{Ident, ItemFn};

#[derive(Debug, PartialEq, EnumString, strum::Display)]
enum Kind {
    InitializePlugin,
    ShutdownPlugin,
    OnCleanUI,
    OnReloadUI,
    OnDrawHUD,
    SetGameState,
    OnPulse,
    OnWriteChatColor,
    OnIncomingChat,
    OnAddSpawn,
    OnRemoveSpawn,
    OnAddGroundItem,
    OnRemoveGroundItem,
    OnBeginZone,
    OnEndZone,
    OnZoned,
    OnUpdateImGui,
    OnMacroStart,
    OnMacroStop,
    OnLoadPlugin,
    OnUnloadPlugin,
}

#[derive(Debug)]
pub(crate) struct HookOpts {
    kind: Kind,
}

impl Parse for HookOpts {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // We currently only support a single Hook, which has to be an ident
        let hook_n: Ident = input.parse()?;
        let Ok(kind) = Kind::from_str(hook_n.to_string().as_str())
        else {
            abort!(hook_n, "The hook must be a supported MacroQuest hook");
        };

        Ok(HookOpts { kind })
    }
}

pub(crate) struct Hook {
    opts: HookOpts,
    hook: ItemFn,
}

impl Hook {
    pub(crate) fn parse(attr: TokenStream, body: TokenStream) -> syn::Result<Self> {
        let opts: HookOpts = syn::parse(attr)?;
        let hook_fn: ItemFn = syn::parse(body)?;

        Ok(Hook {
            opts,
            hook: hook_fn,
        })
    }

    fn to_tokens_simple_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub extern "C" fn #mq_hook_name() {
                let result = ::std::panic::catch_unwind(|| {
                    #hook_fn_name()
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_str_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #mq_hook_name(ptr: *const ::std::os::raw::c_char) {
                let result = ::std::panic::catch_unwind(|| {
                    let c_str = ::std::ffi::CStr::from_ptr(ptr);
                    let r_str = c_str.to_string_lossy();
                    #hook_fn_name(r_str.as_ref())
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_gamestate_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub extern "C" fn #mq_hook_name(c_state: i32) {
                let result = ::std::panic::catch_unwind(|| {
                    #hook_fn_name(::macroquest::eq::GameState::from(c_state))
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_write_chat_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #mq_hook_name(
                ptr: *const ::std::os::raw::c_char,
                color: i32,
                _filter: i32,
            ) {
                let result = ::std::panic::catch_unwind(|| {
                    let c_str =::std::ffi::CStr::from_ptr(ptr);
                    let r_str = c_str.to_string_lossy();
                    #hook_fn_name(r_str.as_ref(), ::macroquest::eq::ChatColor::from(color))
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_incoming_chat_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub unsafe extern "C" fn #mq_hook_name(
                ptr: *const ::std::os::raw::c_char,
                color: u32,
            ) -> bool {
                let result = ::std::panic::catch_unwind(|| {
                    let c_str = ::std::ffi::CStr::from_ptr(ptr);
                    let r_str = c_str.to_string_lossy();
                    #hook_fn_name(r_str.as_ref(), ::macroquest::eq::ChatColor::from(color as i32))
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                        false
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_spawn_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub extern "C" fn #mq_hook_name(
                pc: &::macroquest::ffi::eqlib::PlayerClient,
            ) {
                let result = ::std::panic::catch_unwind(|| {
                    let spawn = ::std::convert::AsRef::<::macroquest::eq::Spawn>::as_ref(pc);
                    #hook_fn_name(spawn)
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }

    fn to_tokens_grounditem_hook(&self, tokens: &mut proc_macro2::TokenStream) {
        let mq_hook_name_s = self.opts.kind.to_string();
        let mq_hook_name = format_ident!("{}", mq_hook_name_s);
        let hook_fn_name = &self.hook.sig.ident;

        quote! {
            #[no_mangle]
            pub extern "C" fn #mq_hook_name(
                eq_item: &::macroquest::ffi::eqlib::EQGroundItem,
            ) {
                let result = ::std::panic::catch_unwind(|| {
                    let item = ::std::convert::AsRef::<::macroquest::eq::GroundItem>::as_ref(eq_item);
                    #hook_fn_name(item)
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        ::macroquest::log::error!(?error, hook = #mq_hook_name_s, "caught an unwind");
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}

impl ToTokens for Hook {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.hook.to_tokens(tokens);

        match self.opts.kind {
            Kind::InitializePlugin
            | Kind::ShutdownPlugin
            | Kind::OnCleanUI
            | Kind::OnReloadUI
            | Kind::OnDrawHUD
            | Kind::OnPulse
            | Kind::OnBeginZone
            | Kind::OnEndZone
            | Kind::OnZoned
            | Kind::OnUpdateImGui => self.to_tokens_simple_hook(tokens),
            Kind::OnMacroStart | Kind::OnMacroStop | Kind::OnLoadPlugin | Kind::OnUnloadPlugin => {
                self.to_tokens_str_hook(tokens);
            }
            Kind::SetGameState => self.to_tokens_gamestate_hook(tokens),
            Kind::OnWriteChatColor => self.to_tokens_write_chat_hook(tokens),
            Kind::OnIncomingChat => self.to_tokens_incoming_chat_hook(tokens),
            Kind::OnAddSpawn | Kind::OnRemoveSpawn => self.to_tokens_spawn_hook(tokens),
            Kind::OnAddGroundItem | Kind::OnRemoveGroundItem => {
                self.to_tokens_grounditem_hook(tokens);
            }
        };
    }
}
