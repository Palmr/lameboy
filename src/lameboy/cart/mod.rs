use crate::lameboy::cart::mbc::{get_mbc, DebuggableMBC};
use crate::lameboy::mmu::mmuobject::MmuObject;

mod debug;
mod mbc;

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

pub struct Cart {
    pub title: String,
    pub cart_type: u8,
    pub rom_size: u8,
    pub ram_size: u8,
    pub valid_checksum: bool,
    mbc: Box<dyn DebuggableMBC>,
}

impl Cart {
    pub fn new(rom_data: Vec<u8>) -> Self {
        let title = Cart::parse_title(&rom_data);

        let cart_type = rom_data[CARTRIDGE_TYPE_OFFSET];
        let rom_size = rom_data[ROM_SIZE_OFFSET];
        let ram_size = rom_data[RAM_SIZE_OFFSET];
        let valid_checksum = Cart::validate_checksum(&rom_data);

        let mbc = get_mbc(rom_data, cart_type, rom_size, ram_size).unwrap();

        Cart {
            title,
            cart_type,
            rom_size,
            ram_size,
            mbc,
            valid_checksum,
        }
    }

    fn parse_title(rom_data: &[u8]) -> String {
        String::from_utf8_lossy(&rom_data[TITLE_OFFSET..TITLE_OFFSET + TITLE_LENGTH_DMG])
            .trim_matches(char::from(0))
            .to_string()
    }

    fn validate_checksum(rom_data: &[u8]) -> bool {
        let mut chksum: u8 = 0;
        for byte in rom_data
            .iter()
            .take(HEADER_CHECKSUM_OFFSET)
            .skip(TITLE_OFFSET)
        {
            chksum = chksum.wrapping_sub(*byte).wrapping_sub(1);
        }
        chksum == rom_data[HEADER_CHECKSUM_OFFSET]
    }
}

impl MmuObject for Cart {
    fn read8(&self, addr: u16) -> u8 {
        self.mbc.read(addr)
    }

    fn write8(&mut self, addr: u16, data: u8) {
        self.mbc.write(addr, data)
    }
}

/// Map ROM size value from cart header into size in bytes
///
/// 00h -  32KByte (no ROM banking)
/// 01h -  64KByte (4 banks)
/// 02h - 128KByte (8 banks)
/// 03h - 256KByte (16 banks)
/// 04h - 512KByte (32 banks)
/// 05h -   1MByte (64 banks)  - only 63 banks used by MBC1
/// 06h -   2MByte (128 banks) - only 125 banks used by MBC1
/// 07h -   4MByte (256 banks)
/// 08h -   8MByte (512 banks)
/// 52h - 1.1MByte (72 banks)
/// 53h - 1.2MByte (80 banks)
/// 54h - 1.5MByte (96 banks)
pub fn parse_rom_size(rom_size: u8) -> Result<usize, String> {
    match rom_size {
        0x00 => Ok(0x0000_8000),
        0x01 => Ok(0x0001_0000),
        0x02 => Ok(0x0002_0000),
        0x03 => Ok(0x0004_0000),
        0x04 => Ok(0x0008_0000),
        0x05 => Ok(0x0010_0000),
        0x06 => Ok(0x0020_0000),
        0x07 => Ok(0x0040_0000),
        0x08 => Ok(0x0080_0000),
        0x52 => Ok(0x0012_0000),
        0x53 => Ok(0x0014_0000),
        0x54 => Ok(0x0018_0000),
        _ => Err(format!(
            "Unknown rom size value found in the cart header: 0x{rom_size:02X}"
        )),
    }
}

/// Map RAM size value from cart header into size in bytes
///
/// 00h - None
/// 01h - 2 KBytes
/// 02h - 8 Kbytes
/// 03h - 32 KBytes (4 banks of 8KBytes each)
/// 04h - 128 KBytes (16 banks of 8KBytes each)
/// 05h - 64 KBytes (8 banks of 8KBytes each)
pub fn parse_ram_size(ram_size: u8) -> Result<usize, String> {
    match ram_size {
        0x00 => Ok(0x800),
        _ => Err(format!(
            "Unknown ram size value found in the cart header: 0x{ram_size:02X}"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_title() {
        let expected_title = "CART title";

        let mut data = vec![0x00 as u8; 0xFFFF];
        data.splice(TITLE_OFFSET..0x0143, expected_title.as_bytes().to_vec());

        assert_eq!(expected_title, Cart::parse_title(&data));
    }

    #[test]
    fn validate_checksum() {
        let invalid_data = vec![0x00 as u8; 0xFFFF];
        assert_eq!(false, Cart::validate_checksum(&invalid_data));

        let mut valid_data = vec![0x00 as u8; 0xFFFF];
        valid_data.splice(
            TITLE_OFFSET..HEADER_CHECKSUM_OFFSET,
            vec![
                0x43, 0x41, 0x52, 0x54, 0x20, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
            ],
        );
        valid_data[HEADER_CHECKSUM_OFFSET] = 0x7A;

        assert_eq!(true, Cart::validate_checksum(&valid_data));
    }
}
