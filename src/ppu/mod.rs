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

pub struct PPU {
    gpu: GPU,
    screen_buffer: Box<Vec<u8>>,
}

impl PPU {
    pub fn new<F: Facade>(display: &F) -> PPU {
        let gpu = GPU::new(display);

        let screen_buffer = Box::new(vec![0 as u8; SCREEN_WIDTH * SCREEN_HEIGHT]);

        PPU {
            gpu: gpu,
            screen_buffer: screen_buffer,
        }
    }

    pub fn draw<S: Surface>(&mut self, target: &mut S) {
        self.gpu.load_texture(&self.screen_buffer);

        self.gpu.draw(target);
    }

    pub fn testing(&mut self, pattern: TestPattern) {
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
