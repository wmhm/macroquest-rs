#![allow(non_snake_case)]
#![allow(unused_variables)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use macroquest::eq;
use macroquest::plugin::Plugin;
#[macroquest::plugin::create]
#[derive(Debug, Default)]
struct MQRustSimple {}

#[macroquest::plugin::hooks]
impl Plugin for MQRustSimple {
    fn initialize(&self) {}

    fn shutdown(&self) {}

    fn on_clean_ui(&self) {}

    fn on_reload_ui(&self) {}

    fn on_draw_hud(&self) {}

    fn on_pulse(&self) {}

    fn on_begin_zone(&self) {}

    fn on_end_zone(&self) {}

    fn on_zoned(&self) {}

    fn on_update_imgui(&self) {}

    fn on_set_game_state(&self, state: eq::GameState) {}

    fn on_write_chat_color(&self, line: &str, color: eq::ChatColor) {}

    fn on_incoming_chat(&self, line: &str, color: eq::ChatColor) -> bool {
        false
    }

    fn on_add_spawn(&self, spawn: &eq::Spawn) {}

    fn on_remove_spawn(&self, spawn: &eq::Spawn) {}

    fn on_add_ground_item(&self, item: &eq::GroundItem) {}

    fn on_remove_ground_item(&self, item: &eq::GroundItem) {}

    fn on_macro_start(&self, name: &str) {}

    fn on_macro_stop(&self, name: &str) {}

    fn on_plugin_load(&self, name: &str) {}

    fn on_plugin_unload(&self, name: &str) {}
}
