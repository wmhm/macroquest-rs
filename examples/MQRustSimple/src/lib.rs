#![allow(non_snake_case)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use macroquest::eq;
use macroquest::log::{debug, trace};

const VERSION: &str = "1.0";

#[macroquest::plugin(logging(file))]
#[derive(Default)]
struct MQRustSimple {}

impl macroquest::Plugin for MQRustSimple {
    fn initialize(&mut self) {
        debug!(version = VERSION, "Initializing");
    }

    fn shutdown(&mut self) {
        debug!("Shutting down");
    }

    fn on_clean_ui(&mut self) {
        trace!("UI cleaned");
    }

    fn on_reload_ui(&mut self) {
        trace!("UI reloaded");
    }

    fn on_draw_hud(&mut self) {
        trace!("HUD drawn");
    }

    fn on_set_game_state(&mut self, state: eq::GameState) {
        trace!(?state, "Game state updated");
    }

    fn on_pulse(&mut self) {
        trace!("Pulsed");
    }

    fn on_write_chat_color(&mut self, line: &str, color: eq::ChatColor) {
        trace!(?color, %line, "WriteChatColor");
    }

    fn on_incoming_chat(&mut self, line: &str, color: eq::ChatColor) -> bool {
        trace!(?color, %line, "Chat");

        false
    }

    fn on_add_spawn(&mut self, spawn: &eq::Spawn) {
        trace!(?spawn, "Spawned");
    }

    fn on_remove_spawn(&mut self, spawn: &eq::Spawn) {
        trace!(?spawn, "Despawned");
    }

    fn on_add_ground_item(&mut self, item: &eq::GroundItem) {
        trace!(?item, "Ground item spawned");
    }

    fn on_remove_ground_item(&mut self, item: &eq::GroundItem) {
        trace!(?item, "Ground item despawned");
    }

    fn on_begin_zone(&mut self) {
        trace!("Zoning started");
    }

    fn on_end_zone(&mut self) {
        trace!("Zoning finished");
    }

    fn on_zoned(&mut self) {
        trace!("Zoned");
    }

    fn on_update_imgui(&mut self) {
        trace!("Rendering the IgGui overlay");
    }

    fn on_macro_start(&mut self, name: &str) {
        trace!(%name, "Macro started");
    }

    fn on_macro_stop(&mut self, name: &str) {
        trace!(%name, "Macro stopped");
    }

    fn on_plugin_load(&mut self, name: &str) {
        trace!(%name, "Plugin loaded");
    }

    fn on_plugin_unload(&mut self, name: &str) {
        trace!(%name, "Plugin unloaded");
    }
}
