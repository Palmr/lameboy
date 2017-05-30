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

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&self) {
        // Deal with MBC
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
