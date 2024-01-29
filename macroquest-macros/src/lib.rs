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
            _: ::macroquest::windows::HINSTANCE,
            call_reason: u32,
            _: *mut (),
        ) -> bool {
            use ::macroquest::windows::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

            match call_reason {
                DLL_PROCESS_ATTACH => #plugin.replace(Some(#plugin_t::new())),
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

    input.to_tokens(&mut output);
    implementation.to_tokens(&mut output);

    TokenStream::from(output)
}
