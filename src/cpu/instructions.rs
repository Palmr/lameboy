#![allow(unused_variables)]

use cpu::CPU;
use cpu::registers::*;

/// Panic if anything tries to run an undefined opcode, likely means the emulator has a bug.
///
pub fn undefined(cpu: &CPU, opcode: u8) -> () {
    panic!("Undefined opcode 0x{:02X}", opcode)
}

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

/// Stop the system clock and oscillator circuit to stop the CPU and LCD controller.
///
/// Stop mode can be cancelled by a reset signal.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// STOP
/// ```
pub fn stop(mut cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.pc);

    // Move PC on
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    if value != 0 {
        panic!("Stop instruction should be followed by a zero but found: 0x{:02X}", value);
    }

    // TODO - Halt the CPU & LCD display until a button is pressed

    return 4
}

/// Halt the system clock, though let the oscillator and LCD controller continue to run.
///
/// Halt mode can be cancelled by an interrupt or reset signal.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// HALT
/// ```
pub fn halt(mut cpu: &mut CPU) -> u8 {
    cpu.halt = true;

    return 4
}

/// This instruction conditionally adjusts the accumulator for BCD addition and subtraction
/// operations.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// DAA
/// ```
pub fn decimal_adjust(mut cpu: &mut CPU) -> u8 {
    let mut carry = false;

    if !cpu.registers.f.contains(SUBTRACT) {
      if cpu.registers.f.contains(CARRY) || cpu.registers.a > 0x99 {
        cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
        carry = true;
      }
      if cpu.registers.f.contains(HALF_CARRY) || cpu.registers.a & 0x0F > 0x09 {
        cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
      }
    } else if cpu.registers.f.contains(CARRY) {
      carry = true;
      cpu.registers.a = cpu.registers.a.wrapping_add(
        if cpu.registers.f.contains(HALF_CARRY) {
            0x9A
        }
        else {
            0xA0
        });
    } else if cpu.registers.f.contains(HALF_CARRY) {
      cpu.registers.a = cpu.registers.a.wrapping_add(0xFA);
    }

    cpu.registers.f.set(ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, carry);

    return 4
}

/// Complement A register, flipping all bits.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CPL
/// ```
pub fn complement(mut cpu: &mut CPU) -> u8 {
    let value = cpu.registers.a;
    cpu.registers.a = !value;

    cpu.registers.f.set(SUBTRACT, true);
    cpu.registers.f.set(HALF_CARRY, true);

    return 4
}

/// Set the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SCF ; Flag::CARRY = 1
/// ```
pub fn set_carry_flag(mut cpu: &mut CPU) -> u8 {
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, true);

    return 4
}

/// Complement the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CCF ; Flag::CARRY = !Flag::CARRY
/// ```
pub fn complement_carry_flag(mut cpu: &mut CPU) -> u8 {
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.toggle(CARRY);

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

    cpu.registers.write8(r8, value);

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

    cpu.registers.write8(r8, value);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(2);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(2);

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
        cpu.registers.pc = cpu.registers.pc.wrapping_add(2);

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
pub fn jump_relative_d8(mut cpu: &mut  CPU) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.mmu.read8(cpu.registers.pc) as i8;

    // Jump PC to that target address
    cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

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
pub fn jump_relative_conditional_d8(mut cpu: &mut  CPU, opcode: u8) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.mmu.read8(cpu.registers.pc) as i8;

    // Move the PC past the address operand
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

    // Test if the condition matches and if we need to jump
    if test_jump_condition(cpu, opcode) {
        // Jump PC to that target address
        cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

        return 12
    }
    else {
        return 8
    }
}

/// Push an 8-bit value to the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
fn push_stack_d8(mut cpu: &mut CPU, d8: u8) -> () {
    // Decrement stack pointer
    cpu.registers.sp.wrapping_sub(1);

    // Write byte to stack
    cpu.mmu.write8(cpu.registers.sp, d8);
}

/// Push a 16-bit value to the stack.
/// Pushing the high byte of the value first, then the low byte.
fn push_stack_d16(mut cpu: &mut CPU, d16: u16) -> () {
    // Write high byte
    push_stack_d8(cpu, ((d16 >> 8) & 0xFF) as u8);
    // Write low byte
    push_stack_d8(cpu, (d16 & 0xFF) as u8);
}

/// Pop an 8-bit value off the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
fn pop_stack_d8(mut cpu: &mut CPU) -> u8 {
    // Read byte from stack
    let value = cpu.mmu.read8(cpu.registers.sp);

    // Decrement stack pointer
    cpu.registers.sp.wrapping_add(1);

    return value;
}

/// Pop a 16-bit value off the stack.
/// Pushing the high byte of the value first, then the low byte.
fn pop_stack_d16(mut cpu: &mut CPU) -> u16 {
    let mut value: u16;
    // Pop low byte
    value = pop_stack_d8(cpu) as u16;
    // Pop high byte
    value |= (pop_stack_d8(cpu) as u16) << 8 ;

    return value;
}

/// Push a 16-bit register to the stack.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// PUSH BC ; STACK <<- BC
/// ```
pub fn push_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.registers.read16(r16);
    push_stack_d16(cpu, value);

    return 16
}

/// Pop the contents of the stack into a 16-bit register.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// POP BC ; BC <<- STACK
/// ```
pub fn pop_r16(mut cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = pop_stack_d16(cpu);
    cpu.registers.write16(r16, value);

    return 12
}

/// Jump to a different address using 16-bit data as an address after first pushing the current PC
/// to the stack.
///
/// Takes 24 cycles.
///
/// # Examples
///
/// ```asm
/// CALL $0150 ; STACK <<- PC; PC <- 0x0150
/// ```
pub fn call_d16(mut cpu: &mut CPU) -> u8 {
    // Read 16-bit jump target address
    let jump_target: u16 = cpu.mmu.read16(cpu.registers.pc);

    // Move the PC past the address operand
    cpu.registers.pc = cpu.registers.pc.wrapping_add(2);

    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    return 24
}

/// Jump to a different address using 16-bit data as an address after first pushing the current PC
/// to the stack if a given flag status condition matches.
///
/// Takes 24 cycles if condition matches, 12 cycles if it doesn't match.
///
/// # Examples
///
/// ```asm
/// CALL NZ $0150 ; IF !Flags::ZERO { STACK <<- PC; PC <- 0x0150 }
/// ```
pub fn call_conditional_d16(mut cpu: &mut CPU, opcode: u8) -> u8 {
    // Read 16-bit jump target address
    let jump_target: u16 = cpu.mmu.read16(cpu.registers.pc);

    // Move the PC past the address operand
    cpu.registers.pc = cpu.registers.pc.wrapping_add(2);

    if test_jump_condition(cpu, opcode) {
        // Push current PC to the stack
        let current_pc = cpu.registers.pc;
        push_stack_d16(cpu, current_pc);

        // Jump PC to the target address
        cpu.registers.pc = jump_target;

        return 24
    }
    else {
        return 12
    }
}

/// Return to an address that was pushed to the stack.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RET ; PC <<- STACK;
/// ```
pub fn ret(mut cpu: &mut CPU) -> u8 {
    // Read 16-bit jump target address
    let jump_target: u16 = pop_stack_d16(cpu);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    return 16
}

/// Return to an address that was pushed to the stack.
/// Enables the master interrupt flag.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RETI ; PC <<- STACK; ime == true
/// ```
pub fn ret_interrupt(mut cpu: &mut CPU) -> u8 {
    cpu.ime = true;
    ret(cpu)
}

/// Return to an address that was pushed to the stack if a given flag status condition matches.
///
/// Takes 20 cycles if condition matches, 8 cycles if it doesn't match.
///
/// # Examples
///
/// ```asm
/// RET ; PC <<- STACK;
/// ```
pub fn ret_conditional(mut cpu: &mut CPU, opcode: u8) -> u8 {
    if test_jump_condition(cpu, opcode) {
        ret(cpu);

        return 20
    }
    else {
        return 8
    }
}

/// Push the current PC to the stack and then jump to one of 8 positions in the zero page.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RST 1 ; STACK <<- PC; PC <- 0x0008
/// ```
pub fn reset(mut cpu: &mut CPU, opcode: u8) -> u8 {
    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Derive target address from opcode bits
    let jump_target = opcode & 0b00111000;

    // Jump PC to the target address
    cpu.registers.pc = jump_target as u16;

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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
    cpu.registers.pc = cpu.registers.pc.wrapping_add(1);

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

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
pub fn alu_rotate_left(mut cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};
    let high_bit = d8 & 0x80;
    let new_low_bit = if through_carry {cy} else {high_bit};
    let rotated_value = (d8 << 1) | new_low_bit;

    cpu.registers.f.set(ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, high_bit != 0);

    return rotated_value
}

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 4 cycles if always using A, otherwise 8 cycles
///
/// # Examples
///
/// ```asm
/// ; 4 cycle
/// RLCA  ; Rotate A left (resets Flag::ZERO)
/// RLA   ; Rotate A left through the carry flag (resets Flag::ZERO)
///
/// ; 8 cycle
/// RLC B ; Rotate B left (sets Flag::ZERO if rotated result == 0)
/// RL B  ; Rotate B left through the carry flag (sets Flag::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_left_r8(mut cpu: &mut CPU, r8: &Reg8, through_carry: bool, reset_zero: bool) -> u8 {
    let value = cpu.registers.read8(r8);

    let rotated_value = alu_rotate_left(cpu, value, through_carry, reset_zero);

    cpu.registers.write8(r8, rotated_value);

    return if reset_zero {4} else {8}
}

/// Rotate an indirect value, taken from memory using a 16-bit register as an address to the left.
///
/// If through_carry is true then the high bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RLC (HL) ; Rotate memory[hl] left (sets Flag::ZERO if rotated result == 0)
/// RL (HL)  ; Rotate memory[hl] left through the carry flag (sets Flag::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_left_indirect_hl(mut cpu: &mut CPU, through_carry: bool, reset_zero: bool) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.mmu.write8(a16_addr, rotated_value);

    return 16
}

/// Rotate an 8-bit value to the right.
///
/// If through_carry is true then the low bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
pub fn alu_rotate_right(mut cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(CARRY) {1} else {0};
    let low_bit = d8 & 0x01;
    let new_high_bit = if through_carry {cy} else {low_bit};

    let rotated_value = (d8 >> 1) | new_high_bit;

    cpu.registers.f.set(ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(SUBTRACT, false);
    cpu.registers.f.set(HALF_CARRY, false);
    cpu.registers.f.set(CARRY, low_bit != 0);

    return rotated_value
}

/// Rotate an 8-bit register to the right.
///
/// If through_carry is true then the low bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 4 cycles if always using A, otherwise 8 cycles
///
/// # Examples
///
/// ```asm
/// ; 4 cycle
/// RRCA  ; Rotate A right (resets Flag::ZERO)
/// RRA   ; Rotate A right through the carry flag (resets Flag::ZERO)
///
/// ; 8 cycle
/// RRC B ; Rotate B right (sets Flag::ZERO if rotated result == 0)
/// RR B  ; Rotate B right through the carry flag (sets Flag::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_right_r8(mut cpu: &mut CPU, r8: &Reg8, through_carry: bool, reset_zero: bool) -> u8 {
    let value = cpu.registers.read8(r8);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.registers.write8(r8, rotated_value);

    return if reset_zero {4} else {8}
}

/// Rotate an indirect value, taken from memory using a 16-bit register as an address to the right.
///
/// If through_carry is true then the low bit will go into the CARRY flag and the old value of the
/// CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the CARRY flag.
///
/// If reset_zero is true the ZERO flag will always be reset.
/// If it is not true the ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RRC (HL) ; Rotate memory[hl] right (sets Flag::ZERO if rotated result == 0)
/// RR (HL)  ; Rotate memory[hl] right through the carry flag (sets Flag::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_right_indirect_hl(mut cpu: &mut CPU, through_carry: bool, reset_zero: bool) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.mmu.write8(a16_addr, rotated_value);

    return 16
}
