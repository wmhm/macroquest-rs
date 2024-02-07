//! The MacroQuest Plugin API and related types and functions.
//!
//! The plugin API exposed by MacroQuest is a series of module level functions
//! with specific, well known names and type signatures that it will call into
//! whenever certain events occur. Plugins may implement any number of these
//! functions to implement their functionality.
//!
//! # Examples
//!
//! Use the API to create a basic, but useless, plugin.
//!
//! ```
//! # use macroquest::log::trace;
//! # use macroquest::eq::ChatColor;
//! # use macroquest::plugin::Hooks;
//! # use std::sync::RwLock;
//!
//! macroquest::plugin::setup!(MyPlugin, 1.0);
//!
//! #[derive(Debug, Default)]
//! struct MyPlugin {
//!     last: RwLock<Option<String>>,
//! }
//!
//! #[macroquest::plugin::hooks]
//! impl Hooks for MyPlugin {
//!     fn incoming_chat(&self, line: &str, color: ChatColor) -> bool {
//!         let mut l = self.last.write().unwrap();
//!         *l = Some(line.to_string());
//!
//!         false
//!     }
//! }
//! ```

use num_enum::TryFromPrimitive;
use windows::Win32::System::SystemServices::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH};

#[doc(inline)]
pub use macroquest_proc_macros::{plugin_hook as hook, plugin_hooks as hooks};

use crate::eq;

/// Describes the reason that the plugin `main` function is being called.
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u32)]
pub enum Reason {
    /// The DLL is being loaded into memory
    Load   = DLL_PROCESS_ATTACH,
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

/// The Hooks trait implements the protocol that a MacroQuest plugin must
/// implement.
///
/// For each process, there is one global plugin instance, created using the
/// [`New::new()`] function, and the MacroQuest plugin hooks will get
/// dispatched to the instance methods of that plugin instance.
///
/// All MacroQuest plugin hooks have a default, no-op implementation, allowing
/// a Hooks implementation to implement only the ones that they actually care
/// about, while leaving the no-op implementations to cover any other hook.
#[allow(unused_variables)]
pub trait Hooks {
    /// This is called once on plugin initialization and can be considered the
    /// startup routine for the plugin.
    #[doc(alias = "InitializePlugin")]
    fn initialize(&self) {}

    /// This is called once when the plugin has been asked to shutdown. The
    /// plugin has not actually shut down until this completes.
    #[doc(alias = "ShutdownPlugin")]
    fn shutdown(&self) {}

    /// This is called once just before the shutdown of the UI system and each
    /// time the game requests that the UI be cleaned. Most commonly this
    /// happens when a `/loadskin` command is issued, but it also occurs when
    /// reaching the character select screen and when first entering the game.
    ///
    /// One purpose of this function is to allow you to destroy any custom
    /// windows that you have created and cleanup any UI items that need to be
    /// removed.
    #[doc(alias = "OnCleanUI")]
    fn clean_ui(&self) {}

    /// This is called once just after the UI system is loaded. Most commonly
    /// this happens when a `/loadskin` command is issued, but it also occurs
    /// when first entering the game.
    ///
    /// One purpose of this function is to allow you to recreate any custom
    /// windows that you have setup.
    #[doc(alias = "OnReloadUI")]
    fn reload_ui(&self) {}

    /// This is called each time the Heads Up Display (HUD) is drawn. The HUD is
    /// responsible for the net status and packet loss bar.
    ///
    /// Note that this is not called at all if the HUD is not shown (default F11
    /// to toggle).
    ///
    /// Because the net status is updated frequently, it is recommended to have
    /// a timer or counter at the start of this call to limit the amount of
    /// times the code in this section is executed.
    #[doc(alias = "OnDrawHUD")]
    fn draw_hud(&self) {}

    /// This is called when the [`crate::eq::GameState`] changes. It is
    /// also called once after the plugin is initialized.
    ///
    /// For a list of known [`crate::eq::GameState`] values, see the
    /// [`crate::eq::GameState`] enum. The most commonly used of these is
    /// [`crate::eq::GameState::InGame`].
    ///
    /// When zoning, this is called once after [`Hooks::begin_zone()`],
    /// [`Hooks::remove_spawn()`], and [`Hooks::remove_ground_item()`]
    /// are all done, and then called once again after [`Hooks::end_zone()`]
    /// and [`Hooks::add_spawn()`] are done but prior to
    /// [`Hooks::add_ground_item()`] and [`Hooks::zoned()`].
    #[doc(alias = "SetGameState")]
    fn game_state(&self, state: eq::GameState) {}

    /// This is called each time MQ2 goes through its heartbeat (pulse)
    /// function.
    ///
    /// Because this happens very frequently, it is recommended to have a timer
    /// or counter at the start of this call to limit the amount of times the
    /// code in this section is executed.
    #[doc(alias = "OnPulse")]
    fn pulse(&self) {}

    /// This is called each time `WriteChatColor` is called (whether by
    /// MQ2Main or by any plugin). This can be considered the "when outputting
    /// text from MQ" callback.
    ///
    /// This ignores filters on display, so if they are needed either implement
    /// them in this section or see [`Hooks::incoming_chat()`] where filters
    /// are already handled.
    ///
    /// If `CEverQuest::dsp_chat` is not called, and events are required,
    /// they'll need to be implemented here as well. Otherwise, see
    /// [`Hooks::incoming_chat()`] where that is already handled.
    ///
    /// For a list of color values, see the [`crate::eq::ChatColor`] enum.
    #[doc(alias = "OnWriteChatColor")]
    fn write_chat(&self, line: &str, color: eq::ChatColor) {}

    /// This is called each time a line of chat is shown. It occurs after MQ
    /// filters and chat events have been handled.  If you need to know when
    /// MQ2 has sent chat, consider using [`Hooks::write_chat()`]
    /// instead.
    ///
    /// For a list of color values, see the [`crate::eq::ChatColor`] enum.
    #[doc(alias = "OnIncomingChat")]
    fn incoming_chat(&self, line: &str, color: eq::ChatColor) -> bool {
        false
    }

    /// This is called each time a spawn is added to a zone (ie, something
    /// spawns). It is also called for each existing spawn when a plugin first
    /// initializes.
    ///
    /// When zoning, this is called for all spawns in the zone after
    /// [`Hooks::end_zone()`] is called and before [`Hooks::zoned()`] is
    /// called.
    #[doc(alias = "OnAddSpawn")]
    fn add_spawn(&self, spawn: &eq::Spawn) {}

    /// This is called each time a spawn is removed from a zone (ie, something
    /// despawns or is killed). It is NOT called when a plugin shuts down.
    ///
    /// When zoning, this is called for all spawns in the zone after
    /// [`Hooks::begin_zone()`] is called.
    #[doc(alias = "OnRemoveSpawn")]
    fn remove_spawn(&self, spawn: &eq::Spawn) {}

    /// This is called each time a ground item is added to a zone (ie, something
    /// spawns). It is also called for each existing ground item when a plugin
    /// first initializes.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// [`Hooks::end_zone()`] is called and before [`Hooks::zoned()`] is
    /// called.
    #[doc(alias = "OnAddGroundItem")]
    fn add_ground_item(&self, item: &eq::GroundItem) {}

    /// This is called each time a ground item is removed from a zone (ie,
    /// something despawns or is picked up). It is NOT called when a plugin
    /// shuts down.
    ///
    /// When zoning, this is called for all ground items in the zone after
    /// [`Hooks::begin_zone()`] is called.
    #[doc(alias = "OnRemoveGroundItem")]
    fn remove_ground_item(&self, item: &eq::GroundItem) {}

    /// This is called just after entering a zone line and as the loading screen
    /// appears.
    #[doc(alias = "OnBeginZone")]
    fn begin_zone(&self) {}

    /// This is called just after the loading screen, but prior to the zone
    /// being fully loaded.
    ///
    /// This should occur before [`Hooks::add_spawn()`] and
    /// [`Hooks::add_ground_item()`] are called. It always occurs before
    /// [`Hooks::zoned()`] is called.
    #[doc(alias = "OnEndZone")]
    fn end_zone(&self) {}

    /// This is called after entering a new zone and the zone is considered
    /// "loaded."
    ///
    /// It occurs after [`Hooks::end_zone()`], [`Hooks::add_spawn()`],
    /// and [`Hooks::add_ground_item()`] have been called.
    #[doc(alias = "OnZoned")]
    fn zoned(&self) {}

    /// This is called each time that the ImGui overlay is rendered. Use this to
    /// render and update plugin specific widgets.
    ///
    /// Because this happens extremely frequently, it is recommended to move any
    /// actual work to a separate call and use this only for updating the
    /// display.
    #[doc(alias = "OnUpdateImGui")]
    fn update_imgui(&self) {}

    /// This is called each time a macro starts (ex: `/mac somemacro.mac`),
    /// prior to launching the macro.
    #[doc(alias = "OnMacroStart")]
    fn macro_start(&self, name: &str) {}

    /// This is called each time a macro stops (ex: `/endmac`), after the
    /// macro has ended.
    #[doc(alias = "OnMacroStop")]
    fn macro_stop(&self, name: &str) {}

    /// This is called each time a plugin is loaded
    /// (ex: `/plugin someplugin`), after the plugin has been loaded and any
    /// associated `-AutoExec.cfg` file have been launched.
    ///
    /// This means it will be executed after the [`Hooks::initialize()`]
    /// callback.
    ///
    /// This is also called when THIS plugin is loaded, but initialization tasks
    /// should still be done in [`Hooks::initialize()`].
    #[doc(alias = "OnLoadPlugin")]
    fn plugin_load(&self, name: &str) {}

    /// This is called each time a plugin is unloaded
    /// (ex: `/plugin someplugin unload`), just prior to the plugin unloading.
    /// This means it will be executed prior to the [`Hooks::shutdown()`]
    /// callback.
    ///
    /// This is also called when THIS plugin is unloaded, but shutdown tasks
    /// should still be done in [`Hooks::shutdown()`].
    #[doc(alias = "OnUnloadPlugin")]
    fn plugin_unload(&self, name: &str) {}
}

#[doc(hidden)]
#[allow(clippy::module_name_repetitions)]
pub struct LazyPlugin<T>(std::sync::OnceLock<T>);

impl<T> LazyPlugin<T> {
    #[must_use]
    pub const fn new() -> Self {
        LazyPlugin(std::sync::OnceLock::new())
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn set(&self, value: T) -> Result<(), T> {
        self.0.set(value)
    }

    pub fn get(&self) -> Option<&T> {
        self.0.get()
    }
}

/// TODO
#[doc(hidden)]
#[allow(clippy::module_name_repetitions)]
#[macro_export]
macro_rules! __plugin_setup {
    ($plugin_type:ident, $plugin_version:literal) => {
        // MacroQuest requires a symbol exported named this to validate that a plugin
        // was compiled for "MQNext", which is the only MacroQuest at this point in
        // time.
        #[no_mangle]
        pub static IsBuiltForNext: bool = ::macroquest::is_mq_next();

        // MacroQuest requires a symbol exported named this, that is used a stand in for
        // "version" of the EverQuest binary, which is compromised of the build date and
        // build time of the eqgame.exe.
        #[no_mangle]
        pub static EverQuestVersion: ::macroquest::EQVersion =
            ::macroquest::eq_version();

        // MacroQuest will look for this symbol, which is a pointer to a mq::MQPlugin,
        // and if it exists, will set the value of that pointer to the mq::MQPlugin
        // instance that it has created for the given plugin.
        #[no_mangle]
        pub static mut ThisPlugin: Option<&::macroquest::ffi::mq::MQPlugin> = None;

        // We have a plugin for our plugin which
        static PLUGIN: ::macroquest::plugin::LazyPlugin<$plugin_type> =
            ::macroquest::plugin::LazyPlugin::new();

        #[inline(always)]
        fn __plugin_main(
            reason: ::macroquest::plugin::Reason,
        ) -> ::std::primitive::bool {
            use ::macroquest::log::error;
            use ::macroquest::plugin::{New, Reason};

            match reason {
                Reason::Load => match PLUGIN.set($plugin_type::new()) {
                    Ok(_) => true,
                    Err(error) => {
                        error!(?error, "there was already a PLUGIN set");
                        false
                    }
                },
                Reason::Unload => true,
            }
        }

        #[no_mangle]
        extern "system" fn DllMain(
            _: *mut (),
            c_reason: ::std::primitive::u32,
            _: *mut (),
        ) -> ::std::primitive::bool {
            use ::macroquest::log::error;

            let result = ::std::panic::catch_unwind(|| {
                use ::macroquest::plugin::{MainResult, Reason};
                use ::std::convert::TryFrom;

                let rvalue = match Reason::try_from(c_reason) {
                    ::std::result::Result::Ok(reason) => {
                        Into::<MainResult>::into(__plugin_main(reason))
                    }
                    ::std::result::Result::Err(_) => {
                        error!(reason = c_reason, "unknown reason in DllMain");

                        MainResult::Bool(false)
                    }
                };

                rvalue.into()
            });

            match result {
                ::std::result::Result::Ok(r) => r,
                ::std::result::Result::Err(error) => {
                    error!(?error, hook = "PluginMain", "caught an unwind");
                    false
                }
            }
        }
    };
}

#[doc(inline)]
pub use crate::__plugin_setup as setup;
