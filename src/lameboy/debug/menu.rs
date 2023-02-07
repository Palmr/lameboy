use imgui::{MenuItem, Ui};
use lameboy::Lameboy;

pub fn build_menu(lameboy: &mut Lameboy, ui: &Ui) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            MenuItem::new(im_str!("Open ROM")).build(ui);
            MenuItem::new(im_str!("Reset")).build(ui);
            ui.separator();

            lameboy.active = !MenuItem::new(im_str!("Exit")).build(ui);

            menu.end(ui);
        }

        if let Some(menu) = ui.begin_menu(im_str!("Debug"), true) {
            ui.checkbox(im_str!("Emulator"), &mut lameboy.debug.show_emulator);
            ui.checkbox(im_str!("Memory"), &mut lameboy.debug.show_memory);
            ui.checkbox(im_str!("CPU"), &mut lameboy.debug.show_cpu);
            ui.checkbox(im_str!("PPU"), &mut lameboy.debug.show_ppu);
            ui.checkbox(im_str!("Cart"), &mut lameboy.debug.show_cart);
            ui.checkbox(im_str!("Joypad"), &mut lameboy.debug.show_joypad);

            menu.end(ui);
        }

        if let Some(menu) = ui.begin_menu(im_str!("Help"), true) {
            MenuItem::new(im_str!("About")).build_with_ref(ui, &mut lameboy.debug.show_about);
            MenuItem::new(im_str!("ImGUI Metrics"))
                .build_with_ref(ui, &mut lameboy.debug.show_imgui_metrics);

            menu.end(ui);
        }

        menu_bar.end(ui);
    }
}
