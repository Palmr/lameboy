#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;

extern crate imgui_sys;
extern crate imgui_glium_renderer;

extern crate nalgebra;

mod gui;
use gui::GUI;
use gui::imguidebug::ImguiDebug;

extern crate clap;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

mod lameboy;
use lameboy::Lameboy;

mod cart;
use cart::Cart;
mod mmu;
use mmu::MMU;
mod cpu;
use cpu::CPU;
mod ppu;
use ppu::PPU;
mod joypad;
use joypad::Joypad;

const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");
const PKG_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);

fn main() {
    let matches = clap::App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author("Nick Palmer <nick@palmr.co.uk>")
        .about(PKG_DESCRIPTION)
        .arg(clap::Arg::with_name("file")
            .help("ROM file to load")
            .required(false))
        .get_matches();

    let rom_file = matches.value_of("file").unwrap_or("roms/tetris.gb");
    let rom_file_name = Path::new(rom_file).file_name().unwrap().to_str().unwrap();
    println!("Filename: {}", rom_file_name);

    let mut data = Vec::new();
    let mut f = File::open(rom_file).expect("Unable to open ROM");
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("File length: {}", data.len());

    let mut gui = GUI::init((640, 576), rom_file_name);

    // Create all our hardware instances
    let mut joypad = Joypad::new();
    let mut cart = Cart::new(data);
    let mut ppu = PPU::new(&gui.display);
    let mut mmu = MMU::new(&mut cart, &mut ppu, &mut joypad);
    let cpu = CPU::new(&mut mmu);

    let mut lameboy = Lameboy::new(cpu);
    lameboy.reset();

    let mut imgui_debug = ImguiDebug::new();

    loop {
        if lameboy.is_running() {
            lameboy.run_frame();
        }

        gui.update_events(&mut imgui_debug, &mut lameboy);

        if !imgui_debug.active {
            break;
        }

        gui.render(CLEAR_COLOR,
           &mut lameboy,
           |ui, emulator| {
               imgui_debug.draw(ui, emulator);
           }
        );
    }
}
