pub struct Cart {
    data: Vec<u8>,
}

const LOGO_OFFSET: usize = 0x0104;
const LOGO_LENGTH: usize = 0x30;

const TITLE_OFFSET: usize = 0x0134;
const TITLE_LENGTH_DMG: usize = 0x0F;
const TITLE_LENGTH_GBC: usize = 0x0B;

const MANUFACTURER_CODE_OFFSET: usize = 0x013F;
const MANUFACTURER_CODE_LENGTH: usize = 0x04;

const CGB_FLAG_OFFSET: usize = 0x0143;

const NEW_LICENSEE_CODE_OFFSET: usize = 0x0144;
const NEW_LICENSEE_CODE_LENGTH: usize = 0x02;

const SGB_FLAG_OFFSET: usize = 0x0146;
const CARTRIDGE_TYPE_OFFSET: usize = 0x0147;
const ROM_SIZE_OFFSET: usize = 0x0148;
const RAM_SIZE_OFFSET: usize = 0x0149;
const DESTINATION_CODE_OFFSET: usize = 0x014A;
const OLD_LICENSEE_CODE_OFFSET: usize = 0x014B;
const MASK_ROM_VERSION_NUMBER_OFFSET: usize = 0x014C;
const HEADER_CHECKSUM_OFFSET: usize = 0x014D;

const GLOBAL_CHECKSUM_OFFSET: usize = 0x014E;
const GLOBAL_CHECKSUM_LENGTH: usize = 0x02;

impl Cart {
    pub fn new(data: Vec<u8>) -> Self {
        Cart { data: data }
    }

    pub fn get_size(&self) -> usize {
        self.data.len()
    }

    pub fn get_title(&self) -> String {
        String::from_utf8_lossy(&self.data[TITLE_OFFSET..TITLE_OFFSET + TITLE_LENGTH_DMG])
            .trim_matches(char::from(0))
            .to_string()
    }

    pub fn get_cart_type(&self) -> u8 {
        self.data[CARTRIDGE_TYPE_OFFSET]
    }

    pub fn get_rom_size(&self) -> u8 {
        self.data[ROM_SIZE_OFFSET]
    }

    pub fn get_ram_size(&self) -> u8 {
        self.data[RAM_SIZE_OFFSET]
    }

    pub fn validate_checksum(&self) -> bool {
        let mut chksum: u8 = 0;
        for addr in TITLE_OFFSET..HEADER_CHECKSUM_OFFSET {
            chksum = chksum.wrapping_sub(self.data[addr]).wrapping_sub(1);
        }
        chksum == self.data[HEADER_CHECKSUM_OFFSET]
    }
}

use mmu::mmuobject::MmuObject;
impl MmuObject for Cart {
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            // TODO - implement MBC variants
            0x0000...0x7FFF => self.data[addr as usize],
            0xA000...0xC000 => 0xFF,
            _ => panic!(
                "Attempted to access [RD] Cart memory from an invalid address: {:#X}",
                addr
            ),
        }
    }

    fn write8(&mut self, addr: u16, data: u8) {
        // TODO - implement MBC variants
        debug!("Write to cart [0x{:04X}] = 0x{:02X}", addr, data);
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
                ui.text(im_str!(
                    "Checksum: {}",
                    if self.validate_checksum() {
                        "VALID"
                    } else {
                        "INVALID"
                    }
                ));
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_size() {
        let expected_size = 0x1234;

        let data = vec![0x00 as u8; expected_size];
        let cart = Cart::new(data);
        assert_eq!(expected_size, cart.get_size());
    }

    #[test]
    fn get_title() {
        let expected_title = "CART title";

        let mut data = vec![0x00 as u8; 0xFFFF];
        data.splice(TITLE_OFFSET..0x0143, expected_title.as_bytes().to_vec());
        let cart = Cart::new(data);
        assert_eq!(expected_title, cart.get_title());
    }

    #[test]
    fn get_cart_type() {
        let expected_cart_type = 0x19;

        let mut data = vec![0x00 as u8; 0xFFFF];
        data[CARTRIDGE_TYPE_OFFSET] = expected_cart_type;
        let cart = Cart::new(data);
        assert_eq!(expected_cart_type, cart.get_cart_type());
    }

    #[test]
    fn get_rom_size() {
        let expected_rom_size = 0x05;

        let mut data = vec![0x00 as u8; 0xFFFF];
        data[ROM_SIZE_OFFSET] = expected_rom_size;
        let cart = Cart::new(data);
        assert_eq!(expected_rom_size, cart.get_rom_size());
    }

    #[test]
    fn get_ram_size() {
        let expected_ram_size = 0x02;

        let mut data = vec![0x00 as u8; 0xFFFF];
        data[RAM_SIZE_OFFSET] = expected_ram_size;
        let cart = Cart::new(data);
        assert_eq!(expected_ram_size, cart.get_ram_size());
    }

    #[test]
    fn validate_checksum() {
        let invalid_data = vec![0x00 as u8; 0xFFFF];
        let invalid_cart = Cart::new(invalid_data);
        assert_eq!(false, invalid_cart.validate_checksum());

        let mut valid_data = vec![0x00 as u8; 0xFFFF];
        valid_data.splice(
            TITLE_OFFSET..HEADER_CHECKSUM_OFFSET,
            vec![
                0x43, 0x41, 0x52, 0x54, 0x20, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            ],
        );
        valid_data[HEADER_CHECKSUM_OFFSET] = 0x7A;
        let valid_cart = Cart::new(valid_data);
        assert_eq!(true, valid_cart.validate_checksum());
    }
}
