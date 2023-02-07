use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::gui::imgui_debuggable::ImguiDebuggable;
use crate::lameboy::cart::Cart;
use imgui::{Condition, Ui};

impl ImguiDebuggable for Cart {
    fn imgui_display(&mut self, ui: &Ui, _: &mut ImguiDebugState) {
        ui.window("Cart")
            .size([180.0, 127.0], Condition::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(format!("Title: {}", self.title));
                ui.text(format!("Type: {}", self.cart_type));
                ui.text(format!("ROM Size: {}", self.rom_size));
                ui.text(format!("RAM Size: {}", self.ram_size));
                ui.text(format!(
                    "Checksum: {}",
                    if self.valid_checksum {
                        "VALID"
                    } else {
                        "INVALID"
                    }
                ));
            });
    }
}
