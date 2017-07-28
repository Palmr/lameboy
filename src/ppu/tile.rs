
use ppu::PPU;
use mmu::mmuobject::MmuObject;

pub struct Tile {
    pub rows: [[u8; 8]; 8],
}

impl Tile {
    pub fn new(ppu: &PPU, tile_index: u8) -> Tile {
        let tile_address = 0x8000 | (tile_index as u16) << 4;
        let mut rows = [[0u8; 8]; 8];

        for y in 0..8 {
            let low = ppu.read8(tile_address + y * 2);
            let high = ppu.read8(tile_address + 1 + y * 2);
            for x in 0..8 {
                rows[y as usize][x as usize] = ((low >> (7 - x)) & 0x01) | (((high >> (7 - x)) & 0x01) << 1);
            }
        }

        Tile {
            rows: rows
        }
    }
}
