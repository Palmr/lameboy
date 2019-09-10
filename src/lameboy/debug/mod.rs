use gui::imguidebug::ImguiDebuggable;
use imgui::Ui;
use lameboy::Lameboy;

use lameboy::debug::about::about_window;
use lameboy::debug::breakpoints::breakpoint_windows;
use lameboy::debug::emulator::emulator_window;
use lameboy::debug::menu::build_menu;

mod about;
mod breakpoints;
mod emulator;
mod menu;

impl Lameboy {
    pub fn imgui_display<'a>(&mut self, ui: &Ui<'a>) {
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

        // TODO - hmmmmmm
        if self.debug.apply_test_pattern {
            self.cpu
                .mmu
                .ppu
                .apply_test_pattern(&self.debug.test_pattern_type, self.debug.ppu_mod as usize);
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
