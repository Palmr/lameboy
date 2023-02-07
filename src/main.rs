#![allow(dead_code)]

#[macro_use]
extern crate bitflags;
extern crate clap;
#[macro_use]
extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
#[macro_use]
extern crate log;
extern crate core;
extern crate log4rs;
extern crate nalgebra;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use crate::gui::Gui;
use crate::lameboy::Lameboy;
use clap::Parser;

mod dis;
mod gui;

mod lameboy;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// ROM file to load
    file: String,
}

fn main() {
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();

    let args = Args::parse();

    let rom_file = args.file.as_str();
    let rom_file_name = Path::new(rom_file).file_name().unwrap().to_str().unwrap();
    info!("Filename: {}", rom_file_name);

    let mut data = Vec::new();
    let mut f = File::open(rom_file).expect("Unable to open ROM");
    f.read_to_end(&mut data).expect("Unable to read data");
    info!("File length: {}", data.len());

    let window_title = format!("{rom_file_name} - Lameboy - v{PKG_VERSION}");
    let gui = Gui::init((640f64, 576f64), window_title, CLEAR_COLOR);

    // Create all our hardware instances
    let mut lameboy = Lameboy::new(data, &gui);
    lameboy.reset();

    gui.main_loop(lameboy);
}
