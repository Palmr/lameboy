#![allow(dead_code)]

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;

extern crate imgui_sys;

extern crate nalgebra;

use imgui::*;

mod gui;
use self::gui::GUI;

mod ppu;
use ppu::{PPU, TestPattern};

extern crate clap;

use std::io::prelude::*;
use std::fs::File;

mod memoryeditor;

mod cart;
use cart::Cart;
mod mmu;
mod cpu;
use cpu::CPU;

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);

pub struct GUIState {
    active: bool,
    emulator_running: bool,
    show_imgui_metrics: bool,
    show_menu: bool,
    show_memory: bool,
    mem_editor: memoryeditor::HexEditor,
    show_cpu: bool,
    show_vram: bool,
    show_about: bool,
    i0: i32
}

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

    let cart = Cart::new(data);

    let mut cpu = CPU::new(&cart);
    cpu.post_boot_reset();

    let mut gui = GUI::init((640, 576));

    let mut ppu = PPU::new(&gui.display);

    let mut gui_state = GUIState{
        active: true,
        emulator_running: false,
        show_imgui_metrics: false,
        show_menu: false,
        show_memory: false,
        mem_editor: memoryeditor::HexEditor::default(),
        show_cpu: false,
        show_vram: false,
        show_about: false,
        i0: 0
    };

    loop {
        if gui_state.emulator_running {
            let mut t_clk: u32 = 0;
            while t_clk < 70224 {
                t_clk += cpu.cycle() as u32;
            }
        }

        gui.update_events(&mut gui_state);

        if !gui_state.active {
            break;
        }

        gui.render(CLEAR_COLOR,
           &mut ppu,
           |ui, mut ppu| {
               imgui_display(ui, &cart, &mut cpu, &mut ppu, &mut gui_state);
           }
        );
    }
}

fn imgui_display<'a>(ui: &Ui<'a>, cart: &Cart, cpu: &mut CPU, ppu: &mut PPU, mut gui_state: &mut GUIState) {
    if gui_state.show_menu {
        ui.main_menu_bar(|| {
            ui.menu(im_str!("File"))
                .build(|| {
                    ui.menu_item(im_str!("Open ROM"))
                        .selected(&mut gui_state.show_memory)
                        .build();
                    ui.menu_item(im_str!("Reload ROM"))
                        .selected(&mut gui_state.show_memory)
                        .build();
                    ui.menu_item(im_str!("Reset"))
                        .selected(&mut gui_state.show_memory)
                        .build();
                    ui.separator();
                    ui.menu_item(im_str!("Exit"))
                        .selected(&mut gui_state.active)
                        .build();
                });
            ui.menu(im_str!("Options"))
                .build(|| {});
            ui.menu(im_str!("Debug"))
                .build(|| {
                    ui.menu_item(im_str!("Memory"))
                        .selected(&mut gui_state.show_memory)
                        .build();
                    ui.menu_item(im_str!("CPU"))
                        .selected(&mut gui_state.show_cpu)
                        .build();
                    ui.menu_item(im_str!("vram"))
                        .selected(&mut gui_state.show_vram)
                        .build();
                });
            ui.menu(im_str!("Help"))
                .build(|| {
                    ui.menu_item(im_str!("About"))
                        .selected(&mut gui_state.show_about)
                        .build();
                    ui.menu_item(im_str!("ImGUI Metrics"))
                        .selected(&mut gui_state.show_imgui_metrics)
                        .build();
                });
        });
    }

    if gui_state.show_imgui_metrics {
        ui.show_metrics_window(&mut gui_state.show_imgui_metrics);
    }

    if gui_state.show_memory {
        ui.window(im_str!("Cart"))
            .size((200.0, 125.0), ImGuiSetCond_Always)
            .resizable(false)
            .build(|| {
                ui.text(im_str!("Size: {} bytes", cart.get_size()));
                ui.text(im_str!("Title: {}", cart.get_title()));
                ui.text(im_str!("Checksum: {}", if cart.validate_checksum() { "VALID" } else { "INVALID" }));

                ui.separator();

                ui.input_int(im_str!("Addr"), &mut gui_state.i0).build();
                if ui.small_button(im_str!("print")) {
                    let byte = cpu.mmu.read8(gui_state.i0 as u16);
                    println!("Memory[{:04X}] = {:02X}", gui_state.i0, byte);
                }
            });
        //gui_state.mem_editor.render(ui, "Memory Editor", &data);
    }

    if gui_state.show_cpu {
        ui.window(im_str!("CPU"))
            .size((260.0, 175.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                ui.text(im_str!("PC: 0x{:04X} - SP: 0x{:04X}", cpu.registers.pc, cpu.registers.sp));
                ui.text(im_str!(" A: 0x{:02X}   -  B: 0x{:02X}", cpu.registers.a, cpu.registers.b));
                ui.text(im_str!(" C: 0x{:02X}   -  D: 0x{:02X}", cpu.registers.c, cpu.registers.d));
                ui.text(im_str!(" E: 0x{:02X}   -  F: 0x{:02X}", cpu.registers.e, cpu.registers.f.bits()));
                ui.text(im_str!(" H: 0x{:02X}   -  L: 0x{:02X}", cpu.registers.h, cpu.registers.l));
                ui.text(im_str!("Flags: {:?}", cpu.registers.f));
                ui.separator();
                ui.checkbox(im_str!("running"), &mut gui_state.emulator_running);
                if ui.small_button(im_str!("step")) {
                    cpu.cycle();
                }
            });
    }

    if gui_state.show_vram {
        ui.window(im_str!("PPU"))
            .size((150.0, 85.0), ImGuiSetCond_FirstUseEver)
            .resizable(true)
            .build(|| {
                if ui.small_button(im_str!("Blank")) {
                    ppu.testing(TestPattern::BLANK);
                }
                if ui.small_button(im_str!("Diagonal")) {
                    ppu.testing(TestPattern::DIAGONAL);
                }
                if ui.small_button(im_str!("XOR")) {
                    ppu.testing(TestPattern::XOR);
                }
            });
    }
}
