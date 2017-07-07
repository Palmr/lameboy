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
    let mut value = cpu.registers.read8(r8);

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
    let mut value = cpu.registers.read8(r8);

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

/// Load an 8-bit register into memory using a 16-bit register as an address. Then increment that 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD (HL+), A ; memory[HL] <- A; HL++
/// ```
pub fn load_indirect_r16_increment_r8(mut cpu: &mut CPU, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    // Copy from memory using 16-bit register value as address
    let a16_addr = cpu.registers.read16(r16_indirect_addr);

    cpu.mmu.write8(a16_addr, value);

    // Increment the 16-bit indirect address register
    cpu.registers.write16(r16_indirect_addr, a16_addr.wrapping_add(1));

    return 8
}

/// Load an 8-bit register with an indirect value, taken from memory using a 16-bit register as an
/// address. Then decrement that 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, (HL-) ; A <- memory[HL]; HL--
/// ```
pub fn load_indirect_r16_decrement_r8(mut cpu: &mut CPU, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    // Copy from memory using 16-bit register value as address
    let a16_addr = cpu.registers.read16(r16_indirect_addr);

    cpu.mmu.write8(a16_addr, value);

    // Increment the 16-bit indirect address register
    cpu.registers.write16(r16_indirect_addr, a16_addr.wrapping_sub(1));

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
    // Copy from memory using 16-bit register value as address
    let value = cpu.mmu.read8(cpu.registers.read16(r16_indirect_addr));
    cpu.registers.write8(r8_target, value);

    return 8
}

/// Load an 8-bit register with an indirect value, taken from memory using a 16-bit register as an
/// address. Then increment that 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, (HL+) ; A <- memory[HL]; HL++
/// ```
pub fn load_r8_indirect_r16_increment(mut cpu: &mut CPU, r8_target: &Reg8, r16_indirect_addr: &Reg16) -> u8 {
    // Copy from memory using 16-bit register value as address
    let r16_value = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(r16_value);
    cpu.registers.write8(r8_target, value);

    // Increment the 16-bit indirect address register
    cpu.registers.write16(r16_indirect_addr, r16_value.wrapping_add(1));

    return 8
}

/// Load an 8-bit register with an indirect value, taken from memory using a 16-bit register as an
/// address. Then decrement that 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, (HL-) ; A <- memory[HL]; HL--
/// ```
pub fn load_r8_indirect_r16_decrement(mut cpu: &mut CPU, r8_target: &Reg8, r16_indirect_addr: &Reg16) -> u8 {
    // Copy from memory using 16-bit register value as address
    let r16_value = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(r16_value);
    cpu.registers.write8(r8_target, value);

    // Decrement the 16-bit indirect address register
    cpu.registers.write16(r16_indirect_addr, r16_value.wrapping_sub(1));

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

/// Load memory, using a 16-bit register as an address, with an 8-bit value.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// LD (HL), $DA ; memory[HL] <- 0xDA
/// ```
pub fn load_indirect_r16_d8(mut cpu: &mut CPU, r16_indirect_addr: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    // Copy from source register to memory using indirect register
    cpu.mmu.write8(cpu.registers.read16(r16_indirect_addr), value);

    return 12
}

/// Load a 16-bit register into a 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD SP, HL
/// ```
pub fn load_r16_r16(mut cpu: &mut CPU, r16_target: &Reg16, r16_source: &Reg16) -> u8 {
    // Copy from source register to target register
    let value = cpu.registers.read16(r16_source);
    cpu.registers.write16(r16_target, value);

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

/// Load memory, using a 16-bit register as an address, with an 8-bit register value.
///
/// Takes 20 cycles.
///
/// # Examples
///
/// ```asm
/// LD ($8000), SP ; memory[0x8000] <- SP
/// ```
pub fn load_indirect_a16_r16(mut cpu: &mut CPU, r16_source: &Reg16) -> u8 {
    // Read 16-bit address
    let a16_addr = cpu.mmu.read16(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 2;

    // Split 16-bit register to low/high
    let r16_value = cpu.registers.read16(r16_source);
    let r16_high: u8 = ((r16_value & 0xFF00) >> 8) as u8;
    let r16_low: u8 = (r16_value & 0x00FF) as u8;

    // Write the two bytes to memory
    cpu.mmu.write8(a16_addr, r16_low);
    cpu.mmu.write8(a16_addr + 1, r16_high);

    return 20
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

/// ADD 16-bit register with register HL, storing the result in HL.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADD HL, BC ; HL <- HL + BC
/// ```
pub fn add_hl_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.registers.read16(r16);
    let original_hl = cpu.registers.read16(&Reg16::HL);

    let combined = original_hl.wrapping_add(value);

    cpu.registers.write16(&Reg16::HL, combined);

    cpu.registers.f.set(ZERO, combined == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_hl & 0x0F) + (value & 0x0F)) > 0x0F);
    cpu.registers.f.set(CARRY, ((original_hl as u16) + (value as u16)) > 0xFF);

    return 8
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
    let value = cpu.registers.read8(r8);
    let original_a = cpu.registers.a;

    cpu.registers.a = original_a.wrapping_add(value);

    cpu.registers.f.set(ZERO, original_a == 0);
    cpu.registers.f.set(SUBTRACT, false);

    cpu.registers.f.set(HALF_CARRY, ((original_a & 0x0F) + (value & 0x0F)) > 0x0F);
    cpu.registers.f.set(CARRY, ((original_a as u16) + (value as u16)) > 0xFF);

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
    let value = cpu.registers.read8(r8);
    let original_a = cpu.registers.a;

    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = original_a.wrapping_add(value).wrapping_add(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_a & 0x0F) + (value & 0x0F) + cy) > 0x0F);
    cpu.registers.f.set(CARRY, (original_a as u16 + value as u16 + cy as u16) > 0xFF);

    return 4
}

/// Subtract 8-bit register from register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SUB B ; A <- A - B
/// ```
pub fn sub_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);
    let original_a = cpu.registers.a;

    cpu.registers.a = original_a.wrapping_sub(value);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (original_a & 0x0F) < (value & 0x0F));
    cpu.registers.f.set(CARRY, original_a < value);

    return 4
}

/// Subtract 8-bit register plus the carry flag from register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SBC B ; A <- A - B - Flag::CARRY
/// ```
pub fn sbc_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);
    let original_a = cpu.registers.a;

    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = original_a.wrapping_sub(value).wrapping_sub(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (original_a & 0x0F) < (value & 0x0F) + cy);
    cpu.registers.f.set(CARRY, (original_a as u16) < (value as u16) + (cy as u16));

    return 4
}

/// AND 8-bit register with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// AND B ; A <- A & B
/// ```
pub fn and_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    cpu.registers.a = cpu.registers.a & value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, true);
    cpu.registers.f.set(CARRY, false);

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
    let value = cpu.registers.read8(r8);

    cpu.registers.a = cpu.registers.a ^ value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);

    return 4
}

/// OR 8-bit register with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// OR B ; A <- A | B
/// ```
pub fn or_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    cpu.registers.a = cpu.registers.a | value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);

    return 4
}

/// Subtract 8-bit register from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit operand register were equal
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CP B ; Flag::ZERO true if A == B, Flag::CARRY true if A < B
/// ```
pub fn cp_r8(mut cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    cpu.registers.f.set(ZERO, cpu.registers.a.wrapping_sub(value) == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (cpu.registers.a & 0x0F) < (value & 0x0F));
    cpu.registers.f.set(CARRY, cpu.registers.a < value);

    return 4
}

/// ADD 8-bit value with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADD $DA ; A <- A + 0x0DA
/// ```
pub fn add_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);
    // Move PC on
    cpu.registers.pc += 1;

    let original_a = cpu.registers.a;

    cpu.registers.a = original_a.wrapping_add(value);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_a & 0x0F) + (value & 0x0F)) > 0x0F);
    cpu.registers.f.set(CARRY, ((original_a as u16) + (value as u16)) > 0xFF);

    return 8
}

/// ADD 8-bit register plus the carry flag with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADC $DA ; A <- A + 0x0DA + Flag::CARRY
/// ```
pub fn adc_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    let original_a = cpu.registers.a;

    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = original_a.wrapping_add(value).wrapping_add(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_a & 0x0F) + (value & 0x0F) + cy) > 0x0F);
    cpu.registers.f.set(CARRY, (original_a as u16 + value as u16 + cy as u16) > 0xFF);

    return 8
}

/// Subtract 8-bit value from register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SUB $DA ; A <- A - 0xDA
/// ```
pub fn sub_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    let original_a = cpu.registers.a;

    cpu.registers.a = original_a.wrapping_sub(value);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (original_a & 0x0F) < (value & 0x0F));
    cpu.registers.f.set(CARRY, original_a < value);

    return 8
}

/// Subtract 8-bit value plus the carry flag from register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SBC $DA ; A <- A - 0xDA - Flag::CARRY
/// ```
pub fn sbc_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    let original_a = cpu.registers.a;

    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = cpu.registers.a.wrapping_sub(value).wrapping_sub(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (original_a & 0x0F) < (value & 0x0F) + cy);
    cpu.registers.f.set(CARRY, (original_a as u16) < (value as u16) + (cy as u16));

    return 8
}

/// AND 8-bit value with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// AND $DA ; A <- A & 0xDA
/// ```
pub fn and_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    cpu.registers.a = cpu.registers.a & value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, true);
    cpu.registers.f.set(CARRY, false);

    return 8
}

/// XOR 8-bit value with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// XOR $DA ; A <- A ^ 0xDA
/// ```
pub fn xor_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    cpu.registers.a = cpu.registers.a ^ value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);

    return 8
}

/// OR 8-bit value with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// OR $DA ; A <- A | 0xDA
/// ```
pub fn or_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    cpu.registers.a = cpu.registers.a | value;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);

    return 8
}

/// Subtract 8-bit value from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit value were equal
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// CP $DA ; Flag::ZERO true if A == 0xDA, Flag::CARRY true if A < 0xDA
/// ```
pub fn cp_d8(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc += 1;

    cpu.registers.f.set(ZERO, cpu.registers.a.wrapping_sub(value) == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (cpu.registers.a & 0x0F) < (value & 0x0F));
    cpu.registers.f.set(CARRY, cpu.registers.a < value);

    return 8
}