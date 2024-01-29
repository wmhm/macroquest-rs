use log::{debug, trace};
use macroquest::{ChatColor, GameState, Plugin};

const LOG_TARGET: &str = "MQRustSkeleton";
const VERSION: &str = "1.0";

#[macroquest::plugin]
struct MQRustSkeleton {}

impl macroquest::Plugin for MQRustSkeleton {
    fn new() -> Self {
        MQRustSkeleton {}
    }

    fn initialize(&mut self) {
        debug!(target: LOG_TARGET, "Initializing version: {}", VERSION);
    }

    fn shutdown(&mut self) {
        debug!(target: LOG_TARGET, "Shutting down");
    }

    fn on_clean_ui(&mut self) {
        trace!(target: LOG_TARGET, "UI cleaned");
    }

    fn on_reload_ui(&mut self) {
        trace!(target: LOG_TARGET, "UI reloaded");
    }

    fn on_draw_hud(&mut self) {
        trace!(target: LOG_TARGET, "HUD drawn");
    }

    fn on_set_game_state(&mut self, state: GameState) {
        trace!(target: LOG_TARGET, "Game state updated: {:?}", state);
    }

    fn on_pulse(&mut self) {
        trace!(target: LOG_TARGET, "Pulsed");
    }

    fn on_write_chat_color(&mut self, line: &str, color: ChatColor) {
        trace!(target: LOG_TARGET, "WriteChatColor ({:?}): {}", color, line);
    }

    fn on_incoming_chat(&mut self, line: &str, color: ChatColor) -> bool {
        trace!(target: LOG_TARGET, "Chat ({:?}): {}", color, line);

        false
    }

    fn on_begin_zone(&mut self) {
        trace!(target: LOG_TARGET, "Zoning begun");
    }

    fn on_end_zone(&mut self) {
        trace!(target: LOG_TARGET, "Zoning finished");
    }

    fn on_zoned(&mut self) {
        trace!(target: LOG_TARGET, "Zoned");
    }

    fn on_update_imgui(&mut self) {
        trace!(target: LOG_TARGET, "Rendering the IgGui overlay");
    }

    fn on_macro_start(&mut self, name: &str) {
        trace!(target: LOG_TARGET, "Macro started: {}", name);
    }

    fn on_macro_stop(&mut self, name: &str) {
        trace!(target: LOG_TARGET, "Macro stopped: {}", name);
    }

    fn on_plugin_load(&mut self, name: &str) {
        trace!(target: LOG_TARGET, "Plugin loaded: {}", name);
    }

    fn on_plugin_unload(&mut self, name: &str) {
        trace!(target: LOG_TARGET, "Plugin unloaded: {}", name);
    }
}
