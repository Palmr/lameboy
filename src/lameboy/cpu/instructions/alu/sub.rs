use lameboy::cpu::registers::Flags;

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
pub fn alu_sub_8bit(accumulator: u8, flags: Flags, d8: u8, use_carry: bool) -> (u8, Flags) {
    let cy = if use_carry && flags.contains(Flags::CARRY) {
        1
    } else {
        0
    };

    let new_accumulator = accumulator.wrapping_sub(d8).wrapping_sub(cy);

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, new_accumulator == 0);
    new_flags.set(Flags::SUBTRACT, true);
    new_flags.set(Flags::HALF_CARRY, (accumulator & 0x0F) < (d8 & 0x0F) + cy);
    new_flags.set(Flags::CARRY, new_accumulator > accumulator);

    (new_accumulator, new_flags)
}

#[cfg(test)]
mod test_alu_sub_8bit {
    use lameboy::cpu::instructions::alu::sub::alu_sub_8bit;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(
            alu_sub_8bit(0xFF, Flags::empty(), 0x01, false),
            (0xFE, Flags::SUBTRACT)
        );
    }

    #[test]
    fn check_underflow_to_zero() {
        assert_eq!(
            alu_sub_8bit(0x01, Flags::empty(), 0x01, false),
            (0x00, Flags::ZERO | Flags::SUBTRACT)
        );
    }

    #[test]
    fn check_underflow() {
        assert_eq!(
            alu_sub_8bit(0x00, Flags::empty(), 0x03, false),
            (0xFD, Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_half_underflow() {
        assert_eq!(
            alu_sub_8bit(0x37, Flags::empty(), 0x17, false),
            (0x20, Flags::SUBTRACT)
        );

        assert_eq!(
            alu_sub_8bit(0x37, Flags::empty(), 0x18, false),
            (0x1F, Flags::SUBTRACT | Flags::HALF_CARRY)
        );
    }

    #[test]
    fn check_basic_use_carry() {
        assert_eq!(
            alu_sub_8bit(0x05, Flags::empty(), 0x01, true),
            (0x04, Flags::SUBTRACT)
        );

        assert_eq!(
            alu_sub_8bit(0x05, Flags::CARRY, 0x01, true),
            (0x03, Flags::SUBTRACT)
        );
    }

    #[test]
    fn check_underflow_to_zero_use_carry() {
        assert_eq!(
            alu_sub_8bit(0x02, Flags::CARRY, 0x01, true),
            (0x00, Flags::ZERO | Flags::SUBTRACT)
        );
    }

    #[test]
    fn check_underflow_use_carry() {
        assert_eq!(
            alu_sub_8bit(0x00, Flags::CARRY, 0x03, true),
            (0xFC, Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY)
        );
    }

    #[test]
    fn check_half_underflow_use_carry() {
        assert_eq!(
            alu_sub_8bit(0x38, Flags::CARRY, 0x18, true),
            (0x1F, Flags::SUBTRACT | Flags::HALF_CARRY)
        );
    }
}
