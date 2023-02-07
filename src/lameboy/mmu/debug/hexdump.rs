use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::lameboy::mmu::Mmu;
use imgui::{Condition, StyleColor, Ui};

pub fn hexdump_window(mmu: &Mmu, ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    ui.window("MMU - dump")
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            ui.checkbox("Lock to PC", &mut imgui_debug.dump_memory_pc_lock);
            ui.same_line();
            ui.input_int("Addr", &mut imgui_debug.dump_memory_addr)
                .chars_hexadecimal(true)
                .build();
            ui.separator();

            let bytes_per_row = 16;
            let context_size = 5;

            let dump_memory_addr: u16 = if imgui_debug.dump_memory_pc_lock {
                imgui_debug.program_counter
            } else {
                imgui_debug.dump_memory_addr as u16
            };

            let memory_addr_row = dump_memory_addr - (dump_memory_addr % bytes_per_row);

            let mut memory_addr_low = memory_addr_row.wrapping_sub(context_size * bytes_per_row);
            let memory_addr_high = memory_addr_row.wrapping_add(context_size * bytes_per_row);

            if memory_addr_low > memory_addr_high {
                memory_addr_low = 0;
            }

            let mut selected_mem_ptr = None;
            for row in 0..=(context_size * 2) {
                let row_addr = memory_addr_low + row * bytes_per_row;

                ui.text_colored([0.7, 0.7, 0.7, 1.0], format!("[0x{row_addr:04X}]"));

                for offset in 0..bytes_per_row {
                    let mem_ptr = row_addr + offset;
                    let colour = if mem_ptr == dump_memory_addr {
                        [0.5, 1.0, 0.5, 1.0]
                    } else if imgui_debug.memory_breakpoints.contains(&mem_ptr) {
                        [1.0, 0.4, 0.4, 1.0]
                    } else {
                        [0.8, 0.8, 0.8, 1.0]
                    };

                    ui.same_line();

                    let style = ui.push_style_color(StyleColor::Text, colour);
                    if ui
                        .selectable_config(format!("{:02X}", mmu.read8_safe(mem_ptr)))
                        .size([11.0, 11.0])
                        .build()
                    {
                        selected_mem_ptr = Some(mem_ptr);
                    }
                    style.pop();
                }
            }

            if let Some(mem_ptr) = selected_mem_ptr {
                match imgui_debug
                    .memory_breakpoints
                    .iter()
                    .position(|&r| r == mem_ptr)
                {
                    None => {
                        imgui_debug.memory_breakpoints.push(mem_ptr);
                    }
                    Some(idx) => {
                        imgui_debug.memory_breakpoints.remove(idx);
                    }
                }
            }
        });
}
