use lameboy::cpu::registers::Flags;

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
pub fn alu_swap(d8: u8, flags: Flags) -> (u8, Flags) {
    let swapped_value = ((d8 & 0x0F) << 4) | ((d8 & 0xF0) >> 4);

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, swapped_value == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, false);

    (swapped_value, new_flags)
}

#[cfg(test)]
mod test_alu_swap {
    use lameboy::cpu::instructions::alu::swap::alu_swap;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_basic() {
        assert_eq!(alu_swap(0xF0, Flags::empty()), (0x0F, Flags::empty()));
    }

    #[test]
    fn set_zero_flag_if_result_zero() {
        assert_eq!(alu_swap(0x00, Flags::empty()), (0x00, Flags::ZERO));
    }

    #[test]
    fn clear_all_flags() {
        assert_eq!(
            alu_swap(
                0x01,
                Flags::ZERO | Flags::SUBTRACT | Flags::HALF_CARRY | Flags::CARRY
            ),
            (0x10, Flags::empty())
        );
    }
}
