use imgui::{ImGuiCond, ImVec2, Ui};

use lameboy::Lameboy;
use ppu::TestPattern;

use {PKG_AUTHORS, PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub trait ImguiDebuggable {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug);
}

pub struct ImguiDebug {
    pub active: bool,
    pub show_imgui_metrics: bool,
    pub show_menu: bool,
    pub show_emulator: bool,
    pub show_cart: bool,
    pub show_memory: bool,
    pub show_cpu: bool,
    pub show_ppu: bool,
    pub apply_test_pattern: bool,
    pub test_pattern_type: TestPattern,
    pub ppu_mod: i32,
    pub ppu_sprite_index: i32,
    pub show_about: bool,
    pub input_breakpoint_addr: i32,
    pub input_memory_addr: i32,
    pub input_memory_value: i32,
    pub dump_memory_addr: i32,
    pub dump_memory_pc_lock: bool,
}

impl ImguiDebug {
    pub fn new() -> ImguiDebug {
        ImguiDebug {
            active: true,
            show_imgui_metrics: false,
            show_menu: false,
            show_emulator: true,
            show_cart: false,
            show_memory: false,
            show_cpu: false,
            show_ppu: false,
            apply_test_pattern: false,
            test_pattern_type: TestPattern::BLANK,
            ppu_mod: 4,
            ppu_sprite_index: 0,
            show_about: false,
            input_breakpoint_addr: 0,
            input_memory_addr: 0,
            input_memory_value: 0,
            dump_memory_addr: 0,
            dump_memory_pc_lock: false,
        }
    }

    pub fn draw<'a>(&mut self, ui: &Ui<'a>, lameboy: &mut Lameboy) {
        if self.show_menu {
            ui.main_menu_bar(|| {
                ui.menu(im_str!("File")).build(|| {
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
                ui.menu(im_str!("Options")).build(|| {
                    ui.menu_item(im_str!("TODO")).enabled(false).build();
                });
                ui.menu(im_str!("Debug")).build(|| {
                    ui.menu_item(im_str!("Emulator"))
                        .selected(&mut self.show_emulator)
                        .build();
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
                ui.menu(im_str!("Help")).build(|| {
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
            lameboy.get_cart().imgui_display(ui, self);
        }

        if self.show_memory {
            lameboy.get_mmu().imgui_display(ui, self);
        }

        if self.show_cpu {
            lameboy.get_cpu().imgui_display(ui, self);
        }

        if self.show_ppu {
            lameboy.get_ppu().imgui_display(ui, self);
        }
        if self.apply_test_pattern {
            lameboy
                .get_ppu()
                .apply_test_pattern(&self.test_pattern_type, self.ppu_mod as usize);
        }

        if self.show_emulator {
            lameboy.imgui_display(ui, self);
        }

        if self.show_about {
            ui.window(im_str!("About - {} v{}", PKG_NAME, PKG_VERSION))
                .size((250.0, 100.0), ImGuiCond::Always)
                .collapsible(false)
                .resizable(false)
                .movable(false)
                .build(|| {
                    ui.text(im_str!("{}", PKG_DESCRIPTION));
                    ui.text(im_str!("{}", PKG_AUTHORS));
                    if ui.button(im_str!("Close"), ImVec2::new(75.0, 30.0)) {
                        self.show_about = false;
                    }
                });
        }
    }
}
