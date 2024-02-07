#![allow(non_snake_case)]
#![allow(unused_variables)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

const PLUGIN_NAME: &str = env!("CARGO_PKG_NAME");

use macroquest::eq;
use macroquest::log::{ConsoleLogger, FileLogger, LevelFilter, Logger};
use macroquest::plugin::{Hooks, Plugin};

macroquest::plugin::setup!(MQRustSimple);

#[derive(Debug)]
struct MQRustSimple {}

impl Plugin for MQRustSimple {
    fn new() -> Self {
        MQRustSimple {}
    }
}

#[macroquest::plugin::hooks]
impl Hooks for MQRustSimple {
    fn initialize(&self) {
        Logger::builder()
            .console(ConsoleLogger::builder().level(LevelFilter::DEBUG).build())
            .file(
                FileLogger::builder()
                    .filename(PLUGIN_NAME)
                    .level(LevelFilter::DEBUG)
                    .build(),
            )
            .build()
            .install();
    }

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
