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

/// Increment memory using a 16-bit register as an address.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// INC (HL)
/// ```
pub fn inc_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);

    let mut value = cpu.mmu.read8(a16_addr);

    value = value.wrapping_add(1);

    cpu.mmu.write8(a16_addr, value);

    return 12
}

/// Decrement memory using a 16-bit register as an address.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// DEC (HL)
/// ```
pub fn dec_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);

    let mut value = cpu.mmu.read8(a16_addr);

    value = value.wrapping_sub(1);

    cpu.mmu.write8(a16_addr, value);

    return 12
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
    cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc.wrapping_add(2);

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
    cpu.registers.pc.wrapping_add(2);

    // Split 16-bit register to low/high
    let r16_value = cpu.registers.read16(r16_source);
    let r16_high: u8 = ((r16_value & 0xFF00) >> 8) as u8;
    let r16_low: u8 = (r16_value & 0x00FF) as u8;

    // Write the two bytes to memory
    cpu.mmu.write8(a16_addr, r16_low);
    cpu.mmu.write8(a16_addr + 1, r16_high);

    return 20
}

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
fn test_jump_condition(cpu: &CPU, opcode: u8) -> bool {
    let cc = (opcode & 0b00011000) >> 3;
    match cc {
        0b00 => !cpu.registers.f.contains(ZERO),
        0b01 => cpu.registers.f.contains(ZERO),
        0b10 => !cpu.registers.f.contains(CARRY),
        0b11 => cpu.registers.f.contains(CARRY),
        _ => {println!("Unhandled condition: {}", cc); false},
    }
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
    // Read 16-bit jump target address
    let jump_target: u16 = cpu.mmu.read16(cpu.registers.pc);

    // Jump PC to that target address
    cpu.registers.pc = jump_target;

    return 16
}

/// Jump to a different address using 16-bit data as an address if a given flag status condition
/// matches.
///
/// Takes 16 cycles if condition matches, 12 if it doesn't match.
///
/// # Examples
///
/// ```asm
/// JP NZ $0150 ; IF !Flags::ZERO { PC <- 0x0150 }
/// ```
pub fn jump_conditional_d16(mut cpu: &mut CPU, opcode: u8) -> u8 {
    // Test if the condition matches and if we need to jump
    if test_jump_condition(cpu, opcode) {
        // Read 16-bit jump target address
        let jump_target: u16 = cpu.mmu.read16(cpu.registers.pc);

        // Jump PC to that target address
        cpu.registers.pc = jump_target;

        return 16
    }
    else {
        // Just move the PC past the address operand
        cpu.registers.pc.wrapping_add(2);

        return 12
    }
}

/// Jump to a different address using 16-bit register as an address.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// JP (HL) ; PC <- HL
/// ```
pub fn jump_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Set PC to whatever the 16-bit register is
    cpu.registers.pc = cpu.registers.read16(r16);

    return 4
}

/// Jump to a different relative address by adding the 8-bit operand to the current PC register.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// JR $DA ; PC <- PC + 0xDA
/// ```
pub fn jump_relative_d8(cpu: &CPU) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.mmu.read8(cpu.registers.pc) as i8;

    // Move the PC past the address operand
    cpu.registers.pc.wrapping_add(2);

    // Jump PC to that target address
    cpu.registers.pc.wrapping_add(jump_offset as u16);

    return 16
}

/// Jump to a different relative address by adding the 8-bit operand to the current PC register if
/// a given flag status condition matches.
///
/// Takes 12 cycles if condition matches, 8 if it doesn't match.
///
/// # Examples
///
/// ```asm
/// JR NZ $DA ; IF !Flags::ZERO { PC <- PC + $DA }
/// ```
pub fn jump_relative_conditional_d8(cpu: &CPU, opcode: u8) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.mmu.read8(cpu.registers.pc) as i8;

    // Move the PC past the address operand
    cpu.registers.pc.wrapping_add(2);

    // Test if the condition matches and if we need to jump
    if test_jump_condition(cpu, opcode) {
        // Jump PC to that target address
        cpu.registers.pc.wrapping_add(jump_offset as u16);

        return 12
    }
    else {
        return 8
    }
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

    // No change for ZERO
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_hl & 0x0F00) + (value & 0x0F00)) > 0x0F00);
    cpu.registers.f.set(CARRY, ((original_hl as u32) + (value as u32)) > 0xFFFF);

    return 8
}

/// ADD 8-bit value with register SP, storing the result in HL.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// ADD SP, $DA ; SP <- SP + 0xDA
/// ```
pub fn add_sp_d8(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc) as u16;
    // Move PC on
    cpu.registers.pc.wrapping_add(1);

    let original_sp = cpu.registers.read16(&Reg16::HL);

    let combined = original_sp.wrapping_add(value);

    cpu.registers.write16(&Reg16::HL, combined);

    cpu.registers.f.set(ZERO, false);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_sp & 0x0F00) + (value & 0x0F00)) > 0x0F00);
    cpu.registers.f.set(CARRY, ((original_sp as u32) + (value as u32)) > 0xFFFF);

    return 16
}

/// Add 8-bit value with register A, storing the result in A.
/// If use_carry is true this will add the content of the carry flag along with the value and take
/// that into account when updating flags too.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always unset
///
/// ## HALF_CARRY
///
/// Set if the lower nibble of the value added to the lower nibble of A was too large to fit in a u4
///
/// ## CARRY
///
/// Set if the value added to A would have been too large to fit in a u8
///
fn alu_add_8bit(mut cpu: &mut CPU, d8: u8, use_carry: bool) -> () {
    let original_a = cpu.registers.a;

    let cy = if use_carry && cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = original_a.wrapping_add(d8).wrapping_add(cy);

    cpu.registers.f.set(ZERO, original_a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, ((original_a & 0x0F) + (d8 & 0x0F) + cy) > 0x0F);
    cpu.registers.f.set(CARRY, ((original_a as u16) + (d8 as u16) + (cy as u16)) > 0xFF);
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

    alu_add_8bit(cpu, value, false);

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
    cpu.registers.pc.wrapping_add(1);

    alu_add_8bit(cpu, value, false);

    return 8
}

/// ADD memory addressed indirectly by a 16-bit register with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADD (HL) ; A <- A + memory[HL]
/// ```
pub fn add_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_add_8bit(cpu, value, false);

    return 8
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

    alu_add_8bit(cpu, value, true);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_add_8bit(cpu, value, true);

    return 8
}

/// ADD memory addressed indirectly by a 16-bit register plus the carry flag with register A,
/// storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADC (HL) ; A <- A + memory[HL] + Flag::CARRY
/// ```
pub fn adc_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_add_8bit(cpu, value, true);

    return 8
}

/// Subtract 8-bit value from register A, storing the result in A.
/// If use_carry is true this will subtract the content of the carry flag along with the value and
/// take that into account when updating flags too.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always set
///
/// ## HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
fn alu_sub_8bit(mut cpu: &mut CPU, d8: u8, use_carry: bool) -> () {
    let original_a = cpu.registers.a;

    let cy = if use_carry && cpu.registers.f.contains(CARRY) {1} else {0};

    cpu.registers.a = original_a.wrapping_sub(d8).wrapping_sub(cy);

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (original_a & 0x0F) < (d8 & 0x0F) + cy);
    cpu.registers.f.set(CARRY, (original_a as u16) < (d8 as u16) + (cy as u16));
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

    alu_sub_8bit(cpu, value, false);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_sub_8bit(cpu, value, false);

    return 8
}

/// Subtract memory addressed indirectly by a 16-bit register from register A, storing the result
/// in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SUB (HL) ; A <- A - memory[HL]
/// ```
pub fn sub_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_sub_8bit(cpu, value, false);

    return 8
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

    alu_sub_8bit(cpu, value, true);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_sub_8bit(cpu, value, true);

    return 8
}

/// Subtract memory addressed indirectly by a 16-bit register plus the carry flag from register A,
/// storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SBC (HL) ; A <- A - memory[HL] - Flag::CARRY
/// ```
pub fn sbc_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_sub_8bit(cpu, value, true);

    return 8
}

/// Logically AND 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always reset
///
/// ## HALF_CARRY
///
/// Always set
///
/// ## CARRY
///
/// Always reset
///
fn alu_and_8bit(mut cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a & d8;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, true);
    cpu.registers.f.set(CARRY, false);
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

    alu_and_8bit(cpu, value);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_and_8bit(cpu, value);

    return 8
}

/// AND memory addressed indirectly by a 16-bit register with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// AND (HL) ; A <- A & memory[HL]
/// ```
pub fn and_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_and_8bit(cpu, value);

    return 8
}

/// Logically XOR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always reset
///
/// ## HALF_CARRY
///
/// Always reset
///
/// ## CARRY
///
/// Always reset
///
fn alu_xor_8bit(mut cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a ^ d8;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);
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

    alu_xor_8bit(cpu, value);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_xor_8bit(cpu, value);

    return 8
}

/// XOR memory addressed indirectly by a 16-bit register with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// XOR (HL) ; A <- A ^ memory[HL]
/// ```
pub fn xor_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_xor_8bit(cpu, value);

    return 8
}

/// Logically OR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always reset
///
/// ## HALF_CARRY
///
/// Always reset
///
/// ## CARRY
///
/// Always reset
///
fn alu_or_8bit(mut cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a | d8;

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, false);
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

    alu_or_8bit(cpu, value);

    return 4
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
    cpu.registers.pc.wrapping_add(1);

    alu_or_8bit(cpu, value);

    return 8
}

/// OR memory addressed indirectly by a 16-bit register with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// OR (HL) ; A <- A | memory[HL]
/// ```
pub fn or_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_or_8bit(cpu, value);

    return 8
}

/// Compare an 8-bit value with register A by subtracting the two but not storing the result, only
/// the flags.
///
/// Update flags:
///
/// ## ZERO
///
/// Set if the result equals zero.
///
/// ## SUBTRACT
///
/// Always set
///
/// ## HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
fn alu_cp_8bit(mut cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.f.set(ZERO, cpu.registers.a.wrapping_sub(d8) == 0);
    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, (cpu.registers.a & 0x0F) < (d8 & 0x0F));
    cpu.registers.f.set(CARRY, cpu.registers.a < d8);
}

/// Subtract 8-bit register from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit operand register were equal.
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

    alu_cp_8bit(cpu, value);

    return 4
}

/// Subtract 8-bit value from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit value were equal.
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
    cpu.registers.pc.wrapping_add(1);

    alu_cp_8bit(cpu, value);

    return 8
}

/// Subtract memory addressed indirectly by a 16-bit register from register A, but don't store the
/// result. Zero flag will be set if register A and the 8-bit value were equal.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// CP (HL) ; Flag::ZERO true if A == memory[HL], Flag::CARRY true if A < memory[HL]
/// ```
pub fn cp_indirect_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_or_8bit(cpu, value);

    return 8
}
