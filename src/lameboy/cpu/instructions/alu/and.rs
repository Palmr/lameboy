use lameboy::cpu::registers::Flags;

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
pub fn alu_and_8bit(accumulator: u8, flags: Flags, d8: u8) -> (u8, Flags) {
    let new_accumulator = accumulator & d8;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, new_accumulator == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, true);
    new_flags.set(Flags::CARRY, false);

    (new_accumulator, new_flags)
}

#[cfg(test)]
mod test_alu_sand_8bit {
    use super::alu_and_8bit;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_and_8bit(0xFF, Flags::empty(), 0xFF),
            (0xFF, Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_flags_get_reset() {
        assert_eq!(
            alu_and_8bit(0x01, Flags::SUBTRACT | Flags::CARRY, 0x01),
            (0x01, Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_zero_flag_set() {
        assert_eq!(
            alu_and_8bit(0xF0, Flags::SUBTRACT | Flags::CARRY, 0x0F),
            (0x00, Flags::ZERO | Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_masking() {
        assert_eq!(
            alu_and_8bit(0b1111_1111, Flags::SUBTRACT | Flags::CARRY, 0b0101_0101),
            (0b0101_0101, Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_and_8bit(0b0000_1111, Flags::SUBTRACT | Flags::CARRY, 0b0101_0101),
            (0b0000_0101, Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_and_8bit(0b1111_0000, Flags::SUBTRACT | Flags::CARRY, 0b0101_0101),
            (0b0101_0000, Flags::HALF_CARRY)
        );

        assert_eq!(
            alu_and_8bit(0b0101_0101, Flags::SUBTRACT | Flags::CARRY, 0b1111_1111),
            (0b0101_0101, Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_and_8bit(0b0101_0101, Flags::SUBTRACT | Flags::CARRY, 0b0000_1111),
            (0b0000_0101, Flags::HALF_CARRY)
        );
        assert_eq!(
            alu_and_8bit(0b0101_0101, Flags::SUBTRACT | Flags::CARRY, 0b1111_0000),
            (0b0101_0000, Flags::HALF_CARRY)
        );
    }
}
