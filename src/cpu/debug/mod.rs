use imgui::Ui;
use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use cpu::CPU;
use cpu::debug::registers::registers_window;
use cpu::debug::stack::stack_window;

mod registers;
mod stack;

impl ImguiDebuggable for CPU {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, _: &mut ImguiDebug) {
        registers_window(self, ui);
        stack_window(self, ui);
    }
}
