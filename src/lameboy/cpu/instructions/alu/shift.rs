use lameboy::cpu::registers::Flags;

/// Shift an 8-bit value to the left.
///
pub fn alu_shift_left(d8: u8, flags: Flags) -> (u8, Flags) {
    let high_bit = d8 & 0x80;
    let shifted_value = d8 << 1;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, shifted_value == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, high_bit != 0);

    (shifted_value, new_flags)
}

#[cfg(test)]
mod test_alu_shift_left {
    use super::alu_shift_left;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(alu_shift_left(0x04, Flags::empty()), (0x08, Flags::empty()));
    }

    #[test]
    fn zero_flag_set_on_zero_result() {
        assert_eq!(
            alu_shift_left(0b1000_0000, Flags::empty()),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_shift_left(0b0000_0000, Flags::empty()),
            (0b0000_0000, Flags::ZERO)
        );
    }

    #[test]
    fn high_bit_put_in_carry_flag() {
        assert_eq!(
            alu_shift_left(0b1100_0000, Flags::empty()),
            (0b1000_0000, Flags::CARRY)
        );
        assert_eq!(
            alu_shift_left(0b1000_0000, Flags::empty()),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_shift_left(0b0000_0000, Flags::empty()),
            (0b0000_0000, Flags::ZERO)
        );
    }

    #[test]
    fn half_carry_and_subtract_flags_reset() {
        assert_eq!(
            alu_shift_left(0b0000_0001, Flags::SUBTRACT | Flags::HALF_CARRY),
            (0b0000_0010, Flags::empty())
        );
    }
}

/// Shift an 8-bit value to the right.
///
pub fn alu_shift_right(d8: u8, flags: Flags, reset_high_bit: bool) -> (u8, Flags) {
    let high_bit = if reset_high_bit { 0 } else { d8 & 0x80 };
    let low_bit = d8 & 0x01;
    let shifted_value = (d8 >> 1) | high_bit;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, shifted_value == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, low_bit != 0);

    (shifted_value, new_flags)
}

#[cfg(test)]
mod test_alu_shift_right {
    use super::alu_shift_right;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_shift_right(0x08, Flags::empty(), false),
            (0x04, Flags::empty())
        );
    }

    #[test]
    fn reset_high_bit_if_told() {
        assert_eq!(
            alu_shift_right(0b1000_0010, Flags::empty(), false),
            (0b1100_0001, Flags::empty())
        );
        assert_eq!(
            alu_shift_right(0b1000_0010, Flags::empty(), true),
            (0b0100_0001, Flags::empty())
        );
    }

    #[test]
    fn zero_flag_set_on_zero_result() {
        assert_eq!(
            alu_shift_right(0b0000_0001, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_shift_right(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );

        assert_eq!(
            alu_shift_right(0b0000_0001, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_shift_right(0b0000_0000, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO)
        );
    }

    #[test]
    fn low_bit_put_in_carry_flag() {
        assert_eq!(
            alu_shift_right(0b0000_0011, Flags::empty(), false),
            (0b0000_0001, Flags::CARRY)
        );
        assert_eq!(
            alu_shift_right(0b0000_0001, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_shift_right(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );
    }

    #[test]
    fn half_carry_and_subtract_flags_reset() {
        assert_eq!(
            alu_shift_right(0b0000_0100, Flags::SUBTRACT | Flags::HALF_CARRY, false),
            (0b0000_0010, Flags::empty())
        );
    }
}
