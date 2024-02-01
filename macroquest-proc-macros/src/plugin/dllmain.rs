use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::ItemFn;

pub(crate) struct PluginMain(ItemFn);

impl Parse for PluginMain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fn_: ItemFn = input.parse()?;

        Ok(PluginMain(fn_))
    }
}

impl ToTokens for PluginMain {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let main_fn = &self.0;
        let main_fn_name = &self.0.sig.ident;

        quote! {
            #main_fn

            #[no_mangle]
            extern "system" fn DllMain(_: *mut (), c_reason: u32, _: *mut ()) -> bool {
                use ::std::convert::TryFrom;

                use ::macroquest::log::error;
                use ::macroquest::plugin::{Reason, PluginMainResult};

                let rvalue = match Reason::try_from(c_reason) {
                    Ok(reason) => Into::<PluginMainResult>::into(#main_fn_name(reason)),
                    Err(_) => {
                        error!(reason = c_reason, "unknown reason in DllMain");

                        PluginMainResult::Bool(false)
                    }
                };

                rvalue.into()
            }
        }
        .to_tokens(tokens);
    }
}
