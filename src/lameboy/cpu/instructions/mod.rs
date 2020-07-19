use lameboy::cpu::registers::Flags as RegisterFlags;
use lameboy::cpu::CPU;

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
fn opcode_flag_test(cpu: &CPU, opcode: u8) -> bool {
    let cc = (opcode & 0b0001_1000) >> 3;
    match cc {
        0b00 => !cpu.registers.f.contains(RegisterFlags::ZERO),
        0b01 => cpu.registers.f.contains(RegisterFlags::ZERO),
        0b10 => !cpu.registers.f.contains(RegisterFlags::CARRY),
        0b11 => cpu.registers.f.contains(RegisterFlags::CARRY),
        _ => {
            warn!("Unhandled condition: {}", cc);
            false
        }
    }
}
