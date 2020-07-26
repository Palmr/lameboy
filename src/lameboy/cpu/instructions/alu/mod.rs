mod add;
mod and;
mod compare;
mod or;
mod sub;
mod swap;
mod xor;

pub use lameboy::cpu::instructions::alu::add::alu_add_8bit;
pub use lameboy::cpu::instructions::alu::and::alu_and_8bit;
pub use lameboy::cpu::instructions::alu::compare::alu_cp_8bit;
pub use lameboy::cpu::instructions::alu::or::alu_or_8bit;
pub use lameboy::cpu::instructions::alu::sub::alu_sub_8bit;
pub use lameboy::cpu::instructions::alu::swap::alu_swap;
pub use lameboy::cpu::instructions::alu::xor::alu_xor_8bit;

use lameboy::cpu::registers::Flags;
use lameboy::cpu::CPU;

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the Flags::CARRY flag and the old value of the
/// Flags::CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the Flags::CARRY flag.
///
/// If reset_zero is true the Flags::ZERO flag will always be reset.
/// If it is not true the Flags::ZERO flag will be set only if the rotated value equals zero.
///
pub fn alu_rotate_left(cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(Flags::CARRY) {
        1
    } else {
        0
    };
    let high_bit = (d8 & 0x80) >> 7;
    let new_low_bit = if through_carry { cy } else { high_bit };
    let rotated_value = (d8 << 1) | new_low_bit;

    cpu.registers
        .f
        .set(Flags::ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, high_bit != 0);

    rotated_value
}

/// Rotate an 8-bit value to the right.
///
/// If through_carry is true then the low bit will go into the Flags::CARRY flag and the old value of the
/// Flags::CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the Flags::CARRY flag.
///
/// If reset_zero is true the Flags::ZERO flag will always be reset.
/// If it is not true the Flags::ZERO flag will be set only if the rotated value equals zero.
///
pub fn alu_rotate_right(cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(Flags::CARRY) {
        1
    } else {
        0
    };
    let low_bit = d8 & 0x01;
    let new_high_bit = if through_carry { cy } else { low_bit };

    let rotated_value = (d8 >> 1) | (new_high_bit << 7);

    cpu.registers
        .f
        .set(Flags::ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, low_bit != 0);

    rotated_value
}

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
