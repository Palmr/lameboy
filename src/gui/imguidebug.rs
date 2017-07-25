use imgui::Ui;

use lameboy::Lameboy;
use ppu::TestPattern;

pub trait ImguiDebuggable {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug);
}

pub struct ImguiDebug {
    pub active: bool,
    pub show_imgui_metrics: bool,
    pub show_menu: bool,
    pub show_cart: bool,
    pub show_memory: bool,
    pub show_cpu: bool,
    pub show_ppu: bool,
    pub apply_test_pattern: bool,
    pub test_pattern_type: TestPattern,
    pub ppu_mod: i32,
    pub show_about: bool,
    pub input_addr: i32,
    pub input_d8: i32,
}

impl ImguiDebug {
     pub fn new() -> ImguiDebug {
        ImguiDebug {
            active: true,
            show_imgui_metrics: false,
            show_menu: false,
            show_cart: false,
            show_memory: false,
            show_cpu: false,
            show_ppu: false,
            apply_test_pattern: false,
            test_pattern_type: TestPattern::BLANK,
            ppu_mod: 4,
            show_about: false,
            input_addr: 0,
            input_d8: 0,
        }
    }

    pub fn draw<'a>(&mut self, ui: &Ui<'a>, mut emu: &mut Lameboy) {
        if self.show_menu {
            ui.main_menu_bar(|| {
                ui.menu(im_str!("File"))
                    .build(|| {
                        ui.menu_item(im_str!("Open ROM"))
                            .selected(&mut self.show_memory)
                            .build();
                        ui.menu_item(im_str!("Reload ROM"))
                            .selected(&mut self.show_memory)
                            .build();
                        ui.menu_item(im_str!("Reset"))
                            .selected(&mut self.show_memory)
                            .build();
                        ui.separator();
                        ui.menu_item(im_str!("Exit"))
                            .selected(&mut self.active)
                            .build();
                    });
                ui.menu(im_str!("Options"))
                    .build(|| {});
                ui.menu(im_str!("Debug"))
                    .build(|| {
                        ui.menu_item(im_str!("Cart"))
                            .selected(&mut self.show_cart)
                            .build();
                        ui.menu_item(im_str!("Memory"))
                            .selected(&mut self.show_memory)
                            .build();
                        ui.menu_item(im_str!("CPU"))
                            .selected(&mut self.show_cpu)
                            .build();
                        ui.menu_item(im_str!("PPU"))
                            .selected(&mut self.show_ppu)
                            .build();
                    });
                ui.menu(im_str!("Help"))
                    .build(|| {
                        ui.menu_item(im_str!("About"))
                            .selected(&mut self.show_about)
                            .build();
                        ui.menu_item(im_str!("ImGUI Metrics"))
                            .selected(&mut self.show_imgui_metrics)
                            .build();
                    });
            });
        }

        if self.show_imgui_metrics {
            ui.show_metrics_window(&mut self.show_imgui_metrics);
        }

        if self.show_cart {
            emu.get_cart().imgui_display(ui, self);
        }

        if self.show_memory {
            emu.get_mmu().imgui_display(ui, self);
        }

        if self.show_cpu {
            emu.get_cpu().imgui_display(ui, self);
        }

        if self.show_ppu {
            emu.get_ppu().imgui_display(ui, self);
        }
        if self.apply_test_pattern {
            emu.get_ppu().apply_test_pattern(&self.test_pattern_type, self.ppu_mod as usize);
        }

        emu.imgui_display(ui, self);
    }
}
