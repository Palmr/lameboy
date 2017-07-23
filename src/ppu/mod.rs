use std::os::raw::c_void;

use glium::backend::Facade;
use glium::Surface;

pub mod gpu;
use ppu::gpu::*;

pub mod registers;
use ppu::registers::Registers;
use ppu::registers::ControlFlags;
use ppu::registers::StatusFlags;

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
    registers: Registers,
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
            registers: Registers::new(),
            gpu: gpu,
            screen_buffer: screen_buffer,
        }
    }

    /// Handle memory reads from the PPU data registers only, otherwise panic
    pub fn read8(&self, addr: u16) -> u8 {
        match addr {
            0xFF40 => self.registers.control.bits(),
            0xFF41 => self.registers.status,
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
            _ => panic!("Attempted to access [RD] PPU memory from an invalid address: {:#X}", addr)
        }
    }

    /// Handle memory writes to the PPU data registers only, otherwise panic
    pub fn write8(&mut self, addr: u16, data: u8) {
        match addr {
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
            _ => panic!("Attempted to access [WR] PPU memory from an invalid address: {:#X}", addr)
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
                    self.registers.ly += 1;

                    if self.registers.ly == 143 {
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

    pub fn draw<S: Surface>(&mut self, target: &mut S) {
        self.gpu.load_texture(&self.screen_buffer);

        self.gpu.draw(target);
    }

    pub fn get_tex_id (&self) -> *mut c_void {
        self.gpu.get_tex_id()
    }

    pub fn apply_test_pattern(&mut self, pattern: &TestPattern, mod_value: usize) {
        for y in 0..144 {
            for x in 0..160 {
                self.screen_buffer[y * SCREEN_WIDTH + x] =
                    match pattern {
                        &TestPattern::BLANK => 0u8,
                        &TestPattern::DIAGONAL => (((x+y) / mod_value) % 4) as u8,
                        &TestPattern::XOR => ((x/mod_value ^ y/mod_value) % 4) as u8,
                    }

            }
        }
    }
}

use gui::imguidebug::{ImguiDebug, ImguiDebuggable};
use imgui::{ImGuiSetCond_FirstUseEver, Ui};
impl ImguiDebuggable for PPU {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
        ui.window(im_str!("PPU"))
                .size((180.0, 95.0), ImGuiSetCond_FirstUseEver)
                .resizable(true)
                .build(|| {
                    ui.checkbox(im_str!("Apply test"), &mut imgui_debug.apply_test_pattern);
                    ui.slider_int(im_str!("Mod"), &mut imgui_debug.ppu_mod, 1, 20).build();
                    if ui.small_button(im_str!("Blank")) {
                        imgui_debug.test_pattern_type = TestPattern::BLANK;
                    }
                    ui.same_line(0.0);
                    if ui.small_button(im_str!("Diagonal")) {
                        imgui_debug.test_pattern_type = TestPattern::DIAGONAL;
                    }
                    ui.same_line(0.0);
                    if ui.small_button(im_str!("XOR")) {
                        imgui_debug.test_pattern_type = TestPattern::XOR;
                    }
                });

        ui.window(im_str!("PPU-registers"))
            .size((355.0, 230.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(im_str!("Control: {:?}", self.registers.control));
                ui.text(im_str!("Status: {:?} - {:?}", self.registers.status, StatusFlags::from_bits_truncate(self.registers.status)));
                ui.text(im_str!("Scroll Y: {:?}", self.registers.scroll_y));
                ui.text(im_str!("Scroll X: {:?}", self.registers.scroll_x));
                ui.text(im_str!("LY: {:?}", self.registers.ly));
                ui.text(im_str!("LYC: {:?}", self.registers.lyc));
                ui.text(im_str!("DMA: {:?}", self.registers.dma));
                ui.text(im_str!("BG Palette: {:?}", self.registers.bg_palette));
                ui.text(im_str!("OBJ0 Palette: {:?}", self.registers.obj0_palette));
                ui.text(im_str!("OBJ1 Palette: {:?}", self.registers.obj1_palette));
                ui.text(im_str!("Window Y: {:?}", self.registers.window_y));
                ui.text(im_str!("Window X: {:?}", self.registers.window_x));
            });
    }
}
