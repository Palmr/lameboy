use lameboy::cpu::registers::Flags;

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
    use super::alu_add_8bit;
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
