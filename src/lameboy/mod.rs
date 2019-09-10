use cart::Cart;
use cpu::CPU;
use mmu::MMU;
use ppu::PPU;

mod debug;
pub mod joypad;

use gui::imgui_debug_state::ImguiDebugState;
use gui::GUI;
use lameboy::joypad::Joypad;

pub struct Lameboy {
    pub active: bool,
    cpu: CPU,
    running: bool,
    trace_count: i32,
    pub debug: ImguiDebugState,
}

impl Lameboy {
    pub fn new(data: Vec<u8>, gui: &GUI) -> Lameboy {
        let joypad = Joypad::new();
        let cart = Cart::new(data);
        let ppu = PPU::new(&gui.display);
        let mmu = MMU::new(cart, ppu, joypad);
        let cpu = CPU::new(mmu);

        Lameboy {
            active: true,
            cpu,
            running: false,
            trace_count: 0,
            debug: ImguiDebugState::new(),
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

    pub fn get_cpu(&mut self) -> &mut CPU {
        &mut self.cpu
    }

    pub fn get_mmu(&mut self) -> &mut MMU {
        &mut self.get_cpu().mmu
    }

    pub fn get_cart(&mut self) -> &mut Cart {
        &mut self.get_mmu().cart
    }

    pub fn get_ppu(&mut self) -> &mut PPU {
        &mut self.get_mmu().ppu
    }

    pub fn get_joypad(&mut self) -> &mut Joypad {
        &mut self.get_mmu().joypad
    }
}
