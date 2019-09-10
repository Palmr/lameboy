use gui::imgui_debug_state::ImguiDebugState;
use gui::imgui_debuggable::ImguiDebuggable;
use imgui::{Condition, Ui, Window};
use ppu::debug::oam::oam_window;
use ppu::debug::registers::registers_window;
use ppu::PPU;

mod oam;
mod registers;

impl ImguiDebuggable for PPU {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState) {
        Window::new(im_str!("PPU"))
            .size([180.0, 115.0], Condition::FirstUseEver)
            .resizable(true)
            .build(ui, || {
                ui.text(im_str!("Mode Clock: {:?}", self.mode_clock));
                ui.text(im_str!("Mode: {:?}", self.mode));
            });

        registers_window(self, ui);
        oam_window(self, ui, imgui_debug);
    }
}
