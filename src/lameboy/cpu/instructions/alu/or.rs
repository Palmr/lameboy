use lameboy::cpu::registers::Flags;

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
pub fn alu_or_8bit(accumulator: u8, flags: Flags, d8: u8) -> (u8, Flags) {
    let new_accumulator = accumulator | d8;

    let mut new_flags = flags;
    new_flags.set(Flags::ZERO, new_accumulator == 0);
    new_flags.set(Flags::SUBTRACT, false);
    new_flags.set(Flags::HALF_CARRY, false);
    new_flags.set(Flags::CARRY, false);

    (new_accumulator, new_flags)
}
