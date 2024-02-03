use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::ItemFn;

pub(crate) struct PluginMain {
    body: ItemFn,
}

impl PluginMain {
    pub(crate) fn new(body: ItemFn) -> Self {
        PluginMain { body }
    }
}

impl Parse for PluginMain {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let body: ItemFn = input.parse()?;

        Ok(PluginMain::new(body))
    }
}

impl ToTokens for PluginMain {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let main_fn = &self.body;
        let main_fn_name = main_fn.sig.ident.clone();

        quote! {
            #main_fn

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
