use lameboy::cpu::registers::Flags;
use lameboy::cpu::CPU;

/// Add 8-bit value with register A, storing the result in A.
/// If use_carry is true this will add the content of the carry flag along with the value and take
/// that into account when updating flags too.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always unset
///
/// ## Flags::HALF_CARRY
///
/// Set if the lower nibble of the value added to the lower nibble of A was too large to fit in a u4
///
/// ## Flags::CARRY
///
/// Set if the value added to A would have been too large to fit in a u8
///
pub fn alu_add_8bit(accumulator: u8, flags: Flags, d8: u8, use_carry: bool) -> (u8, Flags) {
    let cy = if use_carry && flags.contains(Flags::CARRY) {
        1
    } else {
        0
    };

    let new_accumulator = accumulator.wrapping_add(d8).wrapping_add(cy);

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, new_accumulator == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(
        Flags::HALF_CARRY,
        ((accumulator & 0x0F) + (d8 & 0x0F) + cy) > 0x0F,
    );
    new_flags.set(Flags::CARRY, new_accumulator < accumulator);

    (new_accumulator, new_flags)
}

#[cfg(test)]
mod test_alu_add_8bit {
    use lameboy::cpu::instructions::alu::alu_add_8bit;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_add_8bit(0x00, Flags::empty(), 0x01, false),
            (0x01, Flags::empty())
        );
    }

    #[test]
    fn check_reset_subtract() {
        assert_eq!(
            alu_add_8bit(0x00, Flags::SUBTRACT, 0x01, false),
            (0x01, Flags::empty())
        );
    }

    #[test]
    fn check_overflow_to_zero() {
        assert_eq!(
            alu_add_8bit(0xFF, Flags::empty(), 0x01, false),
            (0x00, Flags::ZERO | Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_overflow() {
        assert_eq!(
            alu_add_8bit(0xFF, Flags::empty(), 0x69, false),
            (0x68, Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_half_overflow() {
        assert_eq!(
            alu_add_8bit(0x0F, Flags::CARRY, 0x01, false),
            (0x10, Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_basic_use_carry() {
        assert_eq!(
            alu_add_8bit(0x00, Flags::CARRY, 0x01, true),
            (0x02, Flags::empty())
        );
    }

    #[test]
    fn check_overflow_to_zero_use_carry() {
        assert_eq!(
            alu_add_8bit(0xFE, Flags::CARRY, 0x01, true),
            (0x00, Flags::ZERO | Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_overflow_use_carry() {
        assert_eq!(
            alu_add_8bit(0xFF, Flags::CARRY, 0x69, true),
            (0x69, Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_half_overflow_use_carry() {
        assert_eq!(
            alu_add_8bit(0x0F, Flags::CARRY, 0x01, true),
            (0x11, Flags::HALF_CARRY)
        );
    }
}

/// Subtract 8-bit value from register A, storing the result in A.
/// If use_carry is true this will subtract the content of the carry flag along with the value and
/// take that into account when updating flags too.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always set
///
/// ## Flags::HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## Flags::CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
pub fn alu_sub_8bit(cpu: &mut CPU, d8: u8, use_carry: bool) {
    let original_a = cpu.registers.a;

    let cy = if use_carry && cpu.registers.f.contains(Flags::CARRY) {
        1
    } else {
        0
    };

    cpu.registers.a = original_a.wrapping_sub(d8).wrapping_sub(cy);

    cpu.registers.f.set(Flags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(Flags::SUBTRACT, true);
    cpu.registers
        .f
        .set(Flags::HALF_CARRY, (original_a & 0x0F) < (d8 & 0x0F) + cy);
    cpu.registers
        .f
        .set(Flags::CARRY, cpu.registers.a > original_a);
}

/// Logically AND 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always reset
///
/// ## Flags::HALF_CARRY
///
/// Always set
///
/// ## Flags::CARRY
///
/// Always reset
///
pub fn alu_and_8bit(cpu: &mut CPU, d8: u8) {
    cpu.registers.a &= d8;

    cpu.registers.f.set(Flags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, true);
    cpu.registers.f.set(Flags::CARRY, false);
}

/// Logically XOR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always reset
///
/// ## Flags::HALF_CARRY
///
/// Always reset
///
/// ## Flags::CARRY
///
/// Always reset
///
pub fn alu_xor_8bit(cpu: &mut CPU, d8: u8) {
    cpu.registers.a ^= d8;

    cpu.registers.f.set(Flags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, false);
}

/// Logically OR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always reset
///
/// ## Flags::HALF_CARRY
///
/// Always reset
///
/// ## Flags::CARRY
///
/// Always reset
///
pub fn alu_or_8bit(cpu: &mut CPU, d8: u8) {
    cpu.registers.a |= d8;

    cpu.registers.f.set(Flags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, false);
}

/// Compare an 8-bit value with register A by subtracting the two but not storing the result, only
/// the flags.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Always set
///
/// ## Flags::HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## Flags::CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
pub fn alu_cp_8bit(cpu: &mut CPU, d8: u8) {
    cpu.registers
        .f
        .set(Flags::ZERO, cpu.registers.a.wrapping_sub(d8) == 0);
    cpu.registers.f.set(Flags::SUBTRACT, true);
    cpu.registers
        .f
        .set(Flags::HALF_CARRY, (cpu.registers.a & 0x0F) < (d8 & 0x0F));
    cpu.registers.f.set(Flags::CARRY, cpu.registers.a < d8);
}

/// Swap upper and lower nibbles of an 8-bit value.
///
/// Update flags:
///
/// ## Flags::ZERO
///
/// Set if the result equals zero.
///
/// ## Flags::SUBTRACT
///
/// Reset
///
/// ## Flags::HALF_CARRY
///
/// Reset
///
/// ## Flags::CARRY
///
/// Reset
///
pub fn alu_swap(cpu: &mut CPU, d8: u8) -> u8 {
    let swapped_value = (d8 & 0x0F << 4) & (d8 & 0xF0 >> 4);

    cpu.registers.f.set(Flags::ZERO, swapped_value == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, false);

    swapped_value
}

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
