use imgui::Ui;

pub trait ImguiDebuggable {
    fn imgui_display<'a>(&mut self, ui: &Ui<'a>, imgui_debug: &mut ImguiDebug);
}

pub struct ImguiDebug {
    pub program_counter: u16,
    pub show_imgui_metrics: bool,
    pub show_menu: bool,
    pub show_emulator: bool,
    pub show_cart: bool,
    pub show_memory: bool,
    pub show_cpu: bool,
    pub show_ppu: bool,
    pub show_joypad: bool,
    pub ppu_mod: i32,
    pub ppu_sprite_index: i32,
    pub show_about: bool,
    pub input_breakpoint_addr: i32,
    pub input_memory_addr: i32,
    pub input_memory_value: i32,
    pub dump_memory_addr: i32,
    pub dump_memory_pc_lock: bool,
    pub disassemble_memory_addr: i32,
    pub disassemble_memory_pc_lock: bool,
    pub disassemble_read_args: bool,
    pub breakpoints: Vec<u16>,
    pub memory_breakpoints: Vec<u16>,
}

impl ImguiDebug {
    pub fn new() -> ImguiDebug {
        ImguiDebug {
            program_counter: 0,
            show_imgui_metrics: false,
            show_menu: false,
            show_emulator: true,
            show_cart: false,
            show_memory: false,
            show_cpu: false,
            show_ppu: false,
            show_joypad: false,
            ppu_mod: 4,
            ppu_sprite_index: 0,
            show_about: false,
            input_breakpoint_addr: 0,
            input_memory_addr: 0,
            input_memory_value: 0,
            dump_memory_addr: 0,
            dump_memory_pc_lock: true,
            disassemble_memory_addr: 0,
            disassemble_memory_pc_lock: true,
            disassemble_read_args: false,
            breakpoints: Vec::new(),
            memory_breakpoints: Vec::new(),
        }
    }
}
