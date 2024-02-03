use convert_case::{Case, Casing};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::ItemStruct;

use crate::plugin::dllmain::PluginMain;

pub(crate) struct Plugin {
    body: ItemStruct,
}

impl Parse for Plugin {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body: ItemStruct = input.parse()?;

        Ok(Plugin { body })
    }
}

impl ToTokens for Plugin {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let plugin_struct = &self.body;
        let main_fn_name = format_ident!(
            "__{}",
            plugin_struct
                .ident
                .to_string()
                .as_str()
                .to_case(Case::Snake)
        );
        let type_assertion_name = format_ident!(
            "__assert_{}_implements_traits",
            plugin_struct
                .ident
                .to_string()
                .as_str()
                .to_case(Case::Snake)
        );
        let plugin_name = &plugin_struct.ident;

        let main = PluginMain::new(
            syn::parse2(quote! {
                fn #main_fn_name(reason: ::macroquest::plugin::Reason) -> ::std::primitive::bool {
                    use ::macroquest::plugin::{Reason, New};
                    use ::macroquest::log::error;

                    match reason {
                        Reason::Load => {
                            match PLUGIN.set(#plugin_name::new()) {
                                Ok(_) => true,
                                Err(error) => {
                                    error!(?error, "there was already a PLUGIN set");
                                    false
                                }
                            }
                        }
                        Reason::Unload => true,
                    }
                }
            })
            .unwrap(),
        );

        quote! {
            #plugin_struct

            // If the plugin type doesn't implement the New and Plugin traits,
            // then this function will trigger a compile error.
            fn #type_assertion_name(_: #plugin_name) where #plugin_name: ::macroquest::plugin::New + ::macroquest::plugin::Hooks {}

            macroquest::plugin::preamble!();

            static PLUGIN: ::std::sync::OnceLock<#plugin_name> = ::std::sync::OnceLock::new();

            #main
        }.to_tokens(tokens);
    }
}
