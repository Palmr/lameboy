use crate::lameboy::cpu::registers::Flags;

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
pub fn alu_xor_8bit(accumulator: u8, flags: Flags, d8: u8) -> (u8, Flags) {
    let new_accumulator = accumulator ^ d8;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, new_accumulator == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, false);

    (new_accumulator, new_flags)
}

#[cfg(test)]
mod test_alu_xor_8bit {
    use super::alu_xor_8bit;
    use crate::lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_xor_8bit(0xFF, Flags::empty(), 0x0A),
            (0xF5, Flags::empty())
        );
    }

    #[test]
    fn check_flags_get_reset() {
        assert_eq!(
            alu_xor_8bit(
                0x01,
                Flags::ZERO | Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY,
                0x10
            ),
            (0x11, Flags::empty())
        );
    }

    #[test]
    fn check_zero_flag_set() {
        assert_eq!(
            alu_xor_8bit(0xF0, Flags::empty(), 0xF0),
            (0x00, Flags::ZERO)
        );
    }

    #[test]
    fn check_masking() {
        assert_eq!(
            alu_xor_8bit(0b1111_1111, Flags::empty(), 0b0101_0101),
            (0b1010_1010, Flags::empty())
        );
        assert_eq!(
            alu_xor_8bit(0b0000_1111, Flags::empty(), 0b0101_0101),
            (0b0101_1010, Flags::empty())
        );
        assert_eq!(
            alu_xor_8bit(0b1111_0000, Flags::empty(), 0b0101_0101),
            (0b1010_0101, Flags::empty())
        );

        assert_eq!(
            alu_xor_8bit(0b0101_0101, Flags::empty(), 0b1111_1111),
            (0b1010_1010, Flags::empty())
        );
        assert_eq!(
            alu_xor_8bit(0b0101_0101, Flags::empty(), 0b0000_1111),
            (0b0101_1010, Flags::empty())
        );
        assert_eq!(
            alu_xor_8bit(0b0101_0101, Flags::empty(), 0b1111_0000),
            (0b1010_0101, Flags::empty())
        );
    }
}
