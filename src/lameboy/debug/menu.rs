use crate::lameboy::Lameboy;
use imgui::Ui;

pub fn build_menu(lameboy: &mut Lameboy, ui: &Ui) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu("File") {
            ui.menu_item("Open ROM");
            ui.menu_item("Reset");
            ui.separator();

            lameboy.active = !ui.menu_item("Exit");

            menu.end()
        }

        if let Some(menu) = ui.begin_menu("Debug") {
            ui.checkbox("Emulator", &mut lameboy.debug.show_emulator);
            ui.checkbox("Memory", &mut lameboy.debug.show_memory);
            ui.checkbox("CPU", &mut lameboy.debug.show_cpu);
            ui.checkbox("PPU", &mut lameboy.debug.show_ppu);
            ui.checkbox("Cart", &mut lameboy.debug.show_cart);
            ui.checkbox("Joypad", &mut lameboy.debug.show_joypad);

            menu.end();
        }

        if let Some(menu) = ui.begin_menu("Help") {
            ui.menu_item_config("About")
                .build_with_ref(&mut lameboy.debug.show_about);
            ui.menu_item_config("ImGUI Metrics")
                .build_with_ref(&mut lameboy.debug.show_imgui_metrics);

            menu.end();
        }

        menu_bar.end();
    }
}
