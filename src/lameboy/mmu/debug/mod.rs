pub use crate::lameboy::mmu::debug::disassembly::disassembly_window;
pub use crate::lameboy::mmu::debug::hexdump::hexdump_window;

mod disassembly;
mod hexdump;

use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::gui::imgui_debuggable::ImguiDebuggable;
use crate::lameboy::mmu::Mmu;
use imgui::{Condition, Ui};

impl ImguiDebuggable for Mmu {
    fn imgui_display(&mut self, ui: &Ui, imgui_debug: &mut ImguiDebugState) {
        ui.window("MMU")
            .size([285.0, 122.0], Condition::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.input_int("Addr", &mut imgui_debug.input_memory_addr)
                    .chars_hexadecimal(true)
                    .build();
                ui.text(format!(
                    "[0x{:04X}] = 0x{:02x}",
                    imgui_debug.input_memory_addr,
                    self.read8(imgui_debug.input_memory_addr as u16)
                ));
                ui.separator();
                ui.input_int("Value", &mut imgui_debug.input_memory_value)
                    .chars_hexadecimal(true)
                    .build();
                if ui.small_button("Write") {
                    self.write8(
                        imgui_debug.input_memory_addr as u16,
                        imgui_debug.input_memory_value as u8,
                    );
                }
            });

        hexdump_window(self, ui, imgui_debug);
        disassembly_window(self, ui, imgui_debug);
    }
}
