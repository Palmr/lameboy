use imgui::{MenuItem, Ui};
use lameboy::Lameboy;

pub fn build_menu<'a>(lameboy: &mut Lameboy, ui: &Ui<'a>) {
    if let Some(menu_bar) = ui.begin_main_menu_bar() {
        if let Some(menu) = ui.begin_menu(im_str!("File"), true) {
            MenuItem::new(im_str!("Open ROM"))
                .shortcut(im_str!("CTRL+O"))
                .build(ui);
            MenuItem::new(im_str!("Reset")).build(ui);
            ui.separator();
            MenuItem::new(im_str!("Exit")).build_with_ref(ui, &mut lameboy.active);

            menu.end(ui);
        }

        if let Some(menu) = ui.begin_menu(im_str!("Debug"), true) {
            MenuItem::new(im_str!("Emulator")).build_with_ref(ui, &mut lameboy.debug.show_emulator);
            MenuItem::new(im_str!("Memory")).build_with_ref(ui, &mut lameboy.debug.show_memory);
            MenuItem::new(im_str!("CPU")).build_with_ref(ui, &mut lameboy.debug.show_cpu);
            MenuItem::new(im_str!("PPU")).build_with_ref(ui, &mut lameboy.debug.show_ppu);
            MenuItem::new(im_str!("Cart")).build_with_ref(ui, &mut lameboy.debug.show_cart);
            MenuItem::new(im_str!("Joypad")).build_with_ref(ui, &mut lameboy.debug.show_joypad);

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
