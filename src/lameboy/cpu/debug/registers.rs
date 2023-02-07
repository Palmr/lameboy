use crate::lameboy::cpu::Cpu;
use imgui::{Condition, Ui};

pub fn registers_window(cpu: &Cpu, ui: &Ui) {
    ui.window("CPU - Registers")
        .size([260.0, 140.0], Condition::FirstUseEver)
        .resizable(true)
        .build(|| {
            ui.text(format!(
                "PC: 0x{:04X} - SP: 0x{:04X}",
                cpu.registers.pc, cpu.registers.sp
            ));
            ui.text(format!(
                " A: 0x{:02X}   -  F: 0x{:02X}",
                cpu.registers.a,
                cpu.registers.f.bits()
            ));
            ui.text(format!(
                " B: 0x{:02X}   -  C: 0x{:02X}",
                cpu.registers.b, cpu.registers.c
            ));
            ui.text(format!(
                " D: 0x{:02X}   -  E: 0x{:02X}",
                cpu.registers.d, cpu.registers.e
            ));
            ui.text(format!(
                " H: 0x{:02X}   -  L: 0x{:02X}",
                cpu.registers.h, cpu.registers.l
            ));
            ui.text(format!("Flags: {:?}", cpu.registers.f));
        });
}
