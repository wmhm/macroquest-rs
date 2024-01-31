#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use std::path::PathBuf;

use darling::ast::NestedMeta;
use darling::util::Override;
use darling::{Error, FromMeta};
use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemStruct};

#[derive(Debug, Clone, FromMeta)]
enum LevelFilter {
    #[darling(rename = "off")]
    Off,
    #[darling(rename = "error")]
    Error,
    #[darling(rename = "warn")]
    Warn,
    #[darling(rename = "info")]
    Info,
    #[darling(rename = "debug")]
    Debug,
    #[darling(rename = "trace")]
    Trace,
}

#[derive(Debug, Clone, FromMeta)]
struct ConsoleLogging {
    level: LevelFilter,
}

impl Default for ConsoleLogging {
    fn default() -> Self {
        ConsoleLogging {
            level: LevelFilter::Debug,
        }
    }
}

impl ToTokens for ConsoleLogging {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let level = match self.level {
            LevelFilter::Off => quote! { ::macroquest::log::logger::LevelFilter::OFF },
            LevelFilter::Error => quote! { ::macroquest::log::logger::LevelFilter::ERROR },
            LevelFilter::Warn => quote! { ::macroquest::log::logger::LevelFilter::WARN },
            LevelFilter::Info => quote! { ::macroquest::log::logger::LevelFilter::INFO },
            LevelFilter::Debug => quote! { ::macroquest::log::logger::LevelFilter::DEBUG },
            LevelFilter::Trace => quote! { ::macroquest::log::logger::LevelFilter::TRACE },
        };

        (quote! { Some(#level) }).to_tokens(tokens);
    }
}

#[derive(Debug, Clone, FromMeta)]
struct FileLogging {
    level: Option<LevelFilter>,
    filename: Option<PathBuf>,
}

impl FileLogging {
    fn with_plugin_name(mut self, name: Option<String>) -> Self {
        self.filename = self.filename.or_else(|| name.map(PathBuf::from));
        self
    }
}

impl Default for FileLogging {
    fn default() -> Self {
        FileLogging {
            level: Some(LevelFilter::Debug),
            filename: None,
        }
    }
}

impl ToTokens for FileLogging {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let level = match self.level {
            Some(LevelFilter::Off) => quote! { ::macroquest::log::logger::LevelFilter::OFF },
            Some(LevelFilter::Error) => quote! { ::macroquest::log::logger::LevelFilter::ERROR },
            Some(LevelFilter::Warn) => quote! { ::macroquest::log::logger::LevelFilter::WARN },
            Some(LevelFilter::Info) => quote! { ::macroquest::log::logger::LevelFilter::INFO },
            Some(LevelFilter::Trace) => quote! { ::macroquest::log::logger::LevelFilter::TRACE },
            Some(LevelFilter::Debug) | None => {
                quote! { ::macroquest::log::logger::LevelFilter::DEBUG }
            }
        };

        let filename = self
            .filename
            .as_ref()
            .expect("does not have a filename")
            .to_string_lossy();

        (quote! { Some((#level, #filename)) }).to_tokens(tokens);
    }
}

#[derive(Debug, Default, FromMeta)]
struct Logging {
    console: Option<Override<ConsoleLogging>>,
    file: Option<Override<FileLogging>>,
}

impl Logging {
    fn with_plugin_name(mut self, name: Option<String>) -> Self {
        self.file = self
            .file
            .map(|f| Override::Explicit(f.unwrap_or_default().with_plugin_name(name)));
        self
    }
}

impl ToTokens for Logging {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let console = self
            .console
            .as_ref()
            .map(|c| c.clone().unwrap_or_default())
            .unwrap_or_default();

        let file = self
            .file
            .as_ref()
            .map(|f| f.clone().unwrap_or_default())
            .unwrap_or_default();

        (quote! {
            ::macroquest::log::logger::init(#console, #file);
        })
        .to_tokens(tokens);
    }
}

#[derive(Debug, FromMeta)]
struct PluginArgs {
    name: Option<String>,
    logging: Option<Override<Logging>>,
}

#[proc_macro_attribute]
#[allow(clippy::too_many_lines)]
pub fn plugin(args: TokenStream, stream: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };

    let mut args = match PluginArgs::from_list(&args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let input = parse_macro_input!(stream as ItemStruct);

    if args.name.is_none() {
        args.name = Some(input.ident.to_string());
    }

    let plugin_t = format_ident!("{}", input.ident);
    let plugin = format_ident!("__{}", input.ident.to_string().to_uppercase());
    let logging = args
        .logging
        .map(|l| l.unwrap_or_default().with_plugin_name(args.name.clone()));

    let eq_version_str = include_str!(concat!(env!("OUT_DIR"), "/eq_version.txt")).as_bytes();

    let implementation = quote! {
        #[no_mangle]
        pub static IsBuiltForNext: bool = true;

        #[no_mangle]
        pub static EverQuestVersion: [u8; 21] = [#(#eq_version_str),*,0];


        static #plugin: ::macroquest::PluginHandler<#plugin_t> = ::macroquest::PluginHandler::new();

        #[no_mangle]
        extern "system" fn DllMain(
            _: ::macroquest::windows::HINSTANCE,
            call_reason: u32,
            _: *mut (),
        ) -> bool {
            use ::macroquest::windows::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

            match call_reason {
                DLL_PROCESS_ATTACH => {
                    #logging
                    #plugin.replace(Some(#plugin_t::default()))
                }
                DLL_PROCESS_DETACH => #plugin.replace(None),
                _ => {}
            }

            true
        }

        #[no_mangle]
        pub fn InitializePlugin() {
            #plugin.initialize()
        }

        #[no_mangle]
        pub fn ShutdownPlugin() {
            #plugin.shutdown()
        }

        #[no_mangle]
        pub fn OnCleanUI() {
            #plugin.on_clean_ui()
        }

        #[no_mangle]
        pub fn OnReloadUI() {
            #plugin.on_reload_ui()
        }

        #[no_mangle]
        pub fn OnDrawHUD() {
            #plugin.on_draw_hud()
        }

        #[no_mangle]
        pub fn SetGameState(state: i32) {
            #plugin.on_set_game_state(state)
        }

        #[no_mangle]
        pub fn OnPulse() {
            #plugin.on_pulse()
        }

        #[no_mangle]
        pub unsafe fn OnWriteChatColor(
            line_ptr: *const ::std::os::raw::c_char,
            color: i32,
            _filter: i32,  // Per brainiac, filter appears to be unused
        ) {
            #plugin.on_write_chat_color(line_ptr, color)
        }

        #[no_mangle]
        pub unsafe fn OnIncomingChat(
            line_ptr: *const ::std::os::raw::c_char,
            color: u32,
        ) -> bool {
            #plugin.on_incoming_chat(line_ptr, color as i32)
        }

        #[no_mangle]
        pub unsafe fn OnAddSpawn(spawn: *const ::macroquest::ffi::eqlib::PlayerClient) {
            #plugin.on_add_spawn(spawn)
        }

        #[no_mangle]
        pub unsafe fn OnRemoveSpawn(spawn: *const ::macroquest::ffi::eqlib::PlayerClient) {
            #plugin.on_remove_spawn(spawn)
        }

        #[no_mangle]
        pub unsafe fn OnAddGroundItem(item: *const ::macroquest::ffi::eqlib::EQGroundItem) {
            #plugin.on_add_ground_item(item)
        }

        #[no_mangle]
        pub unsafe fn OnRemoveGroundItem(item: *const ::macroquest::ffi::eqlib::EQGroundItem) {
            #plugin.on_remove_ground_item(item)
        }

        #[no_mangle]
        pub fn OnBeginZone() {
            #plugin.on_begin_zone()
        }

        #[no_mangle]
        pub fn OnEndZone() {
            #plugin.on_end_zone()
        }

        #[no_mangle]
        pub fn OnZoned() {
            #plugin.on_zoned()
        }

        #[no_mangle]
        pub fn OnUpdateImGui() {
            #plugin.on_update_imgui()
        }

        #[no_mangle]
        pub unsafe fn OnMacroStart(name_ptr: *const ::std::os::raw::c_char) {
            #plugin.on_macro_start(name_ptr)
        }

        #[no_mangle]
        pub unsafe fn OnMacroStop(name_ptr: *const ::std::os::raw::c_char) {
            #plugin.on_macro_stop(name_ptr)
        }

        #[no_mangle]
        pub unsafe fn OnLoadPlugin(name_ptr: *const ::std::os::raw::c_char) {
            #plugin.on_plugin_load(name_ptr)
        }

        #[no_mangle]
        pub unsafe fn OnUnloadPlugin(name_ptr: *const ::std::os::raw::c_char) {
            #plugin.on_plugin_unload(name_ptr)
        }
    };

    let mut output = proc_macro2::TokenStream::new();

    input.to_tokens(&mut output);
    implementation.to_tokens(&mut output);

    TokenStream::from(output)
}
