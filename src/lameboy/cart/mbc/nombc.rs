use crate::lameboy::cart::mbc::{DebuggableMBC, Mbc};
use crate::lameboy::cart::parse_rom_size;
use crate::lameboy::mmu::{
    CART_RAM_BANK_X_END, CART_RAM_BANK_X_START, CART_ROM_BANK_0_END, CART_ROM_BANK_0_START,
    CART_ROM_BANK_X_END, CART_ROM_BANK_X_START,
};
use core::fmt;

pub struct NoMBC {
    rom_data: Vec<u8>,
}

impl NoMBC {
    pub fn new(rom_data: Vec<u8>, rom_size: u8) -> Result<NoMBC, String> {
        let file_size = rom_data.len();
        if file_size == parse_rom_size(rom_size)? {
            Ok(NoMBC { rom_data })
        } else {
            Err(format!(
                "ROM defined no MBC: expected file size 32KB but got {file_size} bytes"
            ))
        }
    }
}

impl Mbc for NoMBC {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            CART_ROM_BANK_0_START..=CART_ROM_BANK_0_END
            | CART_ROM_BANK_X_START..=CART_ROM_BANK_X_END => self.rom_data[addr as usize],
            CART_RAM_BANK_X_START..=CART_RAM_BANK_X_END => 0xFF,
            _ => panic!("Attempted to access cart [READ] invalid address: {addr:#X}"),
        }
    }

    fn write(&self, addr: u16, data: u8) {
        debug!("Attempted to access cart [WRITE] to no-MBC cart [0x{addr:04X}] = 0x{data:02X}");
    }
}

impl fmt::Debug for NoMBC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NoMBC")
            .field("file-size", &self.rom_data.len())
            .finish()
    }
}

impl DebuggableMBC for NoMBC {}
