use glium::backend::Facade;
use glium::Surface;

pub mod gpu;
use ppu::gpu::*;

pub struct PPU {
    gpu: GPU,
}

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

impl PPU {
    pub fn new<F: Facade>(display: &F) -> PPU {
        let mut gpu = GPU::new(display);

        gpu.render_test();

        PPU {
            gpu: gpu
        }
    }

    pub fn draw<S: Surface>(&self, target: &mut S) {
        self.gpu.draw(target);
    }
}
