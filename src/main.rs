#[macro_use]
extern crate glium;
#[macro_use]
extern crate imgui;

extern crate clap;

use imgui::*;

mod gui;
use self::gui::GUI;


use std::io::prelude::*;
use std::fs::File;
use std::path;
use std::fmt;


const CLEAR_COLOR: (f32, f32, f32, f32) = (0.5, 0.5, 0.5, 1.0);


fn main() {
    let matches = clap::App::new("rboy")
        .version("0.1.0")
        .author("Nick Palmer <nick@palmr.co.uk>")
        .about("Yet another Gameboy emulator")
        .arg(clap::Arg::with_name("file")
            .help("ROM file to load")
            .required(false))
        .get_matches();

    let rom_file = matches.value_of("file").unwrap_or("roms/tetris.gb");
    println!("Value for file: {}", rom_file);

    let mut data = Vec::new();
    let mut f = File::open(rom_file).expect("Unable to open ROM");
    f.read_to_end(&mut data).expect("Unable to read data");
    println!("File length: {}", data.len());


    let mut gui = GUI::init((320, 200));

    loop {
        gui.render(CLEAR_COLOR, hello_world);
        let active = gui.update_events();
        if !active {
            break;
        }
    }
}

fn hello_world<'a>(ui: &Ui<'a>) {
    ui.window(im_str!("Hello world"))
        .size((300.0, 100.0), ImGuiSetCond_FirstUseEver)
        .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
            ui.separator();
            let mouse_pos = ui.imgui().mouse_pos();
            ui.text(im_str!("Mouse Position: ({:.1},{:.1})", mouse_pos.0, mouse_pos.1));
        })
}
