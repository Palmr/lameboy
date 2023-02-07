use imgui::{Condition, Ui, Window};
use lameboy::cpu::Cpu;

pub fn registers_window(cpu: &Cpu, ui: &Ui) {
    Window::new(im_str!("CPU - Registers"))
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(ui, || {
            ui.text(im_str!(
                "PC: 0x{:04X} - SP: 0x{:04X}",
                cpu.registers.pc,
                cpu.registers.sp
            ));
            ui.text(im_str!(
                " A: 0x{:02X}   -  F: 0x{:02X}",
                cpu.registers.a,
                cpu.registers.f.bits()
            ));
            ui.text(im_str!(
                " B: 0x{:02X}   -  C: 0x{:02X}",
                cpu.registers.b,
                cpu.registers.c
            ));
            ui.text(im_str!(
                " D: 0x{:02X}   -  E: 0x{:02X}",
                cpu.registers.d,
                cpu.registers.e
            ));
            ui.text(im_str!(
                " H: 0x{:02X}   -  L: 0x{:02X}",
                cpu.registers.h,
                cpu.registers.l
            ));
            ui.text(im_str!("Flags: {:?}", cpu.registers.f));
        });
}
