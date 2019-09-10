use dis;
use dis::InstructionArg;
use gui::imgui_debug_state::ImguiDebugState;
use imgui::{Condition, Selectable, StyleColor, Ui, Window};
use lameboy::mmu::MMU;

pub fn disassembly_window<'a>(mmu: &MMU, ui: &Ui<'a>, imgui_debug: &mut ImguiDebugState) {
    Window::new(im_str!("Disassembled code"))
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            ui.checkbox(
                im_str!("Lock to PC"),
                &mut imgui_debug.disassemble_memory_pc_lock,
            );
            ui.same_line(0.0);
            ui.checkbox(im_str!("Read Args"), &mut imgui_debug.disassemble_read_args);
            ui.same_line(0.0);
            ui.input_int(im_str!("Addr"), &mut imgui_debug.disassemble_memory_addr)
                .chars_hexadecimal(true)
                .build();
            ui.separator();

            let context_size = 20;

            let mut dump_memory_addr: u16 = if imgui_debug.disassemble_memory_pc_lock {
                imgui_debug.program_counter
            } else {
                imgui_debug.disassemble_memory_addr as u16
            };

            for _ in 0..context_size {
                let opcode = mmu.read8_safe(dump_memory_addr);
                let instruction = dis::decode_instruction(opcode);

                let instruction_name = String::from(instruction.name);
                let instruction_debug_str: String = if imgui_debug.disassemble_read_args {
                    match &instruction.arg {
                        None => instruction_name,
                        Some(arg) => match arg {
                            InstructionArg::Data8 => {
                                let formatted_arg =
                                    format!("0x{:02X}", mmu.read8_safe(dump_memory_addr + 1));
                                let formatted_addr_arg = format!(
                                    "0XFF00 + 0x{:02X}",
                                    mmu.read8_safe(dump_memory_addr + 1)
                                );
                                instruction_name
                                    .replace("d8", formatted_arg.as_str())
                                    .replace("r8", formatted_arg.as_str())
                                    .replace("a8", formatted_addr_arg.as_str())
                            }
                            InstructionArg::Data16 => {
                                let formatted_arg =
                                    format!("0x{:04X}", mmu.read16_safe(dump_memory_addr + 1));
                                instruction_name
                                    .replace("d16", formatted_arg.as_str())
                                    .replace("a16", formatted_arg.as_str())
                            }
                        },
                    }
                } else {
                    instruction_name
                };

                let style = if imgui_debug.breakpoints.contains(&dump_memory_addr) {
                    ui.push_style_color(StyleColor::Text, [1.0, 0.4, 0.4, 1.0])
                } else {
                    ui.push_style_color(StyleColor::Text, [0.7, 0.7, 0.7, 1.0])
                };
                if Selectable::new(&im_str!("[0x{:04X}] ", dump_memory_addr)).build(ui) {
                    match imgui_debug
                        .breakpoints
                        .iter()
                        .position(|&r| r == dump_memory_addr)
                    {
                        None => {
                            imgui_debug.breakpoints.push(dump_memory_addr);
                        }
                        Some(idx) => {
                            imgui_debug.breakpoints.remove(idx);
                        }
                    }
                }
                style.pop(ui);

                ui.same_line(0.0);
                ui.text(im_str!("{}", instruction_debug_str));

                dump_memory_addr += instruction.get_length() as u16;
            }
        });
}
