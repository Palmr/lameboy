use glium::backend::Facade;
use glium::Surface;

pub mod gpu;
use ppu::gpu::*;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub enum TestPattern {
    BLANK,
    DIAGONAL,
    XOR,
}

enum Mode {
    ReadOam,
    ReadVram,
    HBlank,
    VBlank,
}

pub struct PPU {
    mode_clock: usize,
    mode: Mode,
    line: u8,
    gpu: GPU,
    screen_buffer: Box<Vec<u8>>,
}

impl PPU {
    pub fn new<F: Facade>(display: &F) -> PPU {
        let gpu = GPU::new(display);

        let screen_buffer = Box::new(vec![0 as u8; SCREEN_WIDTH * SCREEN_HEIGHT]);

        PPU {
            mode_clock: 0,
            mode: Mode::HBlank,
            line: 0,
            gpu: gpu,
            screen_buffer: screen_buffer,
        }
    }

    pub fn cycle(&mut self, cpu_duration: u8) {
        self.mode_clock += cpu_duration as usize;

	    match self.mode	{
            // OAM read mode, scanline active
            Mode::ReadOam => {
                if self.mode_clock >= 80 {
                    // Enter scanline Mode::ReadVram
                    self.mode_clock = 0;
                    self.mode = Mode::ReadVram;
                }
            },
            // VRAM read mode, scanline active
            // Treat end of Mode::ReadVram as end of scanline
            Mode::ReadVram => {
                if self.mode_clock >= 172 {
                    // Enter hblank
                    self.mode_clock = 0;
                    self.mode = Mode::HBlank;

                    // Write a scanline to the framebuffer
                    // TODO - self.renderscan();
                }
            },

            // Hblank
            // After the last hblank, push the screen data to canvas
            Mode::HBlank => {
                if self.mode_clock >= 204 {
                    self.mode_clock = 0;
                    self.line += 1;

                    if self.line == 143 {
                        // Enter vblank
                        self.mode = Mode::VBlank;
                        // TODO - self.canvas.putImageData(self.scrn, 0, 0);
                    }
                    else {
                        self.mode = Mode::ReadOam;
                    }
                }
            },


            // Vblank (10 lines)
            Mode::VBlank => {
                if self.mode_clock >= 456 {
                    self.mode_clock = 0;
                    self.line += 1;

                    if self.line > 153 {
                        // Restart scanning modes
                        self.mode = Mode::ReadOam;
                        self.line = 0;
                    }
                }
            }
        }
    }

    pub fn draw<S: Surface>(&mut self, target: &mut S) {
        self.gpu.load_texture(&self.screen_buffer);

        self.gpu.draw(target);
    }

    pub fn apply_test_pattern(&mut self, pattern: TestPattern) {
        for y in 0..144 {
            for x in 0..160 {
                self.screen_buffer[y * SCREEN_WIDTH + x] =
                    match pattern {
                        TestPattern::BLANK => 0u8,
                        TestPattern::DIAGONAL => (((x+y) / 8) % 4) as u8,
                        TestPattern::XOR => ((x/4^y/4) % 4) as u8,
                    }

            }
        }
    }
}
