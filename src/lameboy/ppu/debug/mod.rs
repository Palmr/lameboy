use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::gui::imgui_debuggable::ImguiDebuggable;
use crate::lameboy::ppu::debug::oam::oam_window;
use crate::lameboy::ppu::debug::registers::registers_window;
use crate::lameboy::ppu::Ppu;
use imgui::{Condition, Ui};

mod oam;
mod registers;

impl ImguiDebuggable for Ppu {
    fn imgui_display(&mut self, ui: &Ui, imgui_debug: &mut ImguiDebugState) {
        ui.window("PPU")
            .size([180.0, 115.0], Condition::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(format!("Mode Clock: {:?}", self.mode_clock));
                ui.text(format!("Mode: {:?}", self.mode));
            });

        registers_window(self, ui);
        oam_window(self, ui, imgui_debug);
    }
}
