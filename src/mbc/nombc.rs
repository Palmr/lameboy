use mbc::MBC;

pub struct NoMBC {
    rom: Vec<u8>,
}

impl NoMBC {
    pub fn new(data: Vec<u8>) -> Result<NoMBC, &'static str> {
        if data.len() == 0x8000 {
            Ok(NoMBC { rom: data })
        } else {
            Err("ROM defined no MBC but is not 32KB")
        }
    }
}

impl MBC for NoMBC {
    fn read(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    fn write(&self, addr: u16, data: u8) {
        ()
    }
}
