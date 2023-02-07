use crate::lameboy::cpu::Cpu;
use imgui::{Condition, Ui};

pub fn stack_window(cpu: &Cpu, ui: &Ui) {
    ui.window("CPU - Stack")
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            let display_stack_entry_count = 50;
            let stack_addr_bottom = cpu.registers.sp.wrapping_sub(2);
            let stack_addr_top = stack_addr_bottom.wrapping_add(display_stack_entry_count * 2);

            let mut stack_addr = stack_addr_top;
            while stack_addr > stack_addr_bottom {
                ui.text_colored([0.7, 0.7, 0.7, 1.0], format!("[0x{stack_addr:04X}]"));
                ui.same_line();
                ui.text(" - ");
                ui.same_line();
                ui.text_colored(
                    [1.0, 1.0, 0.0, 1.0],
                    format!("0x{:04X}", cpu.mmu.read16_safe(stack_addr)),
                );

                stack_addr = stack_addr.wrapping_sub(2);
            }
        });
}
