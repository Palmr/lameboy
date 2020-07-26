use lameboy::cpu::registers::{Flags, Reg16, Reg8, Register};

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

/// The BIT opcodes have a 3-bit value in the opcode corresponding to the bit number to work on
///
fn bit_index_from_opcode(opcode: u8) -> u8 {
    (opcode & 0b0011_1000) >> 3
}

#[cfg(test)]
mod tests_bit_index {
    use super::bit_index_from_opcode;

    #[test]
    fn test_bit_index_from_opcode() {
        assert_eq!(0, bit_index_from_opcode(0b0000_0000));
        assert_eq!(1, bit_index_from_opcode(0b0000_1000));
        assert_eq!(2, bit_index_from_opcode(0b0001_0000));
        assert_eq!(3, bit_index_from_opcode(0b0001_1000));
        assert_eq!(4, bit_index_from_opcode(0b0010_0000));
        assert_eq!(5, bit_index_from_opcode(0b0010_1000));
        assert_eq!(6, bit_index_from_opcode(0b0011_0000));
        assert_eq!(7, bit_index_from_opcode(0b0011_1000));
    }
}

fn register_from_opcode(opcode: u8) -> Register {
    let register = opcode & 0b0000_0111;

    match register {
        0b111 => Register::Reg8(Reg8::A),
        0b000 => Register::Reg8(Reg8::B),
        0b001 => Register::Reg8(Reg8::C),
        0b010 => Register::Reg8(Reg8::D),
        0b011 => Register::Reg8(Reg8::E),
        0b100 => Register::Reg8(Reg8::H),
        0b101 => Register::Reg8(Reg8::L),
        0b110 => Register::Reg16(Reg16::HL),
        _ => panic!("Unhandled register bit pattern: 0b{:08b}", register),
    }
}

#[cfg(test)]
mod tests_register_from_opcode {
    use super::register_from_opcode;
    use lameboy::cpu::registers::Reg16::HL;
    use lameboy::cpu::registers::Reg8::{A, B, C, D, E, H, L};
    use lameboy::cpu::registers::Register::{Reg16, Reg8};

    #[test]
    fn test_register_from_opcode() {
        assert_eq!(Reg8(A), register_from_opcode(0b0000_0111));
        assert_eq!(Reg8(B), register_from_opcode(0b0000_0000));
        assert_eq!(Reg8(C), register_from_opcode(0b0000_0001));
        assert_eq!(Reg8(D), register_from_opcode(0b0000_0010));
        assert_eq!(Reg8(E), register_from_opcode(0b0000_0011));
        assert_eq!(Reg8(H), register_from_opcode(0b0000_0100));
        assert_eq!(Reg8(L), register_from_opcode(0b0000_0101));
        assert_eq!(Reg16(HL), register_from_opcode(0b0000_0110));
    }
}
