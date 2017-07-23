bitflags! {
    pub struct ControlFlags: u8 {
        const BG_DISPLAY        = 0b_0000_0001;
        const OBJ_DISPLAY       = 0b_0000_0010;
        const OBJ_SIZE          = 0b_0000_0100;
        const BG_TILE_MAP       = 0b_0000_1000;
        const BG_WIN_TILE_SET   = 0b_0001_0000;
        const WINDOW_DISPLAY    = 0b_0010_0000;
        const WINDOW_TILE_MAP   = 0b_0100_0000;
        const DISPLAY_ENABLE    = 0b_1000_0000;
    }
}
bitflags! {
    pub struct StatusFlags: u8 {
        const COINCIDENCE   = 0b_0000_0100;
        const INT_HBLANK    = 0b_0000_1000;
        const INT_VBLANK    = 0b_0001_0000;
        const INT_OAM       = 0b_0010_0000;
        const INT_LYC       = 0b_0100_0000;
    }
}

/// Registers for video generation
pub struct Registers {
    pub control: ControlFlags,
    pub status: StatusFlags,
    pub scroll_y: u8,
    pub scroll_x: u8,
    pub ly: u8,
    pub lyc: u8,
    pub dma: u8,
    pub bg_palette: u8,
    pub obj0_palette: u8,
    pub obj1_palette: u8,
    pub window_y: u8,
    pub window_x: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            control: ControlFlags::empty(),
            status: StatusFlags::empty(),
            scroll_y: 0,
            scroll_x: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bg_palette: 0,
            obj0_palette: 0,
            obj1_palette: 0,
            window_y: 0,
            window_x: 0,
        }
    }
}
