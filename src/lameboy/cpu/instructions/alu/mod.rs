mod add;
mod and;
mod compare;
mod or;
mod rotate;
mod sub;
mod swap;
mod xor;

pub use lameboy::cpu::instructions::alu::add::alu_add_8bit;
pub use lameboy::cpu::instructions::alu::and::alu_and_8bit;
pub use lameboy::cpu::instructions::alu::compare::alu_cp_8bit;
pub use lameboy::cpu::instructions::alu::or::alu_or_8bit;
pub use lameboy::cpu::instructions::alu::rotate::alu_rotate_left;
pub use lameboy::cpu::instructions::alu::rotate::alu_rotate_right;
pub use lameboy::cpu::instructions::alu::sub::alu_sub_8bit;
pub use lameboy::cpu::instructions::alu::swap::alu_swap_8bit;
pub use lameboy::cpu::instructions::alu::xor::alu_xor_8bit;

use lameboy::cpu::registers::Flags;
use lameboy::cpu::CPU;

/// Shift an 8-bit value to the left.
///
pub fn alu_shift_left(cpu: &mut CPU, d8: u8) -> u8 {
    let high_bit = d8 & 0x80;
    let shifted_value = d8 << 1;

    cpu.registers.f.set(Flags::ZERO, shifted_value == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, high_bit != 0);

    shifted_value
}

/// Shift an 8-bit value to the right.
///
pub fn alu_shift_right(cpu: &mut CPU, d8: u8, reset_high_bit: bool) -> u8 {
    let high_bit = if reset_high_bit { 0 } else { d8 & 0x80 };
    let low_bit = d8 & 0x01;
    let shifted_value = (d8 >> 1) | high_bit;

    cpu.registers.f.set(Flags::ZERO, shifted_value == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, low_bit != 0);

    shifted_value
}
