
use ppu::PPU;
use mmu::mmuobject::MmuObject;

use ppu::palette::ObjectPalette;

bitflags! {
    struct SpriteFlags: u8 {
        const BACKGROUND_PRIORITY   = 0b_1000_0000;
        const Y_FLIP                = 0b_0100_0000;
        const X_FLIP                = 0b_0010_0000;
        const PALETTE               = 0b_0001_0000;
        const TILE_BANK             = 0b_0000_1000; // CGB mode only
    }
}

/// Should a sprite be displayed above background pixels or below them (except colour 0)
#[derive(PartialEq,Debug)]
pub enum SpritePriority {
    AboveBackground,
    BelowBackground,
}

pub struct Sprite {
    pub y: u8,
    pub x: u8,
    pub tile_index: u8,
    pub palette: ObjectPalette,
    pub flip_x: bool,
    pub flip_y: bool,
    pub priority: SpritePriority,
}

impl Sprite {
    pub fn new(ppu: &PPU, sprite_number: u8) -> Sprite {
        let sprite_address = 0xFE00 | ((sprite_number as u16) << 2);

        let sprite_y = ppu.read8(sprite_address);
        let sprite_x = ppu.read8(sprite_address + 1);
        let tile = ppu.read8(sprite_address + 2);
        let flags = SpriteFlags::from_bits_truncate(ppu.read8(sprite_address + 3));

        Sprite {
            y: sprite_y,
            x: sprite_x,
            tile_index: tile,
            priority: if flags.contains(SpriteFlags::BACKGROUND_PRIORITY) { SpritePriority::BelowBackground } else { SpritePriority::AboveBackground },
            flip_y: flags.contains(SpriteFlags::Y_FLIP),
            flip_x: flags.contains(SpriteFlags::X_FLIP),
            palette: if flags.contains(SpriteFlags::PALETTE) { ObjectPalette::Palette1 } else { ObjectPalette::Palette0 },
        }
    }
}
