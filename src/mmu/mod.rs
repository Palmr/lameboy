use cart::Cart;
use joypad::Joypad;
use ppu::PPU;

pub mod mmuobject;
use mmu::mmuobject::MmuObject;

pub struct MMU<'m> {
    pub cart: &'m mut Cart,
    pub ppu: &'m mut PPU,
    pub joypad: &'m mut Joypad,
    /// Work RAM 0 [0xC000 - 0xCFFF]
    wram0: Box<[u8; 0x1000]>,
    /// Work RAM 1 [0xD000 - 0xDFFF] (Bank 1-7 in CGB Mode)
    wram1: Box<[u8; 0x1000]>,
    /// Unusable region [0xFEA0 - 0xFEFF]
    unusable: u8,
    /// I/O Ports [FF00 - 0xFF7F]
    io: Box<[u8; 0x0080]>,
    /// High RAM [0xFF80 - 0xFFFE]
    hram: Box<[u8; 0x007F]>,
    /// Interrupt Enable Register [0xFFFF]
    ier: u8,
    pub memory_breakpoints: Vec<u16>,
    pub breakpoint_hit: u16,
}

impl<'m> MMU<'m> {
    pub fn new(cart: &'m mut Cart, ppu: &'m mut PPU, joypad: &'m mut Joypad) -> MMU<'m> {
        MMU {
            cart,
            ppu,
            joypad,
            wram0: Box::new([0; 0x1000]),
            wram1: Box::new([0; 0x1000]),
            unusable: 0xFF,
            io: Box::new([0; 0x0080]),
            hram: Box::new([0; 0x007F]),
            ier: 0x00,
            memory_breakpoints: Vec::new(),
            breakpoint_hit: 0x0000,
        }
    }

    pub fn reset(&mut self) {
        self.write8(0xFF05, 0x00);
        self.write8(0xFF06, 0x00);
        self.write8(0xFF07, 0x00);
        self.write8(0xFF10, 0x80);
        self.write8(0xFF11, 0xBF);
        self.write8(0xFF12, 0xF3);
        self.write8(0xFF14, 0xBF);
        self.write8(0xFF16, 0x3F);
        self.write8(0xFF17, 0x00);
        self.write8(0xFF19, 0xBF);
        self.write8(0xFF1A, 0x7F);
        self.write8(0xFF1B, 0xFF);
        self.write8(0xFF1C, 0x9F);
        self.write8(0xFF1E, 0xBF);
        self.write8(0xFF20, 0xFF);
        self.write8(0xFF21, 0x00);
        self.write8(0xFF22, 0x00);
        self.write8(0xFF23, 0xBF);
        self.write8(0xFF24, 0x77);
        self.write8(0xFF25, 0xF3);
        self.write8(0xFF26, 0xF1);
        self.write8(0xFF40, 0x91);
        self.write8(0xFF42, 0x00);
        self.write8(0xFF43, 0x00);
        self.write8(0xFF45, 0x00);
        self.write8(0xFF47, 0xFC);
        self.write8(0xFF48, 0xFF);
        self.write8(0xFF49, 0xFF);
        self.write8(0xFF4A, 0x00);
        self.write8(0xFF4B, 0x00);
        self.write8(0xFFFF, 0x00);
    }

    pub fn read8(&mut self, addr: u16) -> u8 {
        if self.memory_breakpoints.contains(&addr) {
            self.breakpoint_hit = addr;
        }

        match addr {
            0x0000...0x7FFF | 0xA000...0xBFFF => self.cart.read8(addr),
            0x8000...0x9FFF => {
                // Return undefined data if accessing VRAM
                if self.ppu.is_vram_accessible() {
                    self.ppu.read8(addr)
                } else {
                    0xFF
                }
            }
            0xC000...0xCFFF | 0xE000...0xEFFF => self.wram0[(addr as usize) & 0x0FFF],
            0xD000...0xDFFF | 0xF000...0xFDFF => self.wram1[(addr as usize) & 0x0FFF],
            0xFE00...0xFE9F => {
                // Return undefined data if accessing VRAM or OAM
                if self.ppu.is_oam_accessible() {
                    self.ppu.read8(addr)
                } else {
                    0xFF
                }
            }
            0xFEA0...0xFEFF => self.unusable,
            0xFF00...0xFF7F => match addr {
                0xFF00 => self.joypad.read8(addr),
                0xFF40...0xFF4B => self.ppu.read8(addr),
                0xFF01...0xFF3F | 0xFF4C...0xFF7F => self.io[(addr as usize) & 0x00FF],
                _ => panic!(
                    "Attempted to access [RD] memory from an invalid address: {:#X}",
                    addr
                ),
            },
            0xFF80...0xFFFE => self.hram[((addr as usize) & 0x00FF) - 0x0080],
            0xFFFF => self.ier,
        }
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        if self.memory_breakpoints.contains(&addr) {
            self.breakpoint_hit = addr;
        }

        match addr {
            0x0000...0x7FFF | 0xA000...0xBFFF => self.cart.write8(addr, data),
            0x8000...0x9FFF => {
                // Ignore update if PPU is accessing VRAM
                if self.ppu.is_vram_accessible() {
                    self.ppu.write8(addr, data)
                }
            }
            0xC000...0xCFFF | 0xE000...0xEFFF => self.wram0[(addr as usize) & 0x0FFF] = data,
            0xD000...0xDFFF | 0xF000...0xFDFF => self.wram1[(addr as usize) & 0x0FFF] = data,
            0xFE00...0xFE9F => {
                // Ignore update if PPU is accessing VRAM or OAM
                if self.ppu.is_oam_accessible() {
                    self.ppu.write8(addr, data)
                }
            }
            0xFEA0...0xFEFF => (),
            0xFF00...0xFF7F => {
                match addr {
                    0xFF00 => self.joypad.write8(addr, data),
                    0xFF46 => {
                        // DMA
                        let source_addr = (u16::from(data)) << 8;
                        for i in 0..160 {
                            let val = self.read8(source_addr + i);
                            self.write8(0xFE00 + i, val);
                        }
                    }
                    0xFF40...0xFF45 | 0xFF47...0xFF4B => self.ppu.write8(addr, data),
                    0xFF01...0xFF3F | 0xFF4C...0xFF7F => self.io[(addr as usize) & 0x00FF] = data,
                    _ => panic!(
                        "Attempted to access [WR] memory from an invalid address: {:#X}",
                        addr
                    ),
                }
            }
            0xFF80...0xFFFE => self.hram[((addr as usize) & 0x00FF) - 0x0080] = data,
            0xFFFF => self.ier = data,
        }
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr.wrapping_add(1));

        ((u16::from(high)) << 8) | (u16::from(low))
    }
}

use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiCond, Ui};
impl<'m> ImguiDebuggable for MMU<'m> {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
        ui.window(im_str!("MMU"))
            .size((285.0, 122.0), ImGuiCond::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.input_int(im_str!("Addr"), &mut imgui_debug.input_memory_addr)
                    .chars_hexadecimal(true)
                    .build();
                ui.text(im_str!(
                    "[0x{:04X}] = 0x{:02x}",
                    imgui_debug.input_memory_addr,
                    self.read8(imgui_debug.input_memory_addr as u16)
                ));
                ui.separator();
                ui.input_int(im_str!("Value"), &mut imgui_debug.input_memory_value)
                    .chars_hexadecimal(true)
                    .build();
                if ui.small_button(im_str!("Write")) {
                    self.write8(
                        imgui_debug.input_memory_addr as u16,
                        imgui_debug.input_memory_value as u8,
                    );
                }
            });
        ui.window(im_str!("MMU - dump"))
            .size((260.0, 140.0), ImGuiCond::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.input_int(im_str!("Addr"), &mut imgui_debug.dump_memory_addr)
                    .chars_hexadecimal(true)
                    .build();
                ui.same_line(0.0);
                ui.checkbox(im_str!("Lock to PC"), &mut imgui_debug.dump_memory_pc_lock);
                ui.separator();

                let bytes_per_row = 16;
                let context_size = 5;

                let dump_memory_addr: u16 = imgui_debug.dump_memory_addr as u16;
                let memory_addr_row = dump_memory_addr - (dump_memory_addr % bytes_per_row);

                let mut memory_addr_low =
                    memory_addr_row.wrapping_sub(context_size * bytes_per_row);
                let memory_addr_high = memory_addr_row.wrapping_add(context_size * bytes_per_row);

                if memory_addr_low > memory_addr_high {
                    memory_addr_low = 0;
                }

                for row in 0..(context_size * 2) {
                    let row_addr = memory_addr_low + row * bytes_per_row;

                    ui.text_colored((0.7, 0.7, 0.7, 1.0), im_str!("[0x{:04X}]", row_addr));

                    for offset in 0..bytes_per_row {
                        let colour;
                        let mem_ptr = row_addr + offset;
                        if mem_ptr == dump_memory_addr {
                            colour = (0.5, 1.0, 0.5, 1.0);
                        } else {
                            colour = (0.8, 0.8, 0.8, 1.0);
                        }

                        ui.same_line(0.0);
                        ui.text_colored(colour, im_str!("{:02X}", self.read8(mem_ptr)));
                    }
                }
            });
    }
}
