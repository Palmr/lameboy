use gui::imguidebug::ImguiDebug;
use imgui::{Condition, Selectable, Ui, Window};

pub fn breakpoint_windows<'a>(ui: &Ui<'a>, imgui_debug: &mut ImguiDebug) {
    Window::new(im_str!("Breakpoints"))
        .size([225.0, 150.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            if ui.button(im_str!("Set"), [0.0, 0.0]) {
                let breakpoint_addr = imgui_debug.input_breakpoint_addr as u16;
                if !imgui_debug.breakpoints.contains(&breakpoint_addr) {
                    imgui_debug.breakpoints.push(breakpoint_addr);
                }
            }
            ui.same_line(0.0);
            ui.input_int(im_str!("Addr"), &mut imgui_debug.input_breakpoint_addr)
                .chars_hexadecimal(true)
                .build();

            if ui.button(im_str!("Clear All"), [0.0, 0.0]) {
                imgui_debug.breakpoints.clear();
            }

            let mut removal_index: Option<usize> = None;
            ui.text(im_str!("Breakpoints:"));
            ui.separator();
            if imgui_debug.breakpoints.is_empty() {
                ui.text(im_str!("None yet"));
            } else {
                for index in 0..imgui_debug.breakpoints.len() {
                    if Selectable::new(&im_str!("0x{:04X}", imgui_debug.breakpoints[index]))
                        .build(ui)
                    {
                        removal_index = Some(index);
                    };
                }
            }
            if let Some(index) = removal_index {
                imgui_debug.breakpoints.remove(index);
            }
        });

    Window::new(im_str!("Memory Breakpoints"))
        .size([225.0, 150.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            if ui.button(im_str!("Set"), [0.0, 0.0]) {
                let breakpoint_addr = imgui_debug.input_breakpoint_addr as u16;
                if !imgui_debug.memory_breakpoints.contains(&breakpoint_addr) {
                    imgui_debug.memory_breakpoints.push(breakpoint_addr);
                }
            }
            ui.same_line(0.0);
            ui.input_int(im_str!("Addr"), &mut imgui_debug.input_breakpoint_addr)
                .chars_hexadecimal(true)
                .build();

            if ui.button(im_str!("Clear All"), [0.0, 0.0]) {
                imgui_debug.memory_breakpoints.clear();
            }

            let mut removal_index: Option<usize> = None;
            ui.text(im_str!("Breakpoints:"));
            ui.separator();
            if imgui_debug.memory_breakpoints.is_empty() {
                ui.text(im_str!("None yet"));
            } else {
                for index in 0..imgui_debug.memory_breakpoints.len() {
                    if Selectable::new(&im_str!("0x{:04X}", imgui_debug.memory_breakpoints[index]))
                        .build(ui)
                    {
                        removal_index = Some(index);
                    };
                }
            }
            if let Some(index) = removal_index {
                imgui_debug.memory_breakpoints.remove(index);
            }
        });
}
