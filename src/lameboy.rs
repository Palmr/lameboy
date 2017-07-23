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

    pub fn test_cycles(&mut self) {
        let mut t_clk: u32 = 0;
        while t_clk < 70224 {
            t_clk += self.cpu.cycle() as u32;
        }
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
