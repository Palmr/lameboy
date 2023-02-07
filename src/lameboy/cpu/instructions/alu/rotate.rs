use crate::lameboy::cpu::registers::Flags;

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the Flags::CARRY flag and the old value of the
/// Flags::CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the Flags::CARRY flag.
///
pub fn alu_rotate_left(d8: u8, flags: Flags, through_carry: bool) -> (u8, Flags) {
    let cy = if flags.contains(Flags::CARRY) { 1 } else { 0 };

    let high_bit = (d8 & 0b1000_0000) >> 7;
    let new_low_bit = if through_carry { cy } else { high_bit };
    let rotated_value = (d8 << 1) | new_low_bit;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, rotated_value == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, high_bit != 0);

    (rotated_value, new_flags)
}

#[cfg(test)]
mod test_alu_rotate_left {
    use super::alu_rotate_left;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_rotate_left(0x04, Flags::empty(), false),
            (0x08, Flags::empty())
        );

        assert_eq!(
            alu_rotate_left(0x04, Flags::empty(), true),
            (0x08, Flags::empty())
        );
    }

    #[test]
    fn reset_subtract_flag() {
        assert_eq!(
            alu_rotate_left(0x04, Flags::SUBTRACT, false),
            (0x08, Flags::empty())
        );

        assert_eq!(
            alu_rotate_left(0x04, Flags::SUBTRACT, true),
            (0x08, Flags::empty())
        );
    }

    #[test]
    fn reset_half_carry_flag() {
        assert_eq!(
            alu_rotate_left(0x04, Flags::HALF_CARRY, false),
            (0x08, Flags::empty())
        );

        assert_eq!(
            alu_rotate_left(0x04, Flags::HALF_CARRY, true),
            (0x08, Flags::empty())
        );
    }

    #[test]
    fn highest_bit_sets_carry() {
        assert_eq!(
            alu_rotate_left(0b1000_0000, Flags::empty(), false),
            (0b0000_0001, Flags::CARRY)
        );
        assert_eq!(
            alu_rotate_left(0b0100_0000, Flags::empty(), false),
            (0b1000_0000, Flags::empty())
        );

        assert_eq!(
            alu_rotate_left(0b1000_0000, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_rotate_left(0b0100_0000, Flags::empty(), true),
            (0b1000_0000, Flags::empty())
        );
    }

    #[test]
    fn zero_flag_gets_set() {
        assert_eq!(
            alu_rotate_left(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );

        assert_eq!(
            alu_rotate_left(0b1000_0000, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
    }

    #[test]
    fn carry_flat_sets_low_bit_if_through_carry() {
        assert_eq!(
            alu_rotate_left(0b0000_0000, Flags::CARRY, false),
            (0b0000_0000, Flags::ZERO)
        );
        assert_eq!(
            alu_rotate_left(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );

        assert_eq!(
            alu_rotate_left(0b0000_0000, Flags::CARRY, true),
            (0b0000_0001, Flags::empty())
        );
        assert_eq!(
            alu_rotate_left(0b0000_0000, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO)
        );
    }
}

/// Rotate an 8-bit value to the right.
///
/// If through_carry is true then the low bit will go into the Flags::CARRY flag and the old value of the
/// Flags::CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the Flags::CARRY flag.
///
pub fn alu_rotate_right(d8: u8, flags: Flags, through_carry: bool) -> (u8, Flags) {
    let cy = if flags.contains(Flags::CARRY) { 1 } else { 0 };
    let low_bit = d8 & 0x01;
    let new_high_bit = if through_carry { cy } else { low_bit };

    let rotated_value = (d8 >> 1) | (new_high_bit << 7);

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, rotated_value == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, low_bit != 0);

    (rotated_value, new_flags)
}

#[cfg(test)]
mod test_alu_rotate_right {
    use super::alu_rotate_right;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_rotate_right(0x08, Flags::empty(), false),
            (0x04, Flags::empty())
        );

        assert_eq!(
            alu_rotate_right(0x08, Flags::empty(), true),
            (0x04, Flags::empty())
        );
    }

    #[test]
    fn reset_subtract_flag() {
        assert_eq!(
            alu_rotate_right(0x08, Flags::SUBTRACT, false),
            (0x04, Flags::empty())
        );

        assert_eq!(
            alu_rotate_right(0x08, Flags::SUBTRACT, true),
            (0x04, Flags::empty())
        );
    }

    #[test]
    fn reset_half_carry_flag() {
        assert_eq!(
            alu_rotate_right(0x08, Flags::HALF_CARRY, false),
            (0x04, Flags::empty())
        );

        assert_eq!(
            alu_rotate_right(0x08, Flags::HALF_CARRY, true),
            (0x04, Flags::empty())
        );
    }

    #[test]
    fn lowest_bit_sets_carry() {
        assert_eq!(
            alu_rotate_right(0b0000_0001, Flags::empty(), false),
            (0b1000_0000, Flags::CARRY)
        );
        assert_eq!(
            alu_rotate_right(0b0000_0010, Flags::empty(), false),
            (0b0000_0001, Flags::empty())
        );

        assert_eq!(
            alu_rotate_right(0b0000_0001, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
        assert_eq!(
            alu_rotate_right(0b0000_0010, Flags::empty(), true),
            (0b0000_0001, Flags::empty())
        );
    }

    #[test]
    fn zero_flag_gets_set() {
        assert_eq!(
            alu_rotate_right(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );

        assert_eq!(
            alu_rotate_right(0b0000_0001, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO | Flags::CARRY)
        );
    }

    #[test]
    fn carry_flat_sets_high_bit_if_through_carry() {
        assert_eq!(
            alu_rotate_right(0b0000_0000, Flags::CARRY, false),
            (0b0000_0000, Flags::ZERO)
        );
        assert_eq!(
            alu_rotate_right(0b0000_0000, Flags::empty(), false),
            (0b0000_0000, Flags::ZERO)
        );

        assert_eq!(
            alu_rotate_right(0b0000_0000, Flags::CARRY, true),
            (0b1000_0000, Flags::empty())
        );
        assert_eq!(
            alu_rotate_right(0b0000_0000, Flags::empty(), true),
            (0b0000_0000, Flags::ZERO)
        );
    }
}
