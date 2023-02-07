use crate::gui::imgui_debuggable::ImguiDebuggable;
use crate::lameboy::Lameboy;
use imgui::Ui;

use crate::lameboy::debug::about::about_window;
use crate::lameboy::debug::breakpoints::breakpoint_windows;
use crate::lameboy::debug::emulator::emulator_window;
use crate::lameboy::debug::menu::build_menu;

mod about;
mod breakpoints;
mod emulator;
mod menu;

impl Lameboy {
    pub fn imgui_display(&mut self, ui: &Ui) {
        if self.debug.show_menu {
            build_menu(self, ui);
        }

        if self.debug.show_imgui_metrics {
            ui.show_metrics_window(&mut self.debug.show_imgui_metrics);
        }

        if self.debug.show_cart {
            self.cpu.mmu.cart.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_joypad {
            self.cpu.mmu.joypad.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_memory {
            self.cpu.mmu.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_cpu {
            self.cpu.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_ppu {
            self.cpu.mmu.ppu.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_emulator {
            emulator_window(self, ui);
            breakpoint_windows(ui, &mut self.debug);
        }

        if self.debug.show_about {
            about_window(ui, &mut self.debug);
        }
    }
}
