use self::super::cart::Cart;

pub struct MMU<'c> {
    cart: &'c Cart,
    /// Video RAM [0x8000 - 0x9FFF] (Bank 0-1 in CGB Mode)
    vram: Box<[u8; 0x2000]>,
	/// Work RAM 0 [0xC000 - 0xCFFF]
	wram0: Box<[u8; 0x1000]>,
	/// Work RAM 1 [0xD000 - 0xDFFF] (Bank 1-7 in CGB Mode)
	wram1: Box<[u8; 0x1000]>,
	/// Sprite Attribute Table [0xFE00 - 0xFE9F]
	oam: Box<[u8; 0x00A0]>,
	/// Unusable region [0xFEA0 - 0xFEFF]
	unusable: u8,
	/// I/O Ports [FF00 - 0xFF7F]
	io: Box<[u8; 0x0080]>,
	/// High RAM [0xFF80 - 0xFFFE]
	hram: Box<[u8; 0x007F]>,
	/// Interrupt Enable Register [0xFFFF]
	ier: u8,
}

impl<'c> MMU<'c> {
    pub fn new(cart: &Cart) -> MMU {
        MMU {
            cart: cart,
            vram: Box::new([0; 0x2000]),
            wram0: Box::new([0; 0x1000]),
            wram1: Box::new([0; 0x1000]),
            oam: Box::new([0; 0x00A0]),
            unusable: 0x00,
            io: Box::new([0; 0x0080]),
            hram: Box::new([0; 0x007F]),
            ier: 0x00,
        }
    }

    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x0000...0x7FFF |
            0xA000...0xBFFF => self.cart.read(addr),
            0x8000...0x9FFF => self.vram[(addr as usize) & 0x1FFF],
            0xC000...0xCFFF |
            0xE000...0xEFFF => self.wram0[(addr as usize) & 0x0FFF],
            0xD000...0xDFFF |
            0xF000...0xFDFF => self.wram1[(addr as usize) & 0x0FFF],
            0xFE00...0xFE9F => self.oam[(addr as usize) & 0x00FF],
            0xFEA0...0xFEFF => self.unusable,
            0xFF00...0xFF7F => self.io[(addr as usize) & 0x00FF],
            0xFF80...0xFFFE => self.hram[(addr as usize) & 0x00FF],
            0xFFFF => self.ier,
            _ => panic!("Attempted to access memory from an invalid address: {:#X}", addr)
        }
    }

    pub fn read16(&self, addr: u16) -> u16 {
        let low = self.read8(addr);
        let high = self.read8(addr+1);

        ((high as u16) << 8) | (low as u16)
    }

    pub fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x0000...0x7FFF |
            0xA000...0xBFFF => self.cart.write(addr, data),
            0x8000...0x9FFF => self.vram[(addr as usize) & 0x1FFF] = data,
            0xC000...0xCFFF |
            0xE000...0xEFFF => self.wram0[(addr as usize) & 0x0FFF] = data,
            0xD000...0xDFFF |
            0xF000...0xFDFF => self.wram1[(addr as usize) & 0x0FFF] = data,
            0xFE00...0xFE9F => self.oam[(addr as usize) & 0x00FF] = data,
            0xFEA0...0xFEFF => (),
            0xFF00...0xFF7F => self.io[(addr as usize) & 0x00FF] = data,
            0xFF80...0xFFFE => self.hram[(addr as usize) & 0x00FF] = data,
            0xFFFF => self.ier = data,
            _ => panic!("Attempted to access memory from an invalid address: {:#X}", addr)
        }
    }
}
