use crate::lameboy::cpu::registers::Flags;

/// Increment 8-bit value
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
/// Set if the lower nibble of the value overflowed
///
/// ## Flags::CARRY
///
/// Unchanged
///
pub fn alu_inc_8bit(d8: u8, flags: Flags) -> (u8, Flags) {
    let mut new_flags = flags;
    new_flags.remove(Flags::SUBTRACT);
    new_flags.set(Flags::HALF_CARRY, d8 & 0x0F == 0x0F);

    let incremented = d8.wrapping_add(1);

    new_flags.set(Flags::ZERO, incremented == 0);

    (incremented, new_flags)
}

#[cfg(test)]
mod test_alu_inc_8bit {
    use super::alu_inc_8bit;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(alu_inc_8bit(0x00, Flags::empty()), (0x01, Flags::empty()));
    }

    #[test]
    fn check_zero_flag_if_overflow() {
        assert_eq!(
            alu_inc_8bit(0xFF, Flags::empty()),
            (0x00, Flags::ZERO | Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_half_carry_flag_if_overflow() {
        assert_eq!(
            alu_inc_8bit(0x0F, Flags::empty()),
            (0x10, Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_subtract_flag_reset() {
        assert_eq!(alu_inc_8bit(0x00, Flags::SUBTRACT), (0x01, Flags::empty()));
    }

    #[test]
    fn check_carry_flag_unchanged() {
        assert_eq!(alu_inc_8bit(0x00, Flags::CARRY), (0x01, Flags::CARRY));
        assert_eq!(
            alu_inc_8bit(0x0F, Flags::CARRY),
            (0x10, Flags::HALF_CARRY | Flags::CARRY)
        );
        assert_eq!(
            alu_inc_8bit(0xFF, Flags::CARRY),
            (0x00, Flags::ZERO | Flags::HALF_CARRY | Flags::CARRY)
        );

        assert_eq!(alu_inc_8bit(0x00, Flags::empty()), (0x01, Flags::empty()));
        assert_eq!(
            alu_inc_8bit(0x0F, Flags::empty()),
            (0x10, Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_inc_8bit(0xFF, Flags::empty()),
            (0x00, Flags::ZERO | Flags::HALF_CARRY)
        );
    }
}

/// Decrement 8-bit value
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
/// Set if the lower nibble of the value was zero and would underflow
///
/// ## Flags::CARRY
///
/// Unchanged
///
pub fn alu_dec_8bit(d8: u8, flags: Flags) -> (u8, Flags) {
    let mut new_flags = flags;
    new_flags.set(Flags::SUBTRACT, true);
    new_flags.set(Flags::HALF_CARRY, d8 & 0x0F == 0x00);

    let decremented = d8.wrapping_sub(1);

    new_flags.set(Flags::ZERO, decremented == 0);

    (decremented, new_flags)
}

#[cfg(test)]
mod test_alu_dec_8bit {
    use super::alu_dec_8bit;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(alu_dec_8bit(0x08, Flags::empty()), (0x07, Flags::SUBTRACT));
    }

    #[test]
    fn check_zero_flag() {
        assert_eq!(
            alu_dec_8bit(0x01, Flags::empty()),
            (0x00, Flags::SUBTRACT | Flags::ZERO)
        );
    }

    #[test]
    fn check_half_carry_flag_if_underflow() {
        assert_eq!(
            alu_dec_8bit(0xF0, Flags::empty()),
            (0xEF, Flags::SUBTRACT | Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_carry_flag_unchanged() {
        assert_eq!(
            alu_dec_8bit(0x00, Flags::CARRY),
            (0xFF, Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY)
        );
        assert_eq!(
            alu_dec_8bit(0xF0, Flags::CARRY),
            (0xEF, Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY)
        );
        assert_eq!(
            alu_dec_8bit(0x01, Flags::CARRY),
            (0x00, Flags::SUBTRACT | Flags::ZERO | Flags::CARRY)
        );

        assert_eq!(
            alu_dec_8bit(0x00, Flags::empty()),
            (0xFF, Flags::SUBTRACT | Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_dec_8bit(0xF0, Flags::empty()),
            (0xEF, Flags::SUBTRACT | Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_dec_8bit(0x01, Flags::empty()),
            (0x00, Flags::SUBTRACT | Flags::ZERO)
        );
    }
}
