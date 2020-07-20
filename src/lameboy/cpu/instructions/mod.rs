use lameboy::cpu::registers::Flags;

pub mod bit_opcodes;
pub mod calls;
pub mod eight_bit_alu;
pub mod eight_bit_loads;
pub mod jumps;
pub mod misc;
pub mod restarts;
pub mod returns;
pub mod rotates_and_shifts;
pub mod sixteen_bit_alu;
pub mod sixteen_bit_loads;

mod alu;
mod stack;

/// Test a jump condition against the flags register of a CPU and return the result as a bool.
///
/// The condition (cc) is defined by the middle two bits (& 0b00011000) of the opcode in the
/// following table:
///
/// | cc | Condition | Flag   |
/// |----|-----------|--------|
/// | 00 | NZ        | Z = 0  |
/// | 01 | Z         | Z = 1  |
/// | 10 | NC        | CY = 0 |
/// | 11 | C         | CY = 0 |
///
fn opcode_flag_test(opcode: u8, flags: Flags) -> bool {
    let cc = (opcode & 0b0001_1000) >> 3;
    match cc {
        0b00 => !flags.contains(Flags::ZERO),
        0b01 => flags.contains(Flags::ZERO),
        0b10 => !flags.contains(Flags::CARRY),
        0b11 => flags.contains(Flags::CARRY),
        _ => {
            warn!("Unhandled condition: {}", cc);
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::opcode_flag_test;
    use lameboy::cpu::registers::Flags;

    #[test]
    fn check_opcode_flag_nz() {
        assert_eq!(true, opcode_flag_test(0b0000_0000, Flags::empty()));
        assert_eq!(false, opcode_flag_test(0b0000_0000, Flags::ZERO));
        assert_eq!(true, opcode_flag_test(0b0000_0000, Flags::SUBTRACT));
        assert_eq!(true, opcode_flag_test(0b0000_0000, Flags::HALF_CARRY));
        assert_eq!(true, opcode_flag_test(0b0000_0000, Flags::CARRY));
    }

    #[test]
    fn check_opcode_flag_z() {
        assert_eq!(false, opcode_flag_test(0b0000_1000, Flags::empty()));
        assert_eq!(true, opcode_flag_test(0b0000_1000, Flags::ZERO));
        assert_eq!(false, opcode_flag_test(0b0000_1000, Flags::SUBTRACT));
        assert_eq!(false, opcode_flag_test(0b0000_1000, Flags::HALF_CARRY));
        assert_eq!(false, opcode_flag_test(0b0000_1000, Flags::CARRY));
    }

    #[test]
    fn check_opcode_flag_nc() {
        assert_eq!(true, opcode_flag_test(0b0001_0000, Flags::empty()));
        assert_eq!(true, opcode_flag_test(0b0001_0000, Flags::ZERO));
        assert_eq!(true, opcode_flag_test(0b0001_0000, Flags::SUBTRACT));
        assert_eq!(true, opcode_flag_test(0b0001_0000, Flags::HALF_CARRY));
        assert_eq!(false, opcode_flag_test(0b0001_0000, Flags::CARRY));
    }

    #[test]
    fn check_opcode_flag_c() {
        assert_eq!(false, opcode_flag_test(0b0001_1000, Flags::empty()));
        assert_eq!(false, opcode_flag_test(0b0001_1000, Flags::ZERO));
        assert_eq!(false, opcode_flag_test(0b0001_1000, Flags::SUBTRACT));
        assert_eq!(false, opcode_flag_test(0b0001_1000, Flags::HALF_CARRY));
        assert_eq!(true, opcode_flag_test(0b0001_1000, Flags::CARRY));
    }
}
