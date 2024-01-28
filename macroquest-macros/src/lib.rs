use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

mod ffi;

#[proc_macro_attribute]
pub fn plugin(_args: TokenStream, stream: TokenStream) -> TokenStream {
    let input = parse_macro_input!(stream as ItemStruct);
    let mut output = proc_macro2::TokenStream::new();

    let plugin_t = format_ident!("{}", input.ident);
    let plugin = format_ident!("__{}", input.ident.to_string().to_uppercase());

    let eq_version_str = format!("{} {}", ffi::eq_version(), ffi::eq_time()).into_bytes();

    let implementation = quote! {
        #[no_mangle]
        pub static IsBuiltForNext: bool = true;

        #[no_mangle]
        pub static EverQuestVersion: [u8; 21] = [#(#eq_version_str),*,0];


        static #plugin: ::macroquest::PluginHandler<#plugin_t> = ::macroquest::PluginHandler::new();

        #[no_mangle]
        extern "system" fn DllMain(
            _: windows::Win32::Foundation::HINSTANCE,
            call_reason: u32,
            _: *mut (),
        ) -> bool {
            use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

            match call_reason {
                DLL_PROCESS_ATTACH => #plugin.replace(Some(#plugin_t::new())),
                DLL_PROCESS_DETACH => #plugin.replace(None),
                _ => {}
            }

            true
        }

        #[no_mangle]
        pub fn InitializePlugin() {
            #plugin.hook_initialize()
        }
    };

    input.to_tokens(&mut output);
    implementation.to_tokens(&mut output);

    TokenStream::from(output)
}
