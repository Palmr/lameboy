mod nombc;

pub trait Mbc {
    fn read(&self, addr: u16) -> u8;
    fn write(&self, addr: u16, data: u8);
}

pub trait DebuggableMBC: Mbc + core::fmt::Debug {}

/// Map cart type value from cart header into an Memory Bank Controller object
/// which can be used to resolve an address.
///
/// 00h  ROM ONLY
/// 01h  MBC1
/// 02h  MBC1+RAM
/// 03h  MBC1+RAM+BATTERY
/// 05h  MBC2
/// 06h  MBC2+BATTERY
/// 08h  ROM+RAM
/// 09h  ROM+RAM+BATTERY
/// 0Bh  MMM01
/// 0Ch  MMM01+RAM
/// 0Dh  MMM01+RAM+BATTERY
/// 0Fh  MBC3+TIMER+BATTERY
/// 10h  MBC3+TIMER+RAM+BATTERY
/// 11h  MBC3
/// 12h  MBC3+RAM
/// 13h  MBC3+RAM+BATTERY
/// 19h  MBC5
/// 1Ah  MBC5+RAM
/// 1Bh  MBC5+RAM+BATTERY
/// 1Ch  MBC5+RUMBLE
/// 1Dh  MBC5+RUMBLE+RAM
/// 1Eh  MBC5+RUMBLE+RAM+BATTERY
/// 20h  MBC6
/// 22h  MBC7+SENSOR+RUMBLE+RAM+BATTERY
/// FCh  POCKET CAMERA
/// FDh  BANDAI TAMA5
/// FEh  HuC3
/// FFh  HuC1+RAM+BATTERY
pub fn get_mbc(
    rom_data: Vec<u8>,
    cart_type: u8,
    rom_size: u8,
    _ram_size: u8,
) -> Result<Box<dyn DebuggableMBC>, String> {
    match cart_type {
        0x00 => {
            nombc::NoMBC::new(rom_data, rom_size).map(|v| Box::new(v) as Box<dyn DebuggableMBC>)
        }
        _ => Err(format!("Unsupported MBC type: 0x{cart_type:02X}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_on_unsupported_cart_type() -> Result<(), String> {
        match get_mbc(vec![0; 1], 0xFF, 0x00, 0x00) {
            Ok(mbc) => Err(format!("Did not expect to get a valid MBC back: {:?}", mbc)),
            Err(msg) => {
                let expected_msg = "Unsupported MBC type: 0xFF";
                if msg == expected_msg {
                    Ok(())
                } else {
                    Err(format!("Expected: '{expected_msg}' Got: '{msg}'"))
                }
            }
        }
    }

    #[test]
    fn error_on_invalid_nombc_rom_size() -> Result<(), String> {
        match get_mbc(vec![0; 1], 0x00, 0x00, 0x00) {
            Ok(mbc) => Err(format!("Did not expect to get a valid MBC back: {:?}", mbc)),
            Err(msg) => {
                let expected_msg = "ROM defined no MBC: expected file size 32KB but got 1 bytes";
                if msg == expected_msg {
                    Ok(())
                } else {
                    Err(format!("Expected: '{expected_msg}' Got: '{msg}'"))
                }
            }
        }
    }

    #[test]
    fn nombc_for_cart_type_zero() -> Result<(), String> {
        match get_mbc(vec![0; 0x8000], 0x00, 0x00, 0x00) {
            Ok(_) => {
                // TODO - not sure how to check the impl of the MBC type here...
                Ok(())
            }
            Err(msg) => Err(format!("Expected valid nombc Got: '{msg}'")),
        }
    }
}
