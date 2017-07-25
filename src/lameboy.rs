use cart::Cart;
use mmu::MMU;
use cpu::CPU;
use ppu::PPU;

pub struct Lameboy<'l> {
    cpu:  CPU<'l>,
    running: bool,
    breakpoints: Vec<u16>,
}
impl<'l> Lameboy<'l> {
    pub fn new(cpu: CPU<'l>) -> Lameboy<'l> {
        Lameboy {
            cpu: cpu,
            running: false,
            breakpoints: Vec::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn run_frame(&mut self) {
        let mut t_clk: u32 = 0;
        while t_clk < 70224 {
            // Break on breakpoints
            let current_pc = self.get_cpu().registers.pc;
            if self.breakpoints.contains(&current_pc) {
                println!("Breakpoint hit: 0x{:04X}", current_pc);
                self.running = false;
                return;
            }

            t_clk += self.step() as u32;
        }
    }

    pub fn step(&mut self) -> u8 {
        let cpu_duration = self.cpu.cycle();
        self.get_ppu().cycle(cpu_duration);

        return cpu_duration;
    }

    pub fn get_cart(&mut self) -> &mut Cart {
        self.cpu.mmu.cart
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.cpu.mmu.ppu
    }

    pub fn get_mmu(&mut self) -> &mut MMU<'l> {
        self.cpu.mmu
    }

    pub fn get_cpu(&mut self) -> &mut CPU<'l> {
        &mut self.cpu
    }
}

use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiSetCond_FirstUseEver, Ui, ImGuiSelectableFlags, ImVec2};
impl<'c> ImguiDebuggable for Lameboy<'c> {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
        ui.window(im_str!("Emulator"))
            .size((260.0, 80.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                if ui.small_button(im_str!("step")) {
                    self.step();
                }
                ui.same_line(0.0);
                ui.checkbox(im_str!("running"), &mut self.running);

                ui.separator();

                if ui.small_button(im_str!("reset")) {
                    self.get_cpu().post_boot_reset();
                }

                ui.separator();

                ui.input_int(im_str!("Addr"), &mut imgui_debug.input_addr)
                    .chars_hexadecimal(true)
                    .build();

                if ui.small_button(im_str!("Add")) {
                    self.breakpoints.push(imgui_debug.input_addr as u16);
                }
                ui.same_line(0.0);
                if ui.small_button(im_str!("List")) {
                    ui.open_popup(im_str!("breakpoints"));
                    println!("brkpt {}", self.breakpoints.len());
                }

                let mut removal_index: Option<usize> = None;
                ui.popup(im_str!("breakpoints"), || {
                        ui.text(im_str!("Breakpoints:"));
                        ui.separator();
                        for index in 0..self.breakpoints.len() {
                            if ui.selectable(im_str!("0x{:04X}", self.breakpoints[index]), false, ImGuiSelectableFlags::empty(), ImVec2::new(0.0, 0.0)) {
                                println!("Removing index {}", index);
                                removal_index = Some(index);
                            }
                        }
                    });
                match removal_index {
                    Some(index) => {self.breakpoints.remove(index);},
                    None => (),
                }
            });
    }
}
