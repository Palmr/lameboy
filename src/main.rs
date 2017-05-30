#![allow(dead_code)]

#[macro_use]
extern crate glium;

#[macro_use]
extern crate imgui;

extern crate imgui_sys;

use imgui::*;

mod gui;
use self::gui::GUI;

extern crate clap;

use std::io::prelude::*;
use std::fs::File;

mod memoryeditor;

mod cart;
use cart::Cart;

// glium
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);
use glium::Surface;
//

const CLEAR_COLOR: (f32, f32, f32, f32) = (0.8784, 0.9725, 0.8156, 1.0);

struct GUIState {
    show_imgui_metrics: bool,
    show_memory: bool,
    mem_editor: memoryeditor::HexEditor,
    show_cpu: bool,
    show_vram: bool,
    show_misc: bool,
    test_str: String,
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

    let mut gui = GUI::init((640, 576));

    // glutin testing
    let vertex1 = Vertex { position: [-0.5, -0.5] };
    let vertex2 = Vertex { position: [ 0.0,  0.5] };
    let vertex3 = Vertex { position: [ 0.5, -0.5] };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&mut gui.display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

let vertex_shader_src = r#"
    #version 140

    in vec2 position;

    void main() {
        gl_Position = vec4(position, 0.0, 1.0);
    }
"#;
let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(0.2039, 0.4078, 0.3372, 1.0);
    }
"#;

    let program = glium::Program::from_source(&mut gui.display, vertex_shader_src, fragment_shader_src, None).unwrap();
    //

    let mut gui_state = GUIState{
        show_imgui_metrics: false,
        show_memory: false,
        mem_editor: memoryeditor::HexEditor::default(),
        show_cpu: false,
        show_vram: false,
        show_misc: false,
        test_str: String::from("Hello world!"),
        i0: 0
    };

    loop {
        gui.render(CLEAR_COLOR,
           |t| {
               t.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms, &Default::default()).unwrap()
           },
           |ui| {
               imgui_display(ui, &cart, &mut gui_state);
           }
        );

        let active = gui.update_events();
        if !active {
            break;
        }
    }
}

fn imgui_display<'a>(ui: &Ui<'a>, cart: &Cart, mut gui_state: &mut GUIState) {
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
                ui.checkbox(im_str!("test"), &mut gui_state.show_memory);
            });
        ui.menu(im_str!("Options"))
            .build(|| {  });
        ui.menu(im_str!("Debug"))
            .build(|| {
                ui.menu_item(im_str!("Memory"))
                    .selected(&mut gui_state.show_memory)
                    .build();
                ui.menu_item(im_str!("CPU"))
                    .selected(&mut gui_state.show_cpu)
                    .build();
                ui.menu_item(im_str!("VRAM"))
                    .selected(&mut gui_state.show_vram)
                    .build();
            });
        ui.menu(im_str!("Help"))
            .build(|| {
                ui.menu_item(im_str!("Misc"))
                    .selected(&mut gui_state.show_misc)
                    .build();
                ui.menu_item(im_str!("ImGUI Metrics"))
                    .selected(&mut gui_state.show_imgui_metrics)
                    .build();
            });
    });

    if gui_state.show_misc {
        ui.window(im_str!("Hello world"))
            .size((300.0, 200.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.input_text(im_str!("=buf"), &mut gui_state.test_str).build();
            });
        ui.window(im_str!("buf-display"))
            .size((100.0, 100.0), ImGuiSetCond_FirstUseEver)
            .build(|| {
                ui.text(im_str!("buf={}", gui_state.test_str));
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
                    let byte = cart.read(gui_state.i0 as u16);
                    println!("Memory[{:04X}] = {:02X}", gui_state.i0, byte);
                }
            });
        //gui_state.mem_editor.render(ui, "Memory Editor", &data);
    }
}
