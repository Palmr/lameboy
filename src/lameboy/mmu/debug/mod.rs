pub use lameboy::mmu::debug::disassembly::disassembly_window;
pub use lameboy::mmu::debug::hexdump::hexdump_window;

mod disassembly;
mod hexdump;

use gui::imgui_debug_state::ImguiDebugState;
use gui::imgui_debuggable::ImguiDebuggable;
use imgui::{Condition, Ui, Window};
use lameboy::mmu::MMU;

impl ImguiDebuggable for MMU {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState) {
        Window::new(im_str!("MMU"))
            .size([285.0, 122.0], Condition::FirstUseEver)
            .resizable(true)
            .build(ui, || {
                ui.input_int(im_str!("Addr"), &mut imgui_debug.input_memory_addr)
                    .chars_hexadecimal(true)
                    .build();
                ui.text(im_str!(
                    "[0x{:04X}] = 0x{:02x}",
                    imgui_debug.input_memory_addr,
                    self.read8(imgui_debug.input_memory_addr as u16)
                ));
                ui.separator();
                ui.input_int(im_str!("Value"), &mut imgui_debug.input_memory_value)
                    .chars_hexadecimal(true)
                    .build();
                if ui.small_button(im_str!("Write")) {
                    self.write8(
                        imgui_debug.input_memory_addr as u16,
                        imgui_debug.input_memory_value as u8,
                    );
                }
            });

        hexdump_window(&self, ui, imgui_debug);
        disassembly_window(&self, ui, imgui_debug);
    }
}
