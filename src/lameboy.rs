use imgui::{Condition, ImGuiSelectableFlags, Ui};

use cart::Cart;
use cpu::CPU;
use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use joypad::Joypad;
use mmu::MMU;
use ppu::PPU;

use {PKG_AUTHORS, PKG_DESCRIPTION, PKG_NAME, PKG_VERSION};

pub struct Lameboy<'l> {
    pub active: bool,
    cpu: CPU<'l>,
    running: bool,
    trace_count: i32,
    pub debug: ImguiDebug,
}
impl<'l> Lameboy<'l> {
    pub fn new(cpu: CPU<'l>) -> Lameboy<'l> {
        Lameboy {
            active: true,
            cpu,
            running: false,
            trace_count: 0,
            debug: ImguiDebug::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn run_frame(&mut self) {
        let mut t_clk: u32 = 0;
        while t_clk < 70224 {
            // Stop emulator running if the current PC is a breakpoint
            let current_pc = self.get_cpu().registers.pc;
            if self.debug.breakpoints.contains(&current_pc) {
                debug!("Breakpoint hit: 0x{:04X}", current_pc);
                self.running = false;
                return;
            }
            let breakpoint_hit = self.get_mmu().breakpoint_hit;
            if breakpoint_hit != 0x0000 {
                debug!("Memory Breakpoint hit: 0x{:04X}", breakpoint_hit);
                self.running = false;
                self.get_mmu().breakpoint_hit = 0x0000;
                return;
            }
            self.get_mmu().memory_breakpoints = self.debug.memory_breakpoints.clone();

            // Step the emulator through a single opcode
            t_clk += u32::from(self.step());
        }
    }

    // Let the CPU fetch, decode, and execute an opcode and update the PPU
    pub fn step(&mut self) -> u8 {
        if self.trace_count > 0 {
            self.trace_count -= 1;
            trace!(
                "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X}",
                self.cpu.registers.a,
                self.cpu.registers.f.bits(),
                self.cpu.registers.b,
                self.cpu.registers.c,
                self.cpu.registers.d,
                self.cpu.registers.e,
                self.cpu.registers.h,
                self.cpu.registers.l,
                self.cpu.registers.sp,
                self.cpu.registers.pc,
            );
        }

        // Run the CPU for one opcode and get its cycle duration for the PPU
        let cpu_duration = self.cpu.cycle();

        // Run the PPU for one cycle getting any updated interrupt flags back
        let int_flags = self.get_mmu().read8(0xFF0F);
        let ppu_int_flags = self.get_ppu().cycle(cpu_duration);
        self.get_mmu().write8(0xFF0F, int_flags | ppu_int_flags);

        self.debug.program_counter = self.cpu.registers.pc;

        cpu_duration
    }

    pub fn reset(&mut self) {
        self.get_ppu().reset();
        self.get_cpu().reset();
        self.get_mmu().reset();
    }

    fn get_cpu(&mut self) -> &mut CPU<'l> {
        &mut self.cpu
    }

    fn get_mmu(&mut self) -> &mut MMU<'l> {
        self.get_cpu().mmu
    }

    fn get_cart(&mut self) -> &mut Cart {
        self.get_mmu().cart
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.get_mmu().ppu
    }

    pub fn get_joypad(&mut self) -> &mut Joypad {
        self.get_mmu().joypad
    }

    pub fn draw<'a>(&mut self, ui: &Ui<'a>) {
        if self.debug.show_menu {
            ui.main_menu_bar(|| {
                ui.menu(im_str!("File")).build(|| {
                    ui.menu_item(im_str!("Open ROM")).enabled(false).build();
                    ui.menu_item(im_str!("Reload ROM")).enabled(false).build();
                    ui.menu_item(im_str!("Reset")).enabled(false).build();
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
                        .selected(&mut self.debug.show_emulator)
                        .build();
                    ui.menu_item(im_str!("Cart"))
                        .selected(&mut self.debug.show_cart)
                        .build();
                    ui.menu_item(im_str!("Memory"))
                        .selected(&mut self.debug.show_memory)
                        .build();
                    ui.menu_item(im_str!("CPU"))
                        .selected(&mut self.debug.show_cpu)
                        .build();
                    ui.menu_item(im_str!("PPU"))
                        .selected(&mut self.debug.show_ppu)
                        .build();
                });
                ui.menu(im_str!("Help")).build(|| {
                    ui.menu_item(im_str!("About"))
                        .selected(&mut self.debug.show_about)
                        .build();
                    ui.menu_item(im_str!("ImGUI Metrics"))
                        .selected(&mut self.debug.show_imgui_metrics)
                        .build();
                });
            });
        }

        if self.debug.show_imgui_metrics {
            ui.show_metrics_window(&mut self.debug.show_imgui_metrics);
        }

        if self.debug.show_cart {
            self.cpu.mmu.cart.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_memory {
            self.cpu.mmu.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_cpu {
            self.cpu.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_ppu {
            self.cpu.mmu.ppu.imgui_display(ui, &mut self.debug);
        }
        if self.debug.apply_test_pattern {
            self.cpu
                .mmu
                .ppu
                .apply_test_pattern(&self.debug.test_pattern_type, self.debug.ppu_mod as usize);
        }

        if self.debug.show_emulator {
            ui.window(im_str!("Emulator"))
                .size([255.0, 75.0], Condition::FirstUseEver)
                .resizable(true)
                .build(|| {
                    if ui.button(im_str!("Reset"), [0.0, 0.0]) {
                        self.reset();
                    }
                    ui.same_line(0.0);
                    if ui.button(im_str!("Step"), [0.0, 0.0]) {
                        self.step();
                    }
                    ui.same_line(0.0);
                    if ui.button(im_str!("Continue"), [0.0, 0.0]) {
                        self.step();
                        self.running = true;
                    }
                    ui.same_line(0.0);
                    ui.checkbox(im_str!("running"), &mut self.running);

                    if ui.button(im_str!("Dump PC history"), [0.0, 0.0]) {
                        info!("Dumping PC history");
                        for i in 0..self.get_cpu().pc_history.len() {
                            let hp = self.get_cpu().pc_history_pointer.wrapping_add(i)
                                % self.get_cpu().pc_history.len();
                            info!("[{}] - PC = 0x{:04X}", i, self.get_cpu().pc_history[hp]);
                        }
                    }

                    ui.input_int(im_str!("Trace Count"), &mut self.trace_count)
                        .chars_decimal(true)
                        .build();
                });

            ui.window(im_str!("Breakpoints"))
                .size([225.0, 150.0], Condition::FirstUseEver)
                .resizable(true)
                .build(|| {
                    if ui.button(im_str!("Set"), [0.0, 0.0]) {
                        let breakpoint_addr = self.debug.input_breakpoint_addr as u16;
                        if !self.debug.breakpoints.contains(&breakpoint_addr) {
                            self.debug.breakpoints.push(breakpoint_addr);
                        }
                    }
                    ui.same_line(0.0);
                    ui.input_int(im_str!("Addr"), &mut self.debug.input_breakpoint_addr)
                        .chars_hexadecimal(true)
                        .build();

                    if ui.button(im_str!("Clear All"), [0.0, 0.0]) {
                        self.debug.breakpoints.clear();
                    }

                    let mut removal_index: Option<usize> = None;
                    ui.text(im_str!("Breakpoints:"));
                    ui.separator();
                    if self.debug.breakpoints.is_empty() {
                        ui.text(im_str!("None yet"));
                    } else {
                        for index in 0..self.debug.breakpoints.len() {
                            if ui.selectable(
                                &im_str!("0x{:04X}", self.debug.breakpoints[index]),
                                false,
                                ImGuiSelectableFlags::empty(),
                                [0.0, 0.0],
                            ) {
                                removal_index = Some(index);
                            }
                        }
                    }
                    if let Some(index) = removal_index {
                        self.debug.breakpoints.remove(index);
                    }
                });

            ui.window(im_str!("Memory Breakpoints"))
                .size([225.0, 150.0], Condition::FirstUseEver)
                .resizable(true)
                .build(|| {
                    if ui.button(im_str!("Set"), [0.0, 0.0]) {
                        let breakpoint_addr = self.debug.input_breakpoint_addr as u16;
                        if !self.debug.memory_breakpoints.contains(&breakpoint_addr) {
                            self.debug.memory_breakpoints.push(breakpoint_addr);
                        }
                    }
                    ui.same_line(0.0);
                    ui.input_int(im_str!("Addr"), &mut self.debug.input_breakpoint_addr)
                        .chars_hexadecimal(true)
                        .build();

                    if ui.button(im_str!("Clear All"), [0.0, 0.0]) {
                        self.debug.memory_breakpoints.clear();
                    }

                    let mut removal_index: Option<usize> = None;
                    ui.text(im_str!("Breakpoints:"));
                    ui.separator();
                    if self.debug.memory_breakpoints.is_empty() {
                        ui.text(im_str!("None yet"));
                    } else {
                        for index in 0..self.debug.memory_breakpoints.len() {
                            if ui.selectable(
                                &im_str!("0x{:04X}", self.debug.memory_breakpoints[index]),
                                false,
                                ImGuiSelectableFlags::empty(),
                                [0.0, 0.0],
                            ) {
                                removal_index = Some(index);
                            }
                        }
                    }
                    if let Some(index) = removal_index {
                        self.debug.memory_breakpoints.remove(index);
                    }
                });

            self.cpu.mmu.joypad.imgui_display(ui, &mut self.debug);
        }

        if self.debug.show_about {
            ui.window(&im_str!("About - {} v{}", PKG_NAME, PKG_VERSION))
                .size([250.0, 100.0], Condition::Always)
                .collapsible(false)
                .resizable(false)
                .build(|| {
                    ui.text(im_str!("{}", PKG_DESCRIPTION));
                    ui.text(im_str!("{}", PKG_AUTHORS));
                    if ui.button(im_str!("Close"), [75.0, 30.0]) {
                        self.debug.show_about = false;
                    }
                });
        }
    }
}
