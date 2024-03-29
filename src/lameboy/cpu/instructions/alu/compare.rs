use crate::lameboy::cpu::registers::Flags;

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
pub fn alu_cp_8bit(accumulator: u8, flags: Flags, d8: u8) -> Flags {
    let mut new_flags = flags;

    new_flags.set(Flags::ZERO, accumulator.wrapping_sub(d8) == 0);
    new_flags.set(Flags::SUBTRACT, true);
    new_flags.set(Flags::HALF_CARRY, (accumulator & 0x0F) < (d8 & 0x0F));
    new_flags.set(Flags::CARRY, accumulator < d8);

    new_flags
}

#[cfg(test)]
mod test_alu_cp_8bit {
    use super::alu_cp_8bit;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(alu_cp_8bit(0xFF, Flags::empty(), 0x01), Flags::SUBTRACT);
    }

    #[test]
    fn check_qual_values_set_zero() {
        assert_eq!(
            alu_cp_8bit(0x01, Flags::empty(), 0x01),
            Flags::ZERO | Flags::SUBTRACT
        );
    }

    #[test]
    fn check_underflow() {
        assert_eq!(
            alu_cp_8bit(0x00, Flags::empty(), 0x03),
            Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY
        );
    }

    #[test]
    fn check_half_underflow() {
        assert_eq!(alu_cp_8bit(0x37, Flags::empty(), 0x17), Flags::SUBTRACT);

        assert_eq!(
            alu_cp_8bit(0x37, Flags::empty(), 0x18),
            Flags::SUBTRACT | Flags::HALF_CARRY
        );
    }
}
