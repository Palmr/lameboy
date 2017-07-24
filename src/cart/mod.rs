use std::num::Wrapping;

pub struct Cart {
    data: Vec<u8>
}

impl Cart {
    pub fn new(data: Vec<u8>) -> Self {
        Cart {
            data: data
        }
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF => self.data[addr as usize],
            0xA000...0xC000 => 0xFF,
            _ => panic!("Attempted to access [RD] Cart memory from an invalid address: {:#X}", addr)
        }

    }

    pub fn write(&mut self, addr: u16, data: u8) {
        // Deal with MBC
        println!("Write to cart [0x{:04X}] = 0x{:02X}", addr, data);
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
        let mut x = Wrapping(0u8);
        for addr in 0x0134..0x014D {
            x = x - Wrapping(self.read(addr)) - Wrapping(1u8);
        }
        x == Wrapping(self.data[0x014D])
    }
}

use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiSetCond_FirstUseEver, Ui};
impl ImguiDebuggable for Cart {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
        ui.window(im_str!("Cart"))
            .size((220.0, 85.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(im_str!("Size: {} bytes", self.get_size()));
                ui.text(im_str!("Title: {}", self.get_title()));
                ui.text(im_str!("Checksum: {}", if self.validate_checksum() { "VALID" } else { "INVALID" }));
            });
    }
}
