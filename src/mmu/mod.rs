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

    fn map(&self, addr: u16) -> &u8 {

    }

    pub fn read8(&self, addr: u16) -> u8 {

    }
    pub fn read16(&self, addr: u16) -> u16 {

    }
    pub fn write8(&self, addr: u16, data: u8) {

    }
}
