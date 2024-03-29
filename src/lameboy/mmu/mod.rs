use crate::lameboy::cart::Cart;
use crate::lameboy::joypad::Joypad;
use crate::lameboy::mmu::mmuobject::MmuObject;
use crate::lameboy::ppu::Ppu;

pub mod mmuobject;

mod debug;

pub(crate) const CART_ROM_BANK_0_START: u16 = 0x0000;
pub(crate) const CART_ROM_BANK_0_END: u16 = 0x3FFF;
pub(crate) const CART_ROM_BANK_X_START: u16 = 0x4000;
pub(crate) const CART_ROM_BANK_X_END: u16 = 0x7FFF;
pub(crate) const VRAM_START: u16 = 0x8000;
pub(crate) const VRAM_END: u16 = 0x9FFF;
pub(crate) const CART_RAM_BANK_X_START: u16 = 0xA000;
pub(crate) const CART_RAM_BANK_X_END: u16 = 0xBFFF;
pub(crate) const RAM_BANK_0_START: u16 = 0xC000;
pub(crate) const RAM_BANK_0_END: u16 = 0xCFFF;
pub(crate) const RAM_BANK_X_START: u16 = 0xD000;
pub(crate) const RAM_BANK_X_END: u16 = 0xDFFF;
pub(crate) const RAM_ECHO_BANK_0_START: u16 = 0xE000;
pub(crate) const RAM_ECHO_BANK_0_END: u16 = 0xEFFF;
pub(crate) const RAM_ECHO_BANK_X_START: u16 = 0xF000;
pub(crate) const RAM_ECHO_BANK_X_END: u16 = 0xFDFF;
pub(crate) const OAM_START: u16 = 0xFE00;
pub(crate) const OAM_END: u16 = 0xFE9F;
pub(crate) const UNUSABLE_START: u16 = 0xFEA0;
pub(crate) const UNUSABLE_END: u16 = 0xFEFF;
pub(crate) const IO_PORTS_START: u16 = 0xFF00;
pub(crate) const IO_PORTS_END: u16 = 0xFF7F;
pub(crate) const HIGH_RAM_START: u16 = 0xFF80;
pub(crate) const HIGH_RAM_END: u16 = 0xFFFE;
pub(crate) const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

pub struct Mmu {
    pub cart: Cart,
    pub ppu: Ppu,
    pub joypad: Joypad,
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

impl Mmu {
    pub fn new(cart: Cart, ppu: Ppu, joypad: Joypad) -> Mmu {
        Mmu {
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

        self.read8_safe(addr)
    }

    pub fn read8_safe(&self, addr: u16) -> u8 {
        match addr {
            CART_ROM_BANK_0_START..=CART_ROM_BANK_0_END
            | CART_ROM_BANK_X_START..=CART_ROM_BANK_X_END
            | CART_RAM_BANK_X_START..=CART_RAM_BANK_X_END => self.cart.read8(addr),
            VRAM_START..=VRAM_END => {
                // Return undefined data if accessing VRAM
                if self.ppu.is_vram_accessible() {
                    self.ppu.read8(addr)
                } else {
                    0xFF
                }
            }
            RAM_BANK_0_START..=RAM_BANK_0_END | RAM_ECHO_BANK_0_START..=RAM_ECHO_BANK_0_END => {
                self.wram0[(addr as usize) & 0x0FFF]
            }
            RAM_BANK_X_START..=RAM_BANK_X_END | RAM_ECHO_BANK_X_START..=RAM_ECHO_BANK_X_END => {
                self.wram1[(addr as usize) & 0x0FFF]
            }
            OAM_START..=OAM_END => {
                // Return undefined data if accessing VRAM or OAM
                if self.ppu.is_oam_accessible() {
                    self.ppu.read8(addr)
                } else {
                    0xFF
                }
            }
            UNUSABLE_START..=UNUSABLE_END => self.unusable,
            IO_PORTS_START..=IO_PORTS_END => match addr {
                0xFF00 => self.joypad.read8(addr),
                0xFF40..=0xFF4B => self.ppu.read8(addr),
                0xFF01..=0xFF3F | 0xFF4C..=0xFF7F => self.io[(addr as usize) & 0x00FF],
                _ => panic!("Attempted to access [RD] memory from an invalid address: {addr:#X}"),
            },
            HIGH_RAM_START..=HIGH_RAM_END => self.hram[((addr as usize) & 0x00FF) - 0x0080],
            INTERRUPT_ENABLE_REGISTER => self.ier,
        }
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        if self.memory_breakpoints.contains(&addr) {
            self.breakpoint_hit = addr;
        }

        match addr {
            CART_ROM_BANK_0_START..=CART_ROM_BANK_0_END
            | CART_ROM_BANK_X_START..=CART_ROM_BANK_X_END
            | CART_RAM_BANK_X_START..=CART_RAM_BANK_X_END => self.cart.write8(addr, data),
            VRAM_START..=VRAM_END => {
                // Ignore update if PPU is accessing VRAM
                if self.ppu.is_vram_accessible() {
                    self.ppu.write8(addr, data)
                }
            }
            RAM_BANK_0_START..=RAM_BANK_0_END | RAM_ECHO_BANK_0_START..=RAM_ECHO_BANK_0_END => {
                self.wram0[(addr as usize) & 0x0FFF] = data
            }
            RAM_BANK_X_START..=RAM_BANK_X_END | RAM_ECHO_BANK_X_START..=RAM_ECHO_BANK_X_END => {
                self.wram1[(addr as usize) & 0x0FFF] = data
            }
            OAM_START..=OAM_END => {
                // Ignore update if PPU is accessing VRAM or OAM
                if self.ppu.is_oam_accessible() {
                    self.ppu.write8(addr, data)
                }
            }
            UNUSABLE_START..=UNUSABLE_END => (),
            IO_PORTS_START..=IO_PORTS_END => {
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
                    0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.write8(addr, data),
                    0xFF01..=0xFF3F | 0xFF4C..=0xFF7F => self.io[(addr as usize) & 0x00FF] = data,
                    _ => {
                        panic!("Attempted to access [WR] memory from an invalid address: {addr:#X}")
                    }
                }
            }
            HIGH_RAM_START..=HIGH_RAM_END => self.hram[((addr as usize) & 0x00FF) - 0x0080] = data,
            INTERRUPT_ENABLE_REGISTER => self.ier = data,
        }
    }

    pub fn read16(&mut self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr.wrapping_add(1));

        ((u16::from(high)) << 8) | (u16::from(low))
    }

    pub fn read16_safe(&self, addr: u16) -> u16 {
        let low = self.read8_safe(addr);
        let high = self.read8_safe(addr.wrapping_add(1));

        ((u16::from(high)) << 8) | (u16::from(low))
    }
}
