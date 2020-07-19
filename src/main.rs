#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate clap;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate nalgebra;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use gui::GUI;
use lameboy::Lameboy;

mod dis;
mod gui;

mod lameboy;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);

fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let matches = clap::App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author("Nick Palmer <nick@palmr.co.uk>")
        .about(PKG_DESCRIPTION)
        .arg(
            clap::Arg::with_name("file")
                .help("ROM file to load")
                .required(false),
        )
        .get_matches();

    let rom_file = matches.value_of("file").unwrap_or("roms/tetris.gb");
    let rom_file_name = Path::new(rom_file).file_name().unwrap().to_str().unwrap();
    info!("Filename: {}", rom_file_name);

    let mut data = Vec::new();
    let mut f = File::open(rom_file).expect("Unable to open ROM");
    f.read_to_end(&mut data).expect("Unable to read data");
    info!("File length: {}", data.len());

    let window_title = format!("{} - Lameboy - v{}", rom_file_name, PKG_VERSION);
    let gui = GUI::init((640f64, 576f64), window_title, CLEAR_COLOR);

    // Create all our hardware instances
    let mut lameboy = Lameboy::new(data, &gui);
    lameboy.reset();

    gui.main_loop(lameboy);
}
