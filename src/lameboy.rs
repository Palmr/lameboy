use cart::Cart;
use mmu::MMU;
use cpu::CPU;
use ppu::PPU;
use joypad::Joypad;

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

    pub fn get_cpu(&mut self) -> &mut CPU<'l> {
        &mut self.cpu
    }

    pub fn get_mmu(&mut self) -> &mut MMU<'l> {
        self.get_cpu().mmu
    }

    pub fn get_cart(&mut self) -> &mut Cart {
        self.get_mmu().cart
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        self.get_mmu().ppu
    }

    pub fn get_joypad(&mut self) -> &mut Joypad {
        self.get_mmu().joypad
    }
}

use mmu::mmuobject::MmuObject;
use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiSetCond_FirstUseEver, Ui, ImGuiSelectableFlags, ImVec2};
impl<'c> ImguiDebuggable for Lameboy<'c> {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
        ui.window(im_str!("Emulator"))
            .size((200.0, 55.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {

                if ui.button(im_str!("Reset"), ImVec2::new(0.0, 0.0)) {
                    self.get_cpu().post_boot_reset();
                }
                ui.same_line(0.0);
                if ui.button(im_str!("Step"), ImVec2::new(0.0, 0.0)) {
                    self.step();
                }
                ui.same_line(0.0);
                ui.checkbox(im_str!("running"), &mut self.running);
            });
        ui.window(im_str!("Breakpoints"))
            .size((260.0, 80.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                if ui.button(im_str!("Set"), ImVec2::new(0.0, 0.0)) {
                    self.breakpoints.push(imgui_debug.input_addr as u16);
                }
                ui.same_line(0.0);
                ui.input_int(im_str!("Addr"), &mut imgui_debug.input_addr)
                    .chars_hexadecimal(true)
                    .build();

                if ui.button(im_str!("List/Remove"), ImVec2::new(0.0, 0.0)) {
                    ui.open_popup(im_str!("breakpoints"));
                }

                let mut removal_index: Option<usize> = None;
                ui.popup(im_str!("breakpoints"), || {
                    ui.text(im_str!("Breakpoints:"));
                    ui.separator();
                    if self.breakpoints.len() == 0{
                        ui.text(im_str!("None yet"));
                    }
                    else {
                        for index in 0..self.breakpoints.len() {
                            if ui.selectable(im_str!("0x{:04X}", self.breakpoints[index]), false, ImGuiSelectableFlags::empty(), ImVec2::new(0.0, 0.0)) {
                                println!("Removing index {}", index);
                                removal_index = Some(index);
                            }
                        }
                    }
                });
                match removal_index {
                    Some(index) => {self.breakpoints.remove(index);},
                    None => (),
                }
            });
        ui.window(im_str!("Joypad"))
            .size((150.0, 115.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(im_str!("A = {}", self.get_joypad().a));
                ui.text(im_str!("B = {}", self.get_joypad().b));
                ui.text(im_str!("Select = {}", self.get_joypad().select));
                ui.text(im_str!("Start = {}", self.get_joypad().start));
                self.get_joypad().write8(0xFF00, 0x10);
                ui.text(im_str!("JOYP = 0B{:04b}", self.get_joypad().read8(0xFF00)));

                ui.separator();

                ui.text(im_str!("Up = {}", self.get_joypad().up));
                ui.text(im_str!("Down = {}", self.get_joypad().down));
                ui.text(im_str!("Left = {}", self.get_joypad().left));
                ui.text(im_str!("Right = {}", self.get_joypad().right));
                self.get_joypad().write8(0xFF00, 0x20);
                ui.text(im_str!("JOYP = 0B{:04b}", self.get_joypad().read8(0xFF00)));
            });
    }
}
