use crate::lameboy::cpu::debug::registers::registers_window;
use crate::lameboy::cpu::debug::stack::stack_window;
use crate::lameboy::cpu::Cpu;

use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::gui::imgui_debuggable::ImguiDebuggable;
use imgui::Ui;

mod registers;
mod stack;

impl ImguiDebuggable for Cpu {
    fn imgui_display(&mut self, ui: &Ui, _: &mut ImguiDebugState) {
        registers_window(self, ui);
        stack_window(self, ui);
    }
}
