use crate::gui::imgui_debug_state::ImguiDebugState;
use imgui::{Condition, Ui};

pub fn breakpoint_windows(ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    ui.window("Breakpoints")
        .size([225.0, 150.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            if ui.button("Set") {
                let breakpoint_addr = imgui_debug.input_breakpoint_addr as u16;
                if !imgui_debug.breakpoints.contains(&breakpoint_addr) {
                    imgui_debug.breakpoints.push(breakpoint_addr);
                }
            }
            ui.same_line();
            ui.input_int("Addr", &mut imgui_debug.input_breakpoint_addr)
                .chars_hexadecimal(true)
                .build();

            if ui.button("Clear All") {
                imgui_debug.breakpoints.clear();
            }

            let mut removal_index: Option<usize> = None;
            ui.text("Breakpoints:");
            ui.separator();
            if imgui_debug.breakpoints.is_empty() {
                ui.text("None yet");
            } else {
                for index in 0..imgui_debug.breakpoints.len() {
                    if ui.selectable(format!("0x{:04X}", imgui_debug.breakpoints[index])) {
                        removal_index = Some(index);
                    };
                }
            }
            if let Some(index) = removal_index {
                imgui_debug.breakpoints.remove(index);
            }
        });

    ui.window("Memory Breakpoints")
        .size([225.0, 150.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            if ui.button("Set") {
                let breakpoint_addr = imgui_debug.input_breakpoint_addr as u16;
                if !imgui_debug.memory_breakpoints.contains(&breakpoint_addr) {
                    imgui_debug.memory_breakpoints.push(breakpoint_addr);
                }
            }
            ui.same_line();
            ui.input_int("Addr", &mut imgui_debug.input_breakpoint_addr)
                .chars_hexadecimal(true)
                .build();

            if ui.button("Clear All") {
                imgui_debug.memory_breakpoints.clear();
            }

            let mut removal_index: Option<usize> = None;
            ui.text("Breakpoints:");
            ui.separator();
            if imgui_debug.memory_breakpoints.is_empty() {
                ui.text("None yet");
            } else {
                for index in 0..imgui_debug.memory_breakpoints.len() {
                    if ui.selectable(format!("0x{:04X}", imgui_debug.memory_breakpoints[index])) {
                        removal_index = Some(index);
                    };
                }
            }
            if let Some(index) = removal_index {
                imgui_debug.memory_breakpoints.remove(index);
            }
        });
}
