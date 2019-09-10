use dis;
use dis::{Instruction, InstructionArg};
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

            let context_size = 100;

            let mut instruction_addr: u16 = if imgui_debug.disassemble_memory_pc_lock {
                imgui_debug.program_counter
            } else {
                imgui_debug.disassemble_memory_addr as u16
            };

            for _ in 0..context_size {
                let opcode = mmu.read8_safe(instruction_addr);
                let instruction = dis::decode_instruction(opcode);

                let instruction_debug_str = get_instruction_debug_string(
                    &instruction,
                    imgui_debug.disassemble_read_args,
                    mmu,
                    instruction_addr,
                );

                let addr_string = im_str!("[0x{:04X}] ", instruction_addr);
                let disassembly_string = im_str!("{}", instruction_debug_str);

                let style = if imgui_debug.breakpoints.contains(&instruction_addr) {
                    ui.push_style_color(StyleColor::Text, [1.0, 0.4, 0.4, 1.0])
                } else {
                    ui.push_style_color(StyleColor::Text, [0.7, 0.7, 0.7, 1.0])
                };
                if Selectable::new(&addr_string).selected(instruction_addr == imgui_debug.program_counter).build(ui) {
                    match imgui_debug
                        .breakpoints
                        .iter()
                        .position(|&r| r == instruction_addr)
                    {
                        None => {
                            imgui_debug.breakpoints.push(instruction_addr);
                        }
                        Some(idx) => {
                            imgui_debug.breakpoints.remove(idx);
                        }
                    }
                }
                style.pop(ui);

                ui.same_line(0.0);

                ui.text(disassembly_string);

                if let Some(memory_comment) = dis::get_memory_comment(&instruction_addr) {
                    ui.same_line(0.0);
                    ui.text_colored([0.5, 0.5, 0.5, 1.0], im_str!(" ; {}", memory_comment));
                }

                instruction_addr =
                    instruction_addr.wrapping_add(u16::from(instruction.get_length()));
            }
        });
}

fn get_instruction_debug_string(
    instruction: &Instruction,
    read_args: bool,
    mmu: &MMU,
    instruction_addr: u16,
) -> String {
    if read_args {
        match &instruction.arg {
            None => String::from(instruction.name),
            Some(arg) => match arg {
                InstructionArg::Data8 => {
                    let formatted_arg = format!("0x{:02X}", mmu.read8_safe(instruction_addr + 1));
                    let formatted_addr_arg =
                        format!("0XFF00 + 0x{:02X}", mmu.read8_safe(instruction_addr + 1));
                    String::from(instruction.name)
                        .replace("d8", formatted_arg.as_str())
                        .replace("r8", formatted_arg.as_str())
                        .replace("a8", formatted_addr_arg.as_str())
                }
                InstructionArg::Data16 => {
                    let formatted_arg = format!("0x{:04X}", mmu.read16_safe(instruction_addr + 1));
                    String::from(instruction.name)
                        .replace("d16", formatted_arg.as_str())
                        .replace("a16", formatted_arg.as_str())
                }
            },
        }
    } else {
        String::from(instruction.name)
    }
}
