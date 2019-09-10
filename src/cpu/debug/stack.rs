use imgui::{Condition, Ui, Window};
use cpu::CPU;

pub fn stack_window<'a>(cpu: &CPU, ui: &Ui<'a>) {
    Window::new(im_str!("CPU - Stack"))
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            let display_stack_entry_count = 50;
            let stack_addr_bottom = cpu.registers.sp.wrapping_sub(2);
            let stack_addr_top = stack_addr_bottom.wrapping_add(display_stack_entry_count * 2);

            let mut stack_addr = stack_addr_top;
            while stack_addr > stack_addr_bottom {
                ui.text_colored([0.7, 0.7, 0.7, 1.0], im_str!("[0x{:04X}]", stack_addr));
                ui.same_line(0.0);
                ui.text(im_str!(" - "));
                ui.same_line(0.0);
                ui.text_colored(
                    [1.0, 1.0, 0.0, 1.0],
                    im_str!("0x{:04X}", cpu.mmu.read16_safe(stack_addr)),
                );

                stack_addr = stack_addr.wrapping_sub(2);
            }
        });
}