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
pub fn nop(cpu: &CPU) -> u8 {
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
    let mut value = match r8 {
        &Reg8::A => cpu.registers.a,
        &Reg8::B => cpu.registers.b,
        &Reg8::C => cpu.registers.c,
        &Reg8::D => cpu.registers.d,
        &Reg8::E => cpu.registers.e,
        &Reg8::H => cpu.registers.h,
        &Reg8::L => cpu.registers.l,
    };

    value = value.wrapping_add(1);

    cpu.registers.f.set(ZERO, value == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, value  & 0xf == 0xf);

    match r8 {
        &Reg8::A => cpu.registers.a = value,
        &Reg8::B => cpu.registers.b = value,
        &Reg8::C => cpu.registers.c = value,
        &Reg8::D => cpu.registers.d = value,
        &Reg8::E => cpu.registers.e = value,
        &Reg8::H => cpu.registers.h = value,
        &Reg8::L => cpu.registers.l = value,
    };

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
    let mut value = match r8 {
        &Reg8::A => cpu.registers.a,
        &Reg8::B => cpu.registers.b,
        &Reg8::C => cpu.registers.c,
        &Reg8::D => cpu.registers.d,
        &Reg8::E => cpu.registers.e,
        &Reg8::H => cpu.registers.h,
        &Reg8::L => cpu.registers.l,
    };

    value = value.wrapping_sub(1);

    cpu.registers.f.set(ZERO, value == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, value  & 0xf == 0x0);

    match r8 {
        &Reg8::A => cpu.registers.a = value,
        &Reg8::B => cpu.registers.b = value,
        &Reg8::C => cpu.registers.c = value,
        &Reg8::D => cpu.registers.d = value,
        &Reg8::E => cpu.registers.e = value,
        &Reg8::H => cpu.registers.h = value,
        &Reg8::L => cpu.registers.l = value,
    };

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
    let mut value = cpu.registers.read16(r16);

    value = value.wrapping_add(1);

    cpu.registers.write16(r16, value);

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
    let mut value = cpu.registers.read16(r16);

    value = value.wrapping_sub(1);

    cpu.registers.write16(r16, value);

    return 8
}

/// Load an 8-bit register into another 8-bit register.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, B ; A <- B
/// LD B, D ; B <- D
/// ```
pub fn load_r8_r8(mut cpu: &mut CPU, r8_target: &Reg8, r8_source: &Reg8) -> u8 {
    // Copy from source register to target register
    let value = cpu.registers.read8(r8_source);
    cpu.registers.write8(r8_target, value);

    return 4
}

/// Load an 8-bit value into a 8-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, $FF
/// LD B, $9F
/// ```
pub fn load_r8_d8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    // Write it to the register
    cpu.registers.write8(r8, value);

    return 8
}

/// Load an 8-bit register with an indirect value, taken from memory using a 16-bit register as an
/// address.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, (HL) ; A <- memory[HL]
/// ```
pub fn load_r8_indirect_r16(mut cpu: &mut CPU, r8_target: &Reg8, r16_indirect_addr: &Reg16) -> u8 {
    // Copy from memory using indirect register to target register
    let value = cpu.mmu.read8(cpu.registers.read16(r16_indirect_addr));
    cpu.registers.write8(r8_target, value);

    return 8
}

/// Load memory, using a 16-bit register as an address, with an 8-bit register value.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD (HL), A ; memory[HL] <- A
/// ```
pub fn load_indirect_r16_r8(mut cpu: &mut CPU, r16_indirect_addr: &Reg16, r8_source: &Reg8) -> u8 {
    // Copy from source register to memory using indirect register
    cpu.mmu.write8(cpu.registers.read16(r16_indirect_addr), cpu.registers.read8(r8_source));

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
    let value: u16 = cpu.mmu.read16(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 2;

    // Write it to the register
    cpu.registers.write16(r16, value);

    return 12
}

/// Jump to a different address using 16-bit data as an address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// JP $0150 ; PC <- 0x0150
/// ```
pub fn jump_d16(mut cpu: &mut CPU) -> u8 {
    // Read 16-bit value
    let value: u16 = cpu.mmu.read16(cpu.registers.pc);

    // Jump PC to that value
    cpu.registers.pc = value;

    return 16
}

/// ADD 8-bit register with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// ADD B ; A <- A + B
/// ```
pub fn add_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = match r8 {
        &Reg8::A => cpu.registers.a,
        &Reg8::B => cpu.registers.b,
        &Reg8::C => cpu.registers.c,
        &Reg8::D => cpu.registers.d,
        &Reg8::E => cpu.registers.e,
        &Reg8::H => cpu.registers.h,
        &Reg8::L => cpu.registers.l,
    };

    cpu.registers.a = cpu.registers.a.wrapping_add(value);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((cpu.registers.a & 0x0F) + (value & 0x0F)) > 0x0F);
    cpu.registers.f.set(CARRY, ((cpu.registers.a as u16) + (value as u16)) > 0xFF);

    return 4
}

/// ADD 8-bit register plus the carry flag with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// ADC B ; A <- A + B + Flag::CARRY
/// ```
pub fn adc_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = match r8 {
        &Reg8::A => cpu.registers.a,
        &Reg8::B => cpu.registers.b,
        &Reg8::C => cpu.registers.c,
        &Reg8::D => cpu.registers.d,
        &Reg8::E => cpu.registers.e,
        &Reg8::H => cpu.registers.h,
        &Reg8::L => cpu.registers.l,
    };

    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = cpu.registers.a.wrapping_add(value).wrapping_add(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((cpu.registers.a & 0x0F) + (value & 0x0F) + cy) > 0x0F);
    cpu.registers.f.set(CARRY, (cpu.registers.a as u16 + value as u16 + cy as u16) > 0xFF);

    return 4
}

/// XOR 8-bit register with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// XOR B ; A <- A ^ B
/// ```
pub fn xor_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = match r8 {
        &Reg8::A => cpu.registers.a,
        &Reg8::B => cpu.registers.b,
        &Reg8::C => cpu.registers.c,
        &Reg8::D => cpu.registers.d,
        &Reg8::E => cpu.registers.e,
        &Reg8::H => cpu.registers.h,
        &Reg8::L => cpu.registers.l,
    };

    cpu.registers.a = cpu.registers.a ^ value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);

    return 4
}
