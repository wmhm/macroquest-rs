use convert_case::{Case, Casing};
use quote::{format_ident, quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{ItemFn, ItemStruct, Token};

enum Kind {
    Function(ItemFn),
    Struct(ItemStruct),
}

pub(crate) struct PluginMain {
    kind: Kind,
}

impl Parse for PluginMain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        let kind = if lookahead.peek(Token![fn]) {
            input.parse::<ItemFn>().map(Kind::Function)
        } else if lookahead.peek(Token![struct]) {
            input.parse::<ItemStruct>().map(Kind::Struct)
        } else {
            Err(lookahead.error())
        }?;

        Ok(PluginMain { kind })
    }
}

impl ToTokens for PluginMain {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let main_fn_name = match &self.kind {
            Kind::Function(main_fn) => {
                quote! {
                    #main_fn
                }
                .to_tokens(tokens);

                main_fn.sig.ident.clone()
            }
            Kind::Struct(plugin_struct) => {
                let mut plugin_struct = plugin_struct.clone();
                let main_fn_name = format_ident!(
                    "__{}",
                    plugin_struct
                        .ident
                        .to_string()
                        .as_str()
                        .to_case(Case::Snake)
                );
                let type_assertion_name = format_ident!(
                    "__assert_{}_implements_plugin_trait",
                    plugin_struct
                        .ident
                        .to_string()
                        .as_str()
                        .to_case(Case::Snake)
                );
                let plugin_name = &plugin_struct.ident;

                let where_clause = plugin_struct.generics.make_where_clause();
                where_clause
                    .predicates
                    .push(syn::parse2(quote!(#plugin_name: ::macroquest::plugin::New)).unwrap());

                quote! {
                    macroquest::plugin::preamble!();
                    static PLUGIN: ::std::sync::OnceLock<#plugin_name> = ::std::sync::OnceLock::new();

                    #plugin_struct

                    // If the plugin type doesn't implement the Plugin trait, then this
                    // function will trigger a compile error.
                    fn #type_assertion_name(_: #plugin_name) where #plugin_name: ::macroquest::plugin::Plugin {}

                    fn #main_fn_name(reason: ::macroquest::plugin::Reason) -> bool {
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
                }.to_tokens(tokens);

                main_fn_name
            }
        };

        quote! {
            #[no_mangle]
            extern "system" fn DllMain(_: *mut (), c_reason: u32, _: *mut ()) -> bool {
                use ::macroquest::log::error;

                let result = ::std::panic::catch_unwind(|| {
                    use ::std::convert::TryFrom;

                    use ::macroquest::plugin::{Reason, MainResult};

                    let rvalue = match Reason::try_from(c_reason) {
                        Ok(reason) => Into::<MainResult>::into(#main_fn_name(reason)),
                        Err(_) => {
                            error!(reason = c_reason, "unknown reason in DllMain");

                            MainResult::Bool(false)
                        }
                    };

                    rvalue.into()
                });

                match result {
                    Ok(r) => r,
                    Err(error) => {
                        error!(?error, hook = "PluginMain", "caught an unwind");
                        false
                    }
                }
            }
        }
        .to_tokens(tokens);
    }
}
