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

    pub fn post_boot_reset(&mut self) {
        self.write8(0xFF05, 0x00);
        self.write8(0xFF06, 0x00);
        self.write8(0xFF07, 0x00);
        self.write8(0xFF10, 0x80);
        self.write8(0xFF11, 0xBF);
        self.write8(0xFF12, 0xF3);
        self.write8(0xFF14, 0xBF);
        self.write8(0xFF16, 0x3F);
        self.write8(0xFF17, 0x00);
        self.write8(0xFF19, 0xBF);
        self.write8(0xFF1A, 0x7F);
        self.write8(0xFF1B, 0xFF);
        self.write8(0xFF1C, 0x9F);
        self.write8(0xFF1E, 0xBF);
        self.write8(0xFF20, 0xFF);
        self.write8(0xFF21, 0x00);
        self.write8(0xFF22, 0x00);
        self.write8(0xFF23, 0xBF);
        self.write8(0xFF24, 0x77);
        self.write8(0xFF25, 0xF3);
        self.write8(0xFF26, 0xF1);
        self.write8(0xFF40, 0x91);
        self.write8(0xFF42, 0x00);
        self.write8(0xFF43, 0x00);
        self.write8(0xFF45, 0x00);
        self.write8(0xFF47, 0xFC);
        self.write8(0xFF48, 0xFF);
        self.write8(0xFF49, 0xFF);
        self.write8(0xFF4A, 0x00);
        self.write8(0xFF4B, 0x00);
        self.write8(0xFFFF, 0x00);
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
            0xFF80...0xFFFE => self.hram[((addr as usize) & 0x00FF) - 0x0080],
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
            0xFF80...0xFFFE => self.hram[((addr as usize) & 0x00FF) - 0x0080] = data,
            0xFFFF => self.ier = data,
            _ => panic!("Attempted to access memory from an invalid address: {:#X}", addr)
        }
    }
}
