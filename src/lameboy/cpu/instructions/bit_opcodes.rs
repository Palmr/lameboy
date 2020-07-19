use lameboy::cpu::registers::Reg8;
use lameboy::cpu::registers::{Flags as RegisterFlags, Reg16};
use lameboy::cpu::CPU;

/// Put the complement of an 8-bit values single bit into the RegisterFlags::ZERO flag.
///
/// Takes 8 cycles unless operating on an indirectly addressed value, then 16 cycles.
///
/// # Examples
///
/// ```asm
/// BIT 4, B  ; Flag::RegisterFlags::ZERO = (B & 0x01 << 4)
/// ```
pub fn bit_test(cpu: &mut CPU, opcode: u8) -> u8 {
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

    cpu.registers.f.set(RegisterFlags::ZERO, tested_value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, true);
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
pub fn bit_assign(cpu: &mut CPU, opcode: u8, set_bit: bool) -> u8 {
    let register = opcode & 0b0000_0111;
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let (mut value, duration) = match register {
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

    if set_bit {
        value |= 0x01 << bit_index;
    } else {
        value &= !(0x01 << bit_index);
    }

    match register {
        0b111 => cpu.registers.write8(&Reg8::A, value),
        0b000 => cpu.registers.write8(&Reg8::B, value),
        0b001 => cpu.registers.write8(&Reg8::C, value),
        0b010 => cpu.registers.write8(&Reg8::D, value),
        0b011 => cpu.registers.write8(&Reg8::E, value),
        0b100 => cpu.registers.write8(&Reg8::H, value),
        0b101 => cpu.registers.write8(&Reg8::L, value),
        0b110 => cpu.mmu.write8(cpu.registers.read16(&Reg16::HL), value),
        _ => panic!("Unhandled register bit pattern: 0b{:08b}", register),
    };

    duration
}
