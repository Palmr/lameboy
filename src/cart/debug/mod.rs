use cart::Cart;
use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{Condition, Ui, Window};

impl ImguiDebuggable for Cart {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, _: &mut ImguiDebug) {
        Window::new(im_str!("Cart"))
            .size([180.0, 127.0], Condition::FirstUseEver)
            .resizable(true)
            .build(ui, || {
                ui.text(im_str!("Size: {} bytes", self.get_size()));
                ui.text(im_str!("Title: {}", self.get_title()));
                ui.text(im_str!("Type: {}", self.get_cart_type()));
                ui.text(im_str!("ROM Size: {}", self.get_rom_size()));
                ui.text(im_str!("RAM Size: {}", self.get_ram_size()));
                ui.text(im_str!(
                    "Checksum: {}",
                    if self.validate_checksum() {
                        "VALID"
                    } else {
                        "INVALID"
                    }
                ));
            });
    }
}
