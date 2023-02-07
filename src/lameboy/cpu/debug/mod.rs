use lameboy::cpu::debug::registers::registers_window;
use lameboy::cpu::debug::stack::stack_window;
use lameboy::cpu::Cpu;

use gui::imgui_debug_state::ImguiDebugState;
use gui::imgui_debuggable::ImguiDebuggable;
use imgui::Ui;

mod registers;
mod stack;

impl ImguiDebuggable for Cpu {
    fn imgui_display(&mut self, ui: &Ui, _: &mut ImguiDebugState) {
        registers_window(self, ui);
        stack_window(self, ui);
    }
}
