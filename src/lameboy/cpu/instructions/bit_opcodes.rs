use lameboy::cpu::instructions::{bit_index_from_opcode, register_from_opcode};
use lameboy::cpu::registers::{Flags, Reg16, Reg8, Register};
use lameboy::cpu::Cpu;

/// Put the complement of an 8-bit values single bit into the RegisterFlags::ZERO flag.
///
/// Takes 8 cycles unless operating on an indirectly addressed value, then 16 cycles.
///
/// # Examples
///
/// ```asm
/// BIT 4, B  ; Flag::RegisterFlags::ZERO = (B & 0x01 << 4)
/// ```
pub fn bit_test(cpu: &mut Cpu, opcode: u8) -> u8 {
    let register = opcode & 0b0000_0111;
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let (value, duration) = match register {
        0b111 => (cpu.registers.read8(&Reg8::A), 8),
        0b000 => (cpu.registers.read8(&Reg8::B), 8),
        0b001 => (cpu.registers.read8(&Reg8::C), 8),
        0b010 => (cpu.registers.read8(&Reg8::D), 8),
        0b011 => (cpu.registers.read8(&Reg8::E), 8),
        0b100 => (cpu.registers.read8(&Reg8::H), 8),
        0b101 => (cpu.registers.read8(&Reg8::L), 8),
        0b110 => (cpu.mmu.read8(cpu.registers.read16(&Reg16::HL)), 16),
        _ => panic!("Unhandled register bit pattern: 0b{:08b}", register),
    };

    let tested_value = value & (0x01 << bit_index);

    cpu.registers.f.set(Flags::ZERO, tested_value == 0);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, true);
    //    cpu.registers.f.set(RegisterFlags::CARRY, false);

    duration
}

/// Set, or reset, an individual bit in an 8-bit value.
///
/// Takes 8 cycles unless operating on an indirectly addressed value, then 16 cycles.
///
/// # Examples
///
/// ```asm
/// SET 4, B  ; B = (B | 0x01 << 4)
/// RES 4, B  ; B = (B & 0x01 << 4)
/// ```
pub fn bit_assign(cpu: &mut Cpu, opcode: u8, set_bit: bool) -> u8 {
    let bit_index = bit_index_from_opcode(opcode);

    let (mut value, duration) = match register_from_opcode(opcode) {
        Register::Reg8(r8) => (cpu.registers.read8(&r8), 8),
        Register::Reg16(r16) => (cpu.mmu.read8(cpu.registers.read16(&r16)), 16),
    };

    if set_bit {
        value |= 0x01 << bit_index;
    } else {
        value &= !(0x01 << bit_index);
    }

    match register_from_opcode(opcode) {
        Register::Reg8(r8) => cpu.registers.write8(&r8, value),
        Register::Reg16(r16) => cpu.mmu.write8(cpu.registers.read16(&r16), value),
    };

    duration
}
