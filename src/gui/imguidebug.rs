use imgui::{ImGuiSetCond_Always, ImGuiSetCond_FirstUseEver, Ui};

use lameboy::Lameboy;
use ppu::TestPattern;

pub struct ImguiDebug {
    pub active: bool,
    pub emulator_running: bool,
    pub show_imgui_metrics: bool,
    pub show_menu: bool,
    pub show_memory: bool,
    pub show_cpu: bool,
    pub show_vram: bool,
    pub apply_test_pattern: bool,
    pub test_pattern_type: TestPattern,
    pub ppu_mod: i32,
    pub show_about: bool,
    pub i0: i32,
}

impl ImguiDebug {
     pub fn new() -> ImguiDebug {
        ImguiDebug {
            active: true,
            emulator_running: false,
            show_imgui_metrics: false,
            show_menu: false,
            show_memory: false,
            show_cpu: false,
            show_vram: false,
            apply_test_pattern: false,
            test_pattern_type: TestPattern::BLANK,
            ppu_mod: 4,
            show_about: false,
            i0: 0,
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
                        ui.menu_item(im_str!("Memory"))
                            .selected(&mut self.show_memory)
                            .build();
                        ui.menu_item(im_str!("CPU"))
                            .selected(&mut self.show_cpu)
                            .build();
                        ui.menu_item(im_str!("vram"))
                            .selected(&mut self.show_vram)
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

        if self.show_memory {
            ui.window(im_str!("Cart"))
                .size((200.0, 125.0), ImGuiSetCond_Always)
                .resizable(false)
                .build(|| {
                    ui.text(im_str!("Size: {} bytes", emu.get_cart().get_size()));
                    ui.text(im_str!("Title: {}", emu.get_cart().get_title()));
                    ui.text(im_str!("Checksum: {}", if emu.get_cart().validate_checksum() { "VALID" } else { "INVALID" }));

                    ui.separator();

                    ui.input_int(im_str!("Addr"), &mut self.i0).build();
                    if ui.small_button(im_str!("print")) {
                        let byte = emu.get_mmu().read8(self.i0 as u16);
                        println!("Memory[{:04X}] = {:02X}", self.i0, byte);
                    }
                });
        }

        if self.show_cpu {
            let cpu = emu.get_cpu();
            ui.window(im_str!("CPU"))
                .size((260.0, 175.0), ImGuiSetCond_FirstUseEver)
                .resizable(true)
                .build(|| {
                    ui.text(im_str!("PC: 0x{:04X} - SP: 0x{:04X}", cpu.registers.pc, cpu.registers.sp));
                    ui.text(im_str!(" A: 0x{:02X}   -  B: 0x{:02X}", cpu.registers.a, cpu.registers.b));
                    ui.text(im_str!(" C: 0x{:02X}   -  D: 0x{:02X}", cpu.registers.c, cpu.registers.d));
                    ui.text(im_str!(" E: 0x{:02X}   -  F: 0x{:02X}", cpu.registers.e, cpu.registers.f.bits()));
                    ui.text(im_str!(" H: 0x{:02X}   -  L: 0x{:02X}", cpu.registers.h, cpu.registers.l));
                    ui.text(im_str!("Flags: {:?}", cpu.registers.f));
                    ui.separator();
                    ui.checkbox(im_str!("running"), &mut self.emulator_running);
                    if ui.small_button(im_str!("step")) {
                        cpu.cycle();
                    }
                });
        }

        if true {
            ui.window(im_str!("PPU"))
                .size((150.0, 130.0), ImGuiSetCond_FirstUseEver)
                .resizable(true)
                .build(|| {
                    ui.checkbox(im_str!("Apply test"), &mut self.apply_test_pattern);
                    ui.slider_int(im_str!("Mod"), &mut self.ppu_mod, 1, 20).build();
                    if ui.small_button(im_str!("Blank")) {
                        self.test_pattern_type = TestPattern::BLANK;
                    }
                    if ui.small_button(im_str!("Diagonal")) {
                        self.test_pattern_type = TestPattern::DIAGONAL;
                    }
                    if ui.small_button(im_str!("XOR")) {
                        self.test_pattern_type = TestPattern::XOR;
                    }

                    if self.apply_test_pattern {
                        emu.get_ppu().apply_test_pattern(&self.test_pattern_type, self.ppu_mod as usize);
                    }
                    // Uncomment below when custom textures get supported
    //                unsafe {
    //                    imgui_sys::igImage(ppu.get_tex_id(), ImVec2::new(160.0, 144.0), ImVec2::new(0.0, 0.0), ImVec2::new(1.0, 1.0), ImVec4::new(0.0, 0.0, 0.0, 1.0), ImVec4::new(1.0, 0.0, 0.0, 1.0));
    //                }
                });
        }
    }
}
