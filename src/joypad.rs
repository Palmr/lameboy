pub struct Joypad {
    selected_column: u8,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            selected_column: 0,
            a: false,
            b: false,
            start: false,
            select: false,
            right: false,
            left: false,
            up: false,
            down: false,
        }
    }

    fn joypad_to_byte(&self) -> u8 {
        let mut mask = 0x0F;
        match self.selected_column {
            0x10 => {
                if self.start {
                    mask &= 0b0111
                }
                if self.select {
                    mask &= 0b1011
                }
                if self.b {
                    mask &= 0b1101
                }
                if self.a {
                    mask &= 0b1110
                }
            }
            0x20 => {
                if self.down {
                    mask &= 0b0111
                }
                if self.up {
                    mask &= 0b1011
                }
                if self.left {
                    mask &= 0b1101
                }
                if self.right {
                    mask &= 0b1110
                }
            }
            _ => {}
        }

        0x0F & mask
    }
}

use mmu::mmuobject::MmuObject;
impl MmuObject for Joypad {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF00 => self.joypad_to_byte(),
            _ => panic!(
                "Attempted to access [RD] Joypad from an invalid address: {:#X}",
                addr
            ),
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0xFF00 => {
                self.selected_column = data & 0x30;
            }
            _ => panic!(
                "Attempted to access [WR] Joypad from an invalid address: {:#X}",
                addr
            ),
        }
    }
}
