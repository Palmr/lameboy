use lameboy::cpu::debug::registers::registers_window;
use lameboy::cpu::debug::stack::stack_window;
use lameboy::cpu::CPU;

use gui::imgui_debug_state::ImguiDebugState;
use gui::imgui_debuggable::ImguiDebuggable;
use imgui::Ui;

mod registers;
mod stack;

impl ImguiDebuggable for CPU {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, _: &mut ImguiDebugState) {
        registers_window(self, ui);
        stack_window(self, ui);
    }
}
