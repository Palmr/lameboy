#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;

extern crate imgui_sys;

extern crate nalgebra;

mod gui;
use gui::GUI;
use gui::imguidebug::ImguiDebug;

extern crate clap;

use std::io::prelude::*;
use std::fs::File;

mod memoryeditor;

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

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);


fn main() {
    let matches = clap::App::new("rboy")
        .version("0.1.0")
        .author("Nick Palmer <nick@palmr.co.uk>")
        .about("Yet another Gameboy emulator")
        .arg(clap::Arg::with_name("file")
            .help("ROM file to load")
            .required(false))
        .get_matches();

    let mut gui = GUI::init((640, 576));

    let rom_file = matches.value_of("file").unwrap_or("roms/tetris.gb");
    println!("Value for file: {}", rom_file);

    let mut data = Vec::new();
    let mut f = File::open(rom_file).expect("Unable to open ROM");
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("File length: {}", data.len());

    let mut cart = Cart::new(data);
    let mut ppu = PPU::new(&gui.display);
    let mut mmu = MMU::new(&mut cart, &mut ppu);
    let cpu = CPU::new(&mut mmu);

    let mut lameboy = Lameboy::new(cpu);

    let mut imgui_debug = ImguiDebug::new();

    loop {
        if imgui_debug.emulator_running {
            lameboy.test_cycles();
        }

        gui.update_events(&mut imgui_debug);

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
