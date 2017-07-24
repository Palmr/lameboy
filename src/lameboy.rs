use cart::Cart;
use mmu::MMU;
use cpu::CPU;
use ppu::PPU;

pub struct Lameboy<'l> {
    cpu:  CPU<'l>,
}
impl<'l> Lameboy<'l> {
    pub fn new(cpu: CPU<'l>) -> Lameboy<'l> {
        Lameboy {
            cpu: cpu,
        }
    }

    pub fn run_frame(&mut self) {
        let mut t_clk: u32 = 0;
        while t_clk < 70224 {
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
use imgui::{ImGuiSetCond_FirstUseEver, Ui};
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
                ui.checkbox(im_str!("running"), &mut imgui_debug.emulator_running);

                ui.separator();

                if ui.small_button(im_str!("reset")) {
                    self.get_cpu().post_boot_reset();
                }
            });
    }
}
