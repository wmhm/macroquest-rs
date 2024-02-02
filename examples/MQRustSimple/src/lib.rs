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

    fn clean_ui(&self) {}

    fn reload_ui(&self) {}

    fn draw_hud(&self) {}

    fn pulse(&self) {}

    fn begin_zone(&self) {}

    fn end_zone(&self) {}

    fn zoned(&self) {}

    fn update_imgui(&self) {}

    fn game_state(&self, state: eq::GameState) {}

    fn write_chat(&self, line: &str, color: eq::ChatColor) {}

    fn incoming_chat(&self, line: &str, color: eq::ChatColor) -> bool {
        false
    }

    fn add_spawn(&self, spawn: &eq::Spawn) {}

    fn remove_spawn(&self, spawn: &eq::Spawn) {}

    fn add_ground_item(&self, item: &eq::GroundItem) {}

    fn remove_ground_item(&self, item: &eq::GroundItem) {}

    fn macro_start(&self, name: &str) {}

    fn macro_stop(&self, name: &str) {}

    fn plugin_load(&self, name: &str) {}

    fn plugin_unload(&self, name: &str) {}
}
