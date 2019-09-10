use gui::imgui_debug_state::ImguiDebugState;
use imgui::{Condition, Selectable, StyleColor, Ui, Window};
use lameboy::mmu::MMU;

pub fn hexdump_window<'a>(mmu: &MMU, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState) {
    Window::new(im_str!("MMU - dump"))
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            ui.checkbox(im_str!("Lock to PC"), &mut imgui_debug.dump_memory_pc_lock);
            ui.same_line(0.0);
            ui.input_int(im_str!("Addr"), &mut imgui_debug.dump_memory_addr)
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

                ui.text_colored([0.7, 0.7, 0.7, 1.0], im_str!("[0x{:04X}]", row_addr));

                for offset in 0..bytes_per_row {
                    let mem_ptr = row_addr + offset;
                    let colour = if mem_ptr == dump_memory_addr {
                        [0.5, 1.0, 0.5, 1.0]
                    } else if imgui_debug.memory_breakpoints.contains(&mem_ptr) {
                        [1.0, 0.4, 0.4, 1.0]
                    } else {
                        [0.8, 0.8, 0.8, 1.0]
                    };

                    ui.same_line(0.0);

                    let style = ui.push_style_color(StyleColor::Text, colour);
                    if Selectable::new(&im_str!("{:02X}", mmu.read8_safe(mem_ptr)))
                        .size([11.0, 11.0])
                        .build(ui)
                    {
                        selected_mem_ptr = Some(mem_ptr);
                    }
                    style.pop(ui);
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
