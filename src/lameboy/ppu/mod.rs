use glium::backend::Facade;
use glium::Surface;

use lameboy::interrupts::{INT_LCD_STAT, INT_VBLANK};
use lameboy::mmu::mmuobject::MmuObject;
use lameboy::ppu::gpu::*;
use lameboy::ppu::palette::*;
use lameboy::ppu::registers::ControlFlags;
use lameboy::ppu::registers::Registers;
use lameboy::ppu::registers::StatusInterruptFlags;
use lameboy::ppu::sprite::{Sprite, SpritePriority};
use lameboy::ppu::tile::Tile;

mod debug;
pub mod gpu;
pub mod palette;
pub mod registers;
pub mod sprite;
pub mod tile;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Debug)]
enum Mode {
    ReadOam,
    ReadVram,
    HBlank,
    VBlank,
}

pub struct PPU {
    /// Video RAM [0x8000 - 0x9FFF] (Bank 0-1 in CGB Mode)
    vram: Box<[u8; 0x2000]>,
    /// Sprite Attribute Table [0xFE00 - 0xFE9F]
    oam: Box<[u8; 0x00A0]>,
    mode_clock: usize,
    mode: Mode,
    registers: Registers,
    gpu: GPU,
    screen_buffer: Box<[u8; SCREEN_WIDTH * SCREEN_HEIGHT]>,
}

impl PPU {
    pub fn new<F: Facade>(display: &F) -> PPU {
        let gpu = GPU::new(display);

        PPU {
            vram: Box::new([0; 0x2000]),
            oam: Box::new([0; 0x00A0]),
            mode_clock: 0,
            mode: Mode::HBlank,
            registers: Registers::new(),
            gpu,
            screen_buffer: Box::new([0; SCREEN_WIDTH * SCREEN_HEIGHT]),
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.mode_clock = 0;
        self.mode = Mode::HBlank;
    }

    /// Build the stat register using its writable value and then overriding the last 4 bits with
    /// status information.
    fn combine_status_mode(&self) -> u8 {
        // Mask away any existing read-only bits
        let mut stat = self.registers.status & 0b1111_1000;

        // Set coincidence flag
        if self.registers.ly == self.registers.lyc {
            stat |= 0b0000_0100
        }

        // Set mode flag
        match self.mode {
            Mode::HBlank => stat & 0b1111_1100,
            Mode::VBlank => stat | 0b0000_0001,
            Mode::ReadOam => stat | 0b0000_0010,
            Mode::ReadVram => stat | 0b0000_0011,
        }
    }

    pub fn is_vram_accessible(&self) -> bool {
        !matches!(self.mode, Mode::ReadVram)
    }

    pub fn is_oam_accessible(&self) -> bool {
        !matches!(self.mode, Mode::ReadVram | Mode::ReadOam)
    }

    /// Cycle the PPU based on the how long the CPU spent since it last cycled.
    /// Return a byte containing the Interrupt Flag value from the PPU
    pub fn cycle(&mut self, cpu_duration: u8) -> u8 {
        let mut int_flag = 0x00;

        if self
            .registers
            .control
            .contains(ControlFlags::DISPLAY_ENABLE)
        {
            self.mode_clock += cpu_duration as usize;

            let status_int_flags: StatusInterruptFlags =
                StatusInterruptFlags::from_bits_truncate(self.read8(0xFF41));

            // If the interrupt for LY Coincidence is set, and it's a coincidence, set interrupt bit
            if status_int_flags.contains(StatusInterruptFlags::INT_ENABLE_LYC)
                && self.registers.ly == self.registers.lyc
            {
                // Set interrupt bit
                int_flag |= INT_LCD_STAT;
            }

            match self.mode {
                // OAM read mode, scanline active
                Mode::ReadOam => {
                    if self.mode_clock >= 80 {
                        // Enter scanline Mode::ReadVram
                        self.mode_clock = 0;
                        self.mode = Mode::ReadVram;
                    }
                }
                // VRAM read mode, scanline active
                // Treat end of Mode::ReadVram as end of scanline
                Mode::ReadVram => {
                    if self.mode_clock >= 172 {
                        // Enter hblank
                        self.mode_clock = 0;
                        self.mode = Mode::HBlank;
                        if status_int_flags.contains(StatusInterruptFlags::INT_ENABLE_HBLANK) {
                            // Set interrupt bit
                            int_flag |= INT_LCD_STAT;
                        }

                        // Write a scanline to the framebuffer
                        self.renderscan();
                    }
                }

                // Hblank
                // After the last hblank, push the screen data to canvas
                Mode::HBlank => {
                    if self.mode_clock >= 204 {
                        self.mode_clock = 0;
                        self.registers.ly += 1;

                        if self.registers.ly == 144 {
                            // Enter vblank
                            self.mode = Mode::VBlank;
                            // Set interrupt bit
                            int_flag |= INT_VBLANK;
                            if status_int_flags.contains(StatusInterruptFlags::INT_ENABLE_VBLANK) {
                                // Set interrupt bit
                                int_flag |= INT_LCD_STAT;
                            }
                            self.gpu.load_texture(self.screen_buffer.as_ref());
                        } else {
                            self.mode = Mode::ReadOam;
                            if status_int_flags.contains(StatusInterruptFlags::INT_ENABLE_OAM) {
                                // Set interrupt bit
                                int_flag |= INT_LCD_STAT;
                            }
                        }
                    }
                }

                // Vblank (10 lines)
                Mode::VBlank => {
                    if self.mode_clock >= 456 {
                        self.mode_clock = 0;
                        self.registers.ly += 1;

                        if self.registers.ly > 153 {
                            // Restart scanning modes
                            self.mode = Mode::ReadOam;
                            self.registers.ly = 0;
                        }
                    }
                }
            }
        }

        int_flag
    }

    fn renderscan(&mut self) {
        let line_buffer = &mut [0u8; SCREEN_WIDTH];

        let bg_palette = unpack_palette(self.registers.bg_palette);

        if self.registers.control.contains(ControlFlags::BG_DISPLAY) {
            // VRAM offset for the tile map
            let mut bg_map_offset: u16 =
                if self.registers.control.contains(ControlFlags::BG_TILE_MAP) {
                    0x1C00
                } else {
                    0x1800
                };

            // Which line of tiles to use in the map
            bg_map_offset +=
                (u16::from(self.registers.ly.wrapping_add(self.registers.scroll_y)) >> 3) * 32;

            // Which tile to start with in the map line
            let mut x_tile_offset = u16::from(self.registers.scroll_x) >> 3;

            // Which line of pixels to use in the tiles
            let tile_y_offset = self.registers.ly.wrapping_add(self.registers.scroll_y) % 8;

            // Where in the tile line to start
            let mut tile_x_offset = self.registers.scroll_x % 8;

            // Read tile index from the background map
            //var colour;
            let mut tile_index: usize =
                self.vram[(bg_map_offset + x_tile_offset) as usize] as usize;

            // If the tile data set in use is #1, the
            // indices are signed; calculate a real tile offset
            if !self
                .registers
                .control
                .contains(ControlFlags::BG_WIN_TILE_SET)
            {
                tile_index = (128 + (i16::from(tile_index as i8) + 128)) as usize;
            };

            for pixel in line_buffer.iter_mut().take(SCREEN_WIDTH) {
                let tile_addr = (tile_index << 4) + (tile_y_offset as usize * 2);
                let low = self.vram[tile_addr];
                let high = self.vram[tile_addr + 1usize];

                let colour = ((low >> (7 - tile_x_offset)) & 0x01)
                    | (((high >> (7 - tile_x_offset)) & 0x01) << 1);

                // Plot the pixel to canvas
                *pixel = bg_palette[(colour & 0x03) as usize];

                // When this tile ends, read another
                tile_x_offset += 1;
                if tile_x_offset == 8 {
                    tile_x_offset = 0;
                    x_tile_offset = (x_tile_offset + 1) & 31;
                    tile_index = self.vram[(bg_map_offset + x_tile_offset) as usize] as usize;
                    if !self
                        .registers
                        .control
                        .contains(ControlFlags::BG_WIN_TILE_SET)
                    {
                        tile_index = (128 + (i16::from(tile_index as i8) + 128)) as usize;
                    };
                }
            }
        }

        if self.registers.control.contains(ControlFlags::OBJ_DISPLAY) {
            let sprite_height = if self.registers.control.contains(ControlFlags::OBJ_SIZE) {
                16
            } else {
                8
            };

            for sprite_index in 0..40 {
                let sprite = Sprite::new(self, sprite_index);

                let sprite_y_i16: i16 = i16::from(sprite.y) - 16;
                let sprite_x_i16: i16 = i16::from(sprite.x) - 8;
                let line_i16 = i16::from(self.registers.ly);

                // Skip sprites out of screen bounds
                if sprite_y_i16 <= -16
                    || sprite_x_i16 <= -8
                    || sprite_y_i16 > SCREEN_HEIGHT as i16
                    || sprite_x_i16 > SCREEN_WIDTH as i16
                {
                    continue;
                }

                let tile = Tile::new(self, sprite.tile_index);

                if sprite_y_i16 <= line_i16 && (sprite_y_i16 + sprite_height) > line_i16 {
                    let sprite_palette = unpack_palette(match sprite.palette {
                        ObjectPalette::Palette0 => self.registers.obj0_palette,
                        ObjectPalette::Palette1 => self.registers.obj0_palette,
                    });

                    let tile_row = if sprite.flip_y {
                        tile.rows[(7 - line_i16 - sprite_y_i16) as usize]
                    } else {
                        tile.rows[(line_i16 - sprite_y_i16) as usize]
                    };

                    for x in 0..8 {
                        let line_x: i16 = i16::from(sprite.x + x) - 8;
                        if line_x >= 0
                            && line_x < 160
                            && tile_row[x as usize] != 0
                            && (sprite.priority == SpritePriority::AboveBackground
                                || line_buffer[line_x as usize] != bg_palette[0])
                        {
                            line_buffer[line_x as usize] =
                                sprite_palette[tile_row[x as usize] as usize];
                        }
                    }
                }
            }
        }

        // Update the screen buffer with the line buffer
        let screen_line_offset = SCREEN_WIDTH * self.registers.ly as usize;
        for (idx, &pixel) in line_buffer.iter().enumerate().take(SCREEN_WIDTH) {
            self.screen_buffer[screen_line_offset + idx] = pixel;
        }
    }

    pub fn draw<S: Surface>(&mut self, target: &mut S) {
        self.gpu.load_texture(self.screen_buffer.as_ref());

        self.gpu.draw(target);
    }
}

impl MmuObject for PPU {
    /// Handle memory reads from the PPU data registers only, otherwise panic
    fn read8(&self, addr: u16) -> u8 {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr as usize) & 0x1FFF],
            0xFE00..=0xFE9F => self.oam[(addr as usize) & 0x00FF],
            0xFF40 => self.registers.control.bits(),
            0xFF41 => self.combine_status_mode(),
            0xFF42 => self.registers.scroll_y,
            0xFF43 => self.registers.scroll_x,
            0xFF44 => self.registers.ly,
            0xFF45 => self.registers.lyc,
            0xFF46 => self.registers.dma,
            0xFF47 => self.registers.bg_palette,
            0xFF48 => self.registers.obj0_palette,
            0xFF49 => self.registers.obj1_palette,
            0xFF4A => self.registers.window_y,
            0xFF4B => self.registers.window_x,
            _ => panic!(
                "Attempted to access [RD] PPU memory from an invalid address: {:#X}",
                addr
            ),
        }
    }

    /// Handle memory writes to the PPU data registers only, otherwise panic
    fn write8(&mut self, addr: u16, data: u8) {
        match addr {
            0x8000..=0x9FFF => self.vram[(addr as usize) & 0x1FFF] = data,
            0xFE00..=0xFE9F => self.oam[(addr as usize) & 0x00FF] = data,
            0xFF40 => self.registers.control = ControlFlags::from_bits_truncate(data),
            0xFF41 => self.registers.status = data,
            0xFF42 => self.registers.scroll_y = data,
            0xFF43 => self.registers.scroll_x = data,
            0xFF44 => self.registers.ly = data,
            0xFF45 => self.registers.lyc = data,
            0xFF46 => self.registers.dma = data,
            0xFF47 => self.registers.bg_palette = data,
            0xFF48 => self.registers.obj0_palette = data,
            0xFF49 => self.registers.obj1_palette = data,
            0xFF4A => self.registers.window_y = data,
            0xFF4B => self.registers.window_x = data,
            _ => panic!(
                "Attempted to access [WR] PPU memory from an invalid address: {:#X}",
                addr
            ),
        }
    }
}
