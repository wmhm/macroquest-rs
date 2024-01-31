use std::ffi::CStr;
use std::os::raw::c_char;

use crate::eq::{ChatColor, GameState, GroundItem, Spawn};
use crate::ffi;

/// The Plugin trait implements the protocol that a MacroQuest plugin must
/// implement.
///
/// For each process, there is one global plugin instance, created using the
/// Plugin::new() function, and the MacroQuest plugin hooks will get dispatched
/// to the instance methods of that plugin instance.
///
/// All MacroQuest plugin hooks have a default, no-op implementation, allowing
/// a Plugin implementation to implement only the ones that they actually care
/// about, while leaving the no-op implementations to cover any other hook.
#[allow(unused_variables)]
pub trait Plugin: Default {
    /// This is called once on plugin initialization and can be considered the
    /// startup routine for the plugin.
    fn initialize(&mut self) {}

    /// This is called once when the plugin has been asked to shutdown. The
    /// plugin has not actually shut down until this completes.
    fn shutdown(&mut self) {}

    /// This is called once just before the shutdown of the UI system and each
    /// time the game requests that the UI be cleaned. Most commonly this
    /// happens when a /loadskin command is issued, but it also occurs when
    /// reaching the character select screen and when first entering the game.
    ///
    /// One purpose of this function is to allow you to destroy any custom
    /// windows that you have created and cleanup any UI items that need to be
    /// removed.
    fn on_clean_ui(&mut self) {}

    /// This is called once just after the UI system is loaded. Most commonly
    /// this happens when a /loadskin command is issued, but it also occurs when
    /// first entering the game.
    ///
    /// One purpose of this function is to allow you to recreate any custom
    /// windows that you have setup.
    fn on_reload_ui(&mut self) {}

    /// This is called each time the Heads Up Display (HUD) is drawn. The HUD is
    /// responsible for the net status and packet loss bar.
    ///
    /// Note that this is not called at all if the HUD is not shown (default F11
    /// to toggle).
    ///
    /// Because the net status is updated frequently, it is recommended to have
    /// a timer or counter at the start of this call to limit the amount of
    /// times the code in this section is executed.
    fn on_draw_hud(&mut self) {}

    /// This is called when the GameState changes. It is also called once after
    /// the plugin is initialized.
    ///
    /// For a list of known GameState values, see the GameState enum. The most
    /// commonly used of these is GameState::InGame.
    ///
    /// When zoning, this is called once after on_begin_zone, on_remove_spawn,
    /// and on_remove_ground_item are all done, and then called once again after
    /// on_end_zone and on_add_spawn are done but prior to on_add_ground_item
    /// and on_zoned.
    fn on_set_game_state(&mut self, state: GameState) {}

    /// This is called each time MQ2 goes through its heartbeat (pulse) function.
    ///
    /// Because this happens very frequently, it is recommended to have a timer
    /// or counter at the start of this call to limit the amount of times the
    /// code in this section is executed.
    fn on_pulse(&mut self) {}

    /// This is called each time WriteChatColor is called (whether by MQ2Main or
    /// by any plugin).  This can be considered the "when outputting text from
    /// MQ" callback.
    ///
    /// This ignores filters on display, so if they are needed either implement
    /// them in this section or see on_incoming_chat where filters are already
    /// handled.
    ///
    /// If CEverQuest::dsp_chat is not called, and events are required, they'll
    /// need to be implemented here as well. Otherwise, see on_incoming_chat
    /// where that is already handled.
    ///
    /// For a list of Color values, see the UserColor enum.
    fn on_write_chat_color(&mut self, line: &str, color: ChatColor) {}

    /// This is called each time a line of chat is shown. It occurs after MQ
    /// filters and chat events have been handled.  If you need to know when
    /// MQ2 has sent chat, consider using o_write_chat_color instead.
    ///
    /// For a list of Color values, see the UserColor enum.
    fn on_incoming_chat(&mut self, line: &str, color: ChatColor) -> bool {
        false
    }

    /// This is called each time a spawn is added to a zone (ie, something
    /// spawns). It is also called for each existing spawn when a plugin first
    /// initializes.
    ///
    /// When zoning, this is called for all spawns in the zone after on_end_zone
    /// is called and before on_zoned is called.
    fn on_add_spawn(&mut self, spawn: &Spawn) {}

    /// This is called each time a spawn is removed from a zone (ie, something
    /// despawns or is killed). It is NOT called when a plugin shuts down.
    ///
    /// When zoning, this is called for all spawns in the zone after
    /// on_begin_zone is called.
    fn on_remove_spawn(&mut self, spawn: &Spawn) {}

    /// This is called each time a ground item is added to a zone (ie, something
    /// spawns). It is also called for each existing ground item when a plugin
    /// first initializes.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// on_end_zone is called and before on_zoned is called.
    fn on_add_ground_item(&mut self, item: &GroundItem) {}

    /// This is called each time a ground item is removed from a zone (ie,
    /// something despawns or is picked up). It is NOT called when a plugin
    /// shuts down.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// on_begin_zone is called.
    fn on_remove_ground_item(&mut self, item: &GroundItem) {}

    /// This is called just after entering a zone line and as the loading screen
    /// appears.
    fn on_begin_zone(&mut self) {}

    /// This is called just after the loading screen, but prior to the zone
    /// being fully loaded.
    ///
    /// This should occur before on_add_spawn and on_add_ground_item are called.
    /// It always occurs before on_zoned is called.
    fn on_end_zone(&mut self) {}

    /// This is called after entering a new zone and the zone is considered
    /// "loaded."
    ///
    /// It occurs after on_end_zone, on_add_spawn, and on_add_ground_item have
    /// been called.
    fn on_zoned(&mut self) {}

    /// This is called each time that the ImGui Overlay is rendered. Use this to
    /// render and update plugin specific widgets.
    ///
    /// Because this happens extremely frequently, it is recommended to move any
    /// actual work to a separate call and use this only for updating the
    /// display.
    fn on_update_imgui(&mut self) {}

    /// This is called each time a macro starts (ex: /mac somemacro.mac), prior
    /// to launching the macro.
    fn on_macro_start(&mut self, name: &str) {}

    /// This is called each time a macro stops (ex: /endmac), after the macro
    /// has ended.
    fn on_macro_stop(&mut self, name: &str) {}

    /// This is called each time a plugin is loaded (ex: /plugin someplugin),
    /// after the plugin has been loaded and any associated -AutoExec.cfg file
    /// have been launched.
    ///
    /// This means it will be executed after the plugin's initialize callback.
    ///
    /// This is also called when THIS plugin is loaded, but initialization tasks
    /// should still be done in initialize.
    fn on_plugin_load(&mut self, name: &str) {}

    /// This is called each time a plugin is unloaded (ex: /plugin someplugin unload),
    /// just prior to the plugin unloading. This means it will be executed prior
    /// to that plugin's shutdown callback.
    ///
    /// This is also called when THIS plugin is unloaded, but shutdown tasks
    /// should still be done in shutdown.
    fn on_plugin_unload(&mut self, name: &str) {}
}

#[doc(hidden)]
pub struct PluginHandler<T: Plugin> {
    data: parking_lot::Mutex<Option<T>>,
}

#[allow(clippy::missing_safety_doc)]
impl<T: Plugin> PluginHandler<T> {
    pub const fn new() -> PluginHandler<T> {
        PluginHandler {
            data: parking_lot::Mutex::new(None),
        }
    }

    pub fn replace(&self, new: Option<T>) {
        let mut plugin = self.data.lock();
        *plugin = new;
    }

    simple_hook!(initialize);
    simple_hook!(shutdown);
    simple_hook!(on_clean_ui);
    simple_hook!(on_reload_ui);
    simple_hook!(on_draw_hud);
    simple_hook!(on_pulse);
    simple_hook!(on_begin_zone);
    simple_hook!(on_end_zone);
    simple_hook!(on_zoned);
    simple_hook!(on_update_imgui);

    str_hook!(on_macro_start);
    str_hook!(on_macro_stop);
    str_hook!(on_plugin_load);
    str_hook!(on_plugin_unload);

    pub fn on_set_game_state<S: Into<GameState>>(&self, state: S) {
        hook!(self, on_set_game_state, state.into())
    }

    pub unsafe fn on_write_chat_color<C: Into<ChatColor>>(&self, ptr: *const c_char, color: C) {
        let value = CStr::from_ptr(ptr);

        match value.to_str() {
            Ok(s) => hook!(self, on_write_chat_color, s, color.into()),
            Err(_) => todo!("figure out error handling"),
        }
    }

    pub unsafe fn on_incoming_chat<C: Into<ChatColor>>(
        &self,
        ptr: *const c_char,
        color: C,
    ) -> bool {
        let value = CStr::from_ptr(ptr);

        match value.to_str() {
            Ok(s) => hook!(self, on_incoming_chat, s, color.into()),
            Err(_) => todo!("figure out error handling"),
        }
    }

    pub unsafe fn on_add_spawn(&self, ptr: *const ffi::eqlib::PlayerClient) {
        match ptr.as_ref() {
            Some(ffi_item) => {
                let item = Spawn(ffi_item);
                hook!(self, on_add_spawn, &item)
            }
            None => todo!("figure out error handling"),
        }
    }

    pub unsafe fn on_remove_spawn(&self, ptr: *const ffi::eqlib::PlayerClient) {
        match ptr.as_ref() {
            Some(ffi_item) => {
                let item = Spawn(ffi_item);
                hook!(self, on_remove_spawn, &item)
            }
            None => todo!("figure out error handling"),
        }
    }

    pub unsafe fn on_add_ground_item(&self, ptr: *const ffi::eqlib::EQGroundItem) {
        match ptr.as_ref() {
            Some(ffi_item) => {
                let item = GroundItem(ffi_item);
                hook!(self, on_add_ground_item, &item)
            }
            None => todo!("figure out error handling"),
        }
    }

    pub unsafe fn on_remove_ground_item(&self, ptr: *const ffi::eqlib::EQGroundItem) {
        match ptr.as_ref() {
            Some(ffi_item) => {
                let item = GroundItem(ffi_item);
                hook!(self, on_remove_ground_item, &item)
            }
            None => todo!("figure out error handling"),
        }
    }
}

mod macros {

    macro_rules! hook {
        ($handler:ident, $hook:ident, $($param:expr),*) => {{
            let mut lock = $handler.data.lock();
            let plugin: &mut T = lock.as_mut().expect("no plugin");

            plugin.$hook($($param),*)
        }};
    }

    macro_rules! simple_hook {
        ($hook:ident) => {
            pub fn $hook(&self) {
                let mut lock = self.data.lock();
                let plugin: &mut T = lock.as_mut().expect("no plugin");

                plugin.$hook()
            }
        };
    }

    macro_rules! str_hook {
        ($hook:ident) => {
            #[allow(clippy::missing_safety_doc)]
            pub unsafe fn $hook(&self, ptr: *const c_char) {
                let value = CStr::from_ptr(ptr);

                match value.to_str() {
                    Ok(s) => {
                        let mut lock = self.data.lock();
                        let plugin: &mut T = lock.as_mut().expect("no plugin");

                        plugin.$hook(s)
                    }
                    Err(_) => todo!("figure out error handling"),
                }
            }
        };
    }

    pub(super) use {hook, simple_hook, str_hook};
}
use macros::{hook, simple_hook, str_hook};
