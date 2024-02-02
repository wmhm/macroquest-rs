//! The MacroQuest Plugin API and related types and functions.
//!
//! The plugin API exposed by MacroQuest is a series of module level functions
//! with specific, well known names and type signatures that it will call into
//! whenever certain events occur. Plugins may implement any number of these
//! functions to implement their functionality.
//!
//! # Examples
//!
//! Use the high level API to create a basic, but useless, plugin.
//!
//! ```
//! # use macroquest::log::trace;
//! # use macroquest::eq::ChatColor;
//! # use macroquest::plugin::Plugin;
//! # use std::sync::RwLock;
//! #[derive(Debug, Default)]
//! #[macroquest::plugin::main]
//! struct MyPlugin {
//!     last: RwLock<Option<String>>,
//! }
//!
//! #[macroquest::plugin::hooks]
//! impl Plugin for MyPlugin {
//!     fn on_incoming_chat(&self, line: &str, color: ChatColor) -> bool {
//!         let mut l = self.last.write().unwrap();
//!         *l = Some(line.to_string());
//!
//!         false
//!     }
//! }
//! ```
//!
//! Use the low level API to create a basic, but useless, plugin.
//!
//! ```
//! # use std::sync::OnceLock;
//! # use macroquest::log::trace;
//! # use macroquest::plugin::Reason;
//! # use macroquest::eq::ChatColor;
//! static DATA: OnceLock<String> = OnceLock::new();
//!
//! macroquest::plugin::preamble!();
//!
//! #[macroquest::plugin::main]
//! fn pmain(reason: Reason) {
//!     match reason {
//!         Reason::Load => {
//!             trace!("module loaded");
//!
//!             DATA.set(String::new());
//!         }
//!         Reason::Unload => trace!("module unloaded"),
//!     };
//! }
//!
//! #[macroquest::plugin::hook(InitializePlugin)]
//! fn initialize() {
//!     trace!("plugin initialized")
//! }
//!
//! #[macroquest::plugin::hook(OnIncomingChat)]
//! fn incoming_chat(line: &str, color: ChatColor) -> bool {
//!     trace!(?line, ?color, "got a new line of chat");
//!
//!     false
//! }
//! ```

use num_enum::TryFromPrimitive;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[doc(inline)]
pub use macroquest_proc_macros::{
    plugin_hook as hook, plugin_hooks as hooks, plugin_main as main, plugin_preamble as preamble,
};

use crate::eq;

/// Describes the reason that the plugin `main` function is being called.
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Reason {
    /// The DLL is being loaded into memory
    Load = DLL_PROCESS_ATTACH,
    /// The DLL is being unloaded from memory
    Unload = DLL_PROCESS_DETACH,
}

/// Helper type that is used to allow the [`main`] macro to support multiple
/// return types. Functionally acts as an adapter from Any supported type into
/// [`std::primitive::bool`].
///
/// Each variant represents another possible return type that we support for the
/// decorated `main` function.
///
/// This type is technically an implementation detail, but needs to be exposed
/// as pub because the [`main`] macro will generate code that uses it within the
/// user's own crate.
#[doc(hidden)]
pub enum MainResult {
    Unit,
    Bool(bool),
}

/// Adapt a given [`MainResult`] into a bool for return to the OS when
/// Windows calls the `DllMain` function.
///
/// If this returns [`false`](std::primitive::bool) then the module will be
/// unloaded immediately.
impl From<MainResult> for bool {
    fn from(value: MainResult) -> Self {
        match value {
            MainResult::Unit => true,
            MainResult::Bool(b) => b,
        }
    }
}

impl From<()> for MainResult {
    #[allow(clippy::ignored_unit_patterns)]
    fn from(_: ()) -> Self {
        MainResult::Unit
    }
}

impl From<bool> for MainResult {
    fn from(value: bool) -> Self {
        MainResult::Bool(value)
    }
}

/// Provides a way to create new instances of a plugin type.
///
/// When using plugin types and the high level plugin API, this trait is used
/// when creating the global instance of the Plugin type.
///
/// This trait has a blanket implementation for [`std::default::Default`] and
/// implementing that trait should be preferred unless you need different
/// behavior specific to when creating the global instance of the plugin type
/// for loading into MacroQuest.
pub trait New {
    /// Creates the new instance of the plugin type.
    fn new() -> Self;
}

impl<T: Default> New for T {
    fn new() -> Self {
        Self::default()
    }
}

/// The Plugin trait implements the protocol that a MacroQuest plugin must
/// implement.
///
/// For each process, there is one global plugin instance, created using the
/// [`New::new()`] function, and the MacroQuest plugin hooks will get
/// dispatched to the instance methods of that plugin instance.
///
/// All MacroQuest plugin hooks have a default, no-op implementation, allowing
/// a Plugin implementation to implement only the ones that they actually care
/// about, while leaving the no-op implementations to cover any other hook.
#[allow(unused_variables)]
pub trait Plugin {
    /// This is called once on plugin initialization and can be considered the
    /// startup routine for the plugin.
    fn initialize(&self) {}

    /// This is called once when the plugin has been asked to shutdown. The
    /// plugin has not actually shut down until this completes.
    fn shutdown(&self) {}

    /// This is called once just before the shutdown of the UI system and each
    /// time the game requests that the UI be cleaned. Most commonly this
    /// happens when a `/loadskin` command is issued, but it also occurs when
    /// reaching the character select screen and when first entering the game.
    ///
    /// One purpose of this function is to allow you to destroy any custom
    /// windows that you have created and cleanup any UI items that need to be
    /// removed.
    fn on_clean_ui(&self) {}

    /// This is called once just after the UI system is loaded. Most commonly
    /// this happens when a `/loadskin` command is issued, but it also occurs
    /// when first entering the game.
    ///
    /// One purpose of this function is to allow you to recreate any custom
    /// windows that you have setup.
    fn on_reload_ui(&self) {}

    /// This is called each time the Heads Up Display (HUD) is drawn. The HUD is
    /// responsible for the net status and packet loss bar.
    ///
    /// Note that this is not called at all if the HUD is not shown (default F11
    /// to toggle).
    ///
    /// Because the net status is updated frequently, it is recommended to have
    /// a timer or counter at the start of this call to limit the amount of
    /// times the code in this section is executed.
    fn on_draw_hud(&self) {}

    /// This is called when the [`crate::eq::GameState`] changes. It is
    /// also called once after the plugin is initialized.
    ///
    /// For a list of known [`crate::eq::GameState`] values, see the
    /// [`crate::eq::GameState`] enum. The most commonly used of these is
    /// [`crate::eq::GameState::InGame`].
    ///
    /// When zoning, this is called once after [`Plugin::on_begin_zone()`],
    /// [`Plugin::on_remove_spawn()`], and [`Plugin::on_remove_ground_item()`]
    /// are all done, and then called once again after [`Plugin::on_end_zone()`]
    /// and [`Plugin::on_add_spawn()`] are done but prior to
    /// [`Plugin::on_add_ground_item()`] and [`Plugin::on_zoned()`].
    fn on_set_game_state(&self, state: eq::GameState) {}

    /// This is called each time MQ2 goes through its heartbeat (pulse)
    /// function.
    ///
    /// Because this happens very frequently, it is recommended to have a timer
    /// or counter at the start of this call to limit the amount of times the
    /// code in this section is executed.
    fn on_pulse(&self) {}

    /// This is called each time `WriteChatColor` is called (whether by
    /// MQ2Main or by any plugin). This can be considered the "when outputting
    /// text from MQ" callback.
    ///
    /// This ignores filters on display, so if they are needed either implement
    /// them in this section or see [`Plugin::on_incoming_chat()`] where filters
    /// are already handled.
    ///
    /// If `CEverQuest::dsp_chat` is not called, and events are required,
    /// they'll need to be implemented here as well. Otherwise, see
    /// [`Plugin::on_incoming_chat()`] where that is already handled.
    ///
    /// For a list of color values, see the [`crate::eq::ChatColor`] enum.
    fn on_write_chat_color(&self, line: &str, color: eq::ChatColor) {}

    /// This is called each time a line of chat is shown. It occurs after MQ
    /// filters and chat events have been handled.  If you need to know when
    /// MQ2 has sent chat, consider using [`Plugin::on_write_chat_color()`]
    /// instead.
    ///
    /// For a list of color values, see the [`crate::eq::ChatColor`] enum.
    fn on_incoming_chat(&self, line: &str, color: eq::ChatColor) -> bool {
        false
    }

    /// This is called each time a spawn is added to a zone (ie, something
    /// spawns). It is also called for each existing spawn when a plugin first
    /// initializes.
    ///
    /// When zoning, this is called for all spawns in the zone after
    /// [`Plugin::on_end_zone()`] is called and before [`Plugin::on_zoned()`] is
    /// called.
    fn on_add_spawn(&self, spawn: &eq::Spawn) {}

    /// This is called each time a spawn is removed from a zone (ie, something
    /// despawns or is killed). It is NOT called when a plugin shuts down.
    ///
    /// When zoning, this is called for all spawns in the zone after
    /// [`Plugin::on_begin_zone()`] is called.
    fn on_remove_spawn(&self, spawn: &eq::Spawn) {}

    /// This is called each time a ground item is added to a zone (ie, something
    /// spawns). It is also called for each existing ground item when a plugin
    /// first initializes.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// [`Plugin::on_end_zone()`] is called and before [`Plugin::on_zoned()`] is
    /// called.
    fn on_add_ground_item(&self, item: &eq::GroundItem) {}

    /// This is called each time a ground item is removed from a zone (ie,
    /// something despawns or is picked up). It is NOT called when a plugin
    /// shuts down.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// [`Plugin::on_begin_zone()`] is called.
    fn on_remove_ground_item(&self, item: &eq::GroundItem) {}

    /// This is called just after entering a zone line and as the loading screen
    /// appears.
    fn on_begin_zone(&self) {}

    /// This is called just after the loading screen, but prior to the zone
    /// being fully loaded.
    ///
    /// This should occur before [`Plugin::on_add_spawn()`] and
    /// [`Plugin::on_add_ground_item()`] are called. It always occurs before
    /// [`Plugin::on_zoned()`] is called.
    fn on_end_zone(&self) {}

    /// This is called after entering a new zone and the zone is considered
    /// "loaded."
    ///
    /// It occurs after [`Plugin::on_end_zone()`], [`Plugin::on_add_spawn()`],
    /// and [`Plugin::on_add_ground_item()`] have been called.
    fn on_zoned(&self) {}

    /// This is called each time that the ImGui overlay is rendered. Use this to
    /// render and update plugin specific widgets.
    ///
    /// Because this happens extremely frequently, it is recommended to move any
    /// actual work to a separate call and use this only for updating the
    /// display.
    fn on_update_imgui(&self) {}

    /// This is called each time a macro starts (ex: `/mac somemacro.mac`),
    /// prior to launching the macro.
    fn on_macro_start(&self, name: &str) {}

    /// This is called each time a macro stops (ex: `/endmac`), after the
    /// macro has ended.
    fn on_macro_stop(&self, name: &str) {}

    /// This is called each time a plugin is loaded
    /// (ex: `/plugin someplugin`), after the plugin has been loaded and any
    /// associated `-AutoExec.cfg` file have been launched.
    ///
    /// This means it will be executed after the [`Plugin::initialize()`]
    /// callback.
    ///
    /// This is also called when THIS plugin is loaded, but initialization tasks
    /// should still be done in [`Plugin::initialize()`].
    fn on_plugin_load(&self, name: &str) {}

    /// This is called each time a plugin is unloaded
    /// (ex: `/plugin someplugin unload`), just prior to the plugin unloading.
    /// This means it will be executed prior to the [`Plugin::shutdown()`]
    /// callback.
    ///
    /// This is also called when THIS plugin is unloaded, but shutdown tasks
    /// should still be done in [`Plugin::shutdown()`].
    fn on_plugin_unload(&self, name: &str) {}
}
