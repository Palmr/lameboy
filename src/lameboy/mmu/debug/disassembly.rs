use crate::dis;
use crate::dis::{Instruction, InstructionArg};
use crate::gui::imgui_debug_state::ImguiDebugState;
use crate::lameboy::mmu::Mmu;
use imgui::{Condition, StyleColor, Ui};

pub fn disassembly_window(mmu: &Mmu, ui: &Ui, imgui_debug: &mut ImguiDebugState) {
    ui.window("Disassembled code")
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            ui.checkbox("Lock to PC", &mut imgui_debug.disassemble_memory_pc_lock);
            ui.same_line();
            ui.checkbox("Read Args", &mut imgui_debug.disassemble_read_args);

            ui.input_int("Addr", &mut imgui_debug.disassemble_memory_addr)
                .chars_hexadecimal(true)
                .build();
            ui.separator();

            let context_size = 100;

            let mut instruction_addr: u16 = if imgui_debug.disassemble_memory_pc_lock {
                imgui_debug.disassemble_memory_addr = i32::from(imgui_debug.program_counter);
                imgui_debug.program_counter
            } else {
                imgui_debug.disassemble_memory_addr as u16
            };

            for _ in 0..context_size {
                let instruction = dis::decode_instruction(instruction_addr, mmu);

                let raw_instruction_debug_string =
                    get_raw_instruction_debug_string(&instruction, mmu, instruction_addr);
                let instruction_debug_str = get_instruction_debug_string(
                    &instruction,
                    imgui_debug.disassemble_read_args,
                    mmu,
                    instruction_addr,
                );

                let addr_string = format!("[0x{instruction_addr:04X}]");
                let disassembly_string =
                    format!("{raw_instruction_debug_string: <14} | {instruction_debug_str}");

                let style = if imgui_debug.breakpoints.contains(&instruction_addr) {
                    ui.push_style_color(StyleColor::Text, [1.0, 0.4, 0.4, 1.0])
                } else {
                    ui.push_style_color(StyleColor::Text, [0.7, 0.7, 0.7, 1.0])
                };
                if ui
                    .selectable_config(&addr_string)
                    .selected(instruction_addr == imgui_debug.program_counter)
                    .build()
                {
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
                style.pop();

                ui.same_line();

                ui.text(disassembly_string);

                if let Some(memory_comment) = dis::get_memory_comment(instruction_addr) {
                    ui.same_line();
                    ui.text_colored([0.5, 0.5, 0.5, 1.0], format!(" ; {memory_comment}"));
                }

                instruction_addr =
                    instruction_addr.wrapping_add(u16::from(instruction.get_length()));
            }
        });
}

fn get_raw_instruction_debug_string(
    instruction: &Instruction,
    mmu: &Mmu,
    instruction_addr: u16,
) -> String {
    format!(
        "0x{:02X}{}",
        mmu.read8_safe(instruction_addr),
        match &instruction.arg {
            None => String::new(),
            Some(arg) => match arg {
                InstructionArg::Data8 => format!(" 0x{:02X}", mmu.read8_safe(instruction_addr + 1)),
                InstructionArg::Data16 => format!(
                    " 0x{:02X} 0x{:02X}",
                    mmu.read8_safe(instruction_addr + 1),
                    mmu.read8_safe(instruction_addr + 2)
                ),
            },
        }
    )
}

fn get_instruction_debug_string(
    instruction: &Instruction,
    read_args: bool,
    mmu: &Mmu,
    instruction_addr: u16,
) -> String {
    if read_args {
        match &instruction.arg {
            None => String::from(instruction.name),
            Some(arg) => match arg {
                InstructionArg::Data8 => {
                    let formatted_arg = format!("0x{:02X}", mmu.read8_safe(instruction_addr + 1));
                    let formatted_addr_arg =
                        format!("0xFF00 + 0x{:02X}", mmu.read8_safe(instruction_addr + 1));
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
