#![allow(non_snake_case)]
#![allow(unused_variables)]
#![warn(clippy::cargo)]
#![warn(clippy::correctness)]
#![warn(clippy::suspicious)]
#![warn(clippy::complexity)]
#![warn(clippy::perf)]
#![warn(clippy::style)]
#![warn(clippy::pedantic)]

use macroquest::eq::{ChatColor, GameState, GroundItem, Spawn};
use macroquest::log::trace;
use macroquest::plugin::Reason;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");

macroquest::plugin::preamble!();

#[macroquest::plugin::main]
fn main(reason: Reason) {
    match reason {
        Reason::Load => {
            trace!(module = PKG_NAME, "module loaded");
        }
        Reason::Unload => {
            trace!(module = PKG_NAME, "module unloaded");
        }
    };
}

#[macroquest::plugin::hook(InitializePlugin)]
fn initialize() {}

#[macroquest::plugin::hook(ShutdownPlugin)]
fn shutdown() {}

#[macroquest::plugin::hook(OnCleanUI)]
fn clean_ui() {}

#[macroquest::plugin::hook(OnReloadUI)]
fn reload_ui() {}

#[macroquest::plugin::hook(OnDrawHUD)]
fn draw_hud() {}

#[macroquest::plugin::hook(SetGameState)]
fn gamestate(state: GameState) {}

#[macroquest::plugin::hook(OnPulse)]
fn pulse() {}

#[macroquest::plugin::hook(OnWriteChatColor)]
fn write_chat(line: &str, color: ChatColor) {}

#[macroquest::plugin::hook(OnIncomingChat)]
fn incoming_chat(line: &str, color: ChatColor) -> bool {
    false
}

#[macroquest::plugin::hook(OnAddSpawn)]
fn add_spawn(spawn: &Spawn) {}

#[macroquest::plugin::hook(OnRemoveSpawn)]
fn remove_spawn(spawn: &Spawn) {}

#[macroquest::plugin::hook(OnAddGroundItem)]
fn add_ground_item(item: &GroundItem) {}

#[macroquest::plugin::hook(OnRemoveGroundItem)]
fn remove_ground_item(item: &GroundItem) {}

#[macroquest::plugin::hook(OnBeginZone)]
fn begin_zone() {}

#[macroquest::plugin::hook(OnEndZone)]
fn end_zone() {}

#[macroquest::plugin::hook(OnZoned)]
fn zoned() {}

#[macroquest::plugin::hook(OnUpdateImGui)]
fn update_imgui() {}

#[macroquest::plugin::hook(OnMacroStart)]
fn macro_start(name: &str) {}

#[macroquest::plugin::hook(OnMacroStop)]
fn macro_end(name: &str) {}

#[macroquest::plugin::hook(OnLoadPlugin)]
fn load_plugin(name: &str) {}

#[macroquest::plugin::hook(OnUnloadPlugin)]
fn unload_plugin(name: &str) {}
