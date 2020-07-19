use gui::imgui_debug_state::ImguiDebugState;
use gui::imgui_debuggable::ImguiDebuggable;
use imgui::{Condition, Ui, Window};
use lameboy::cart::Cart;

impl ImguiDebuggable for Cart {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, _: &mut ImguiDebugState) {
        Window::new(im_str!("Cart"))
            .size([180.0, 127.0], Condition::FirstUseEver)
            .resizable(true)
            .build(ui, || {
                ui.text(im_str!("Title: {}", self.title));
                ui.text(im_str!("Type: {}", self.cart_type));
                ui.text(im_str!("ROM Size: {}", self.rom_size));
                ui.text(im_str!("RAM Size: {}", self.ram_size));
                ui.text(im_str!(
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
