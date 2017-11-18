pub struct Cart {
    data: Vec<u8>
}

impl Cart {
    pub fn new(data: Vec<u8>) -> Self {
        Cart {
            data: data
        }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_title(&self) -> String {
        String::from_utf8_lossy(&self.data[0x0134..0x0143]).to_string()
    }

    pub fn get_cart_type(&self) -> u8 {
        self.data[0x0147]
    }

    pub fn get_rom_size(&self) -> u8 {
        self.data[0x0148]
    }

    pub fn get_ram_size(&self) -> u8 {
        self.data[0x0149]
    }

    pub fn validate_checksum(&self) -> bool {
        let mut chksum: u8 = 0;
        for addr in 0x0134..0x014D {
            chksum = chksum.wrapping_sub(self.data[addr]).wrapping_sub(1);
        }
        chksum == self.data[0x014D]
    }
}

use mmu::mmuobject::MmuObject;
impl MmuObject for Cart {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // TODO - implement MBC variants
            0x0000...0x7FFF => self.data[addr as usize],
            0xA000...0xC000 => 0xFF,
            _ => panic!("Attempted to access [RD] Cart memory from an invalid address: {:#X}", addr)
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        // TODO - implement MBC variants
        println!("Write to cart [0x{:04X}] = 0x{:02X}", addr, data);
    }
}

use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiCond, Ui};
impl ImguiDebuggable for Cart {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, _: &mut ImguiDebug) {
        ui.window(im_str!("Cart"))
            .size((180.0, 127.0), ImGuiCond::FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(im_str!("Size: {} bytes", self.get_size()));
                ui.text(im_str!("Title: {}", self.get_title()));
                ui.text(im_str!("Type: {}", self.get_cart_type()));
                ui.text(im_str!("ROM Size: {}", self.get_rom_size()));
                ui.text(im_str!("RAM Size: {}", self.get_ram_size()));
                ui.text(im_str!("Checksum: {}", if self.validate_checksum() { "VALID" } else { "INVALID" }));
            });
    }
}
