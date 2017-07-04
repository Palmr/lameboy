#![allow(unused_mut)]
#![allow(unused_variables)]

use cpu::CPU;
use cpu::registers::*;

/// No operation instruction, does nothing.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// NOP
/// ```
pub fn nop(mut cpu: &mut CPU) -> u8 {
    // Do nothing
    return 4
}

/// Increment 8-bit registers.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// INC A
/// INC B
/// ```
pub fn inc_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    match r8 {
        &Reg8::A => cpu.registers.a = cpu.registers.a + 1,
        &Reg8::B => cpu.registers.b = cpu.registers.b + 1,
        &Reg8::C => cpu.registers.c = cpu.registers.c + 1,
        &Reg8::D => cpu.registers.d = cpu.registers.d + 1,
        &Reg8::E => cpu.registers.e = cpu.registers.e + 1,
        &Reg8::H => cpu.registers.h = cpu.registers.h + 1,
        &Reg8::L => cpu.registers.l = cpu.registers.l + 1,
    }

    return 4
}

/// Decrement 8-bit registers.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// DEC A
/// DEC B
/// ```
pub fn dec_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    match r8 {
        &Reg8::A => cpu.registers.a = cpu.registers.a - 1,
        &Reg8::B => cpu.registers.b = cpu.registers.b - 1,
        &Reg8::C => cpu.registers.c = cpu.registers.c - 1,
        &Reg8::D => cpu.registers.d = cpu.registers.d - 1,
        &Reg8::E => cpu.registers.e = cpu.registers.e - 1,
        &Reg8::H => cpu.registers.h = cpu.registers.h - 1,
        &Reg8::L => cpu.registers.l = cpu.registers.l - 1,
    }

    return 4
}

/// Increment 16-bit registers.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// INC AB
/// INC CD
/// ```
pub fn inc_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let inc = cpu.registers.read16(r16) + 1;
    cpu.registers.write16(r16, inc);

    return 8
}

/// Decrement 16-bit registers.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// DEC AB
/// DEC CD
/// ```
pub fn dec_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let dec = cpu.registers.read16(r16) - 1;
    cpu.registers.write16(r16, dec);

    return 8
}

/// Load a 16-bit value into a 16-bit register.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// LD SP, $FFFE
/// LD HL, $9FFF
/// ```
pub fn load_r16_d16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 16-bit value
    let value: u16 = 0xFFFF;

    // Write it to the register
    cpu.registers.write16(r16, value);

    return 12
}
