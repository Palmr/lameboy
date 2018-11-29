use cpu::registers::Flags as RegisterFlags;
use cpu::registers::{Reg16, Reg8};
use cpu::CPU;

/// Panic if anything tries to run an undefined opcode, likely means the emulator has a bug.
pub fn undefined(cpu: &CPU, opcode: u8) -> u8 {
    panic!(
        "Undefined opcode 0x{:02X} at pc=0x{:04X}",
        opcode, cpu.registers.pc
    )
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
pub fn nop(_: &CPU) -> u8 {
    // Do nothing
    return 4;
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
pub fn stop(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    if value != 0 {
        panic!(
            "Stop instruction should be followed by a zero but found: 0x{:02X} at pc=0x{:04X}",
            value, cpu.registers.pc
        );
    }

    // TODO - Halt the CPU & LCD display until a button is pressed

    return 4;
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
pub fn halt(cpu: &mut CPU) -> u8 {
    cpu.halt = true;

    return 4;
}

/// Enable or disable interrupts.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// DI
/// EI
/// ```
pub fn interrupts(cpu: &mut CPU, enabled: bool) -> u8 {
    cpu.ime = enabled;

    return 4;
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
pub fn decimal_adjust(cpu: &mut CPU) -> u8 {
    let mut carry = false;

    if !cpu.registers.f.contains(RegisterFlags::SUBTRACT) {
        if cpu.registers.f.contains(RegisterFlags::CARRY) || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            carry = true;
        }
        if cpu.registers.f.contains(RegisterFlags::HALF_CARRY) || cpu.registers.a & 0x0F > 0x09 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
        }
    } else if cpu.registers.f.contains(RegisterFlags::CARRY) {
        carry = true;
        cpu.registers.a =
            cpu.registers
                .a
                .wrapping_add(if cpu.registers.f.contains(RegisterFlags::HALF_CARRY) {
                    0x9A
                } else {
                    0xA0
                });
    } else if cpu.registers.f.contains(RegisterFlags::HALF_CARRY) {
        cpu.registers.a = cpu.registers.a.wrapping_add(0xFA);
    }

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, carry);

    return 4;
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
pub fn complement(cpu: &mut CPU) -> u8 {
    let value = cpu.registers.a;
    cpu.registers.a = !value;

    cpu.registers.f.set(RegisterFlags::SUBTRACT, true);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, true);

    return 4;
}

/// Set the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SCF ; Flag::RegisterFlags::CARRY = 1
/// ```
pub fn set_carry_flag(cpu: &mut CPU) -> u8 {
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, true);

    return 4;
}

/// Complement the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CCF ; Flag::RegisterFlags::CARRY = !Flag::RegisterFlags::CARRY
/// ```
pub fn complement_carry_flag(cpu: &mut CPU) -> u8 {
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.toggle(RegisterFlags::CARRY);

    return 4;
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
pub fn inc_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let mut value = cpu.registers.read8(r8);

    cpu.registers
        .f
        .set(RegisterFlags::HALF_CARRY, value & 0xf == 0xf);

    value = value.wrapping_add(1);

    cpu.registers.f.set(RegisterFlags::ZERO, value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);

    cpu.registers.write8(r8, value);

    return 4;
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
pub fn dec_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let mut value = cpu.registers.read8(r8);

    cpu.registers
        .f
        .set(RegisterFlags::HALF_CARRY, value & 0xf == 0x0);

    value = value.wrapping_sub(1);

    cpu.registers.f.set(RegisterFlags::ZERO, value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, true);

    cpu.registers.write8(r8, value);

    return 4;
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
pub fn inc_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let mut value = cpu.registers.read16(r16);

    value = value.wrapping_add(1);

    cpu.registers.write16(r16, value);

    return 8;
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
pub fn dec_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let mut value = cpu.registers.read16(r16);

    value = value.wrapping_sub(1);

    cpu.registers.write16(r16, value);

    return 8;
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
pub fn inc_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);

    let mut value = cpu.mmu.read8(a16_addr);

    value = value.wrapping_add(1);

    cpu.mmu.write8(a16_addr, value);

    return 12;
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
pub fn dec_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);

    let mut value = cpu.mmu.read8(a16_addr);

    value = value.wrapping_sub(1);

    cpu.mmu.write8(a16_addr, value);

    return 12;
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
pub fn load_r8_r8(cpu: &mut CPU, r8_target: &Reg8, r8_source: &Reg8) -> u8 {
    // Copy from source register to target register
    let value = cpu.registers.read8(r8_source);
    cpu.registers.write8(r8_target, value);

    return 4;
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
pub fn load_r8_d8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    // Write it to the register
    cpu.registers.write8(r8, value);

    return 8;
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
pub fn load_indirect_r16_increment_r8(cpu: &mut CPU, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    // Copy from memory using 16-bit register value as address
    let a16_addr = cpu.registers.read16(r16_indirect_addr);

    cpu.mmu.write8(a16_addr, value);

    // Increment the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, a16_addr.wrapping_add(1));

    return 8;
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
pub fn load_indirect_r16_decrement_r8(cpu: &mut CPU, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    // Copy from memory using 16-bit register value as address
    let a16_addr = cpu.registers.read16(r16_indirect_addr);

    cpu.mmu.write8(a16_addr, value);

    // Increment the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, a16_addr.wrapping_sub(1));

    return 8;
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
pub fn load_r8_indirect_r16(cpu: &mut CPU, r8_target: &Reg8, r16_indirect_addr: &Reg16) -> u8 {
    // Copy from memory using 16-bit register value as address
    let value = cpu.mmu.read8(cpu.registers.read16(r16_indirect_addr));
    cpu.registers.write8(r8_target, value);

    return 8;
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
pub fn load_r8_indirect_r16_increment(
    cpu: &mut CPU,
    r8_target: &Reg8,
    r16_indirect_addr: &Reg16,
) -> u8 {
    // Copy from memory using 16-bit register value as address
    let r16_value = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(r16_value);
    cpu.registers.write8(r8_target, value);

    // Increment the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, r16_value.wrapping_add(1));

    return 8;
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
pub fn load_r8_indirect_r16_decrement(
    cpu: &mut CPU,
    r8_target: &Reg8,
    r16_indirect_addr: &Reg16,
) -> u8 {
    // Copy from memory using 16-bit register value as address
    let r16_value = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(r16_value);
    cpu.registers.write8(r8_target, value);

    // Decrement the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, r16_value.wrapping_sub(1));

    return 8;
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
pub fn load_indirect_r16_r8(cpu: &mut CPU, r16_indirect_addr: &Reg16, r8_source: &Reg8) -> u8 {
    // Copy from source register to memory using indirect register
    cpu.mmu.write8(
        cpu.registers.read16(r16_indirect_addr),
        cpu.registers.read8(r8_source),
    );

    return 8;
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
pub fn load_indirect_r16_d8(cpu: &mut CPU, r16_indirect_addr: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    // Copy from source register to memory using indirect register
    cpu.mmu
        .write8(cpu.registers.read16(r16_indirect_addr), value);

    return 12;
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
pub fn load_r16_r16(cpu: &mut CPU, r16_target: &Reg16, r16_source: &Reg16) -> u8 {
    // Copy from source register to target register
    let value = cpu.registers.read16(r16_source);
    cpu.registers.write16(r16_target, value);

    return 8;
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
pub fn load_r16_d16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 16-bit value
    let value: u16 = cpu.fetch16();

    // Write it to the register
    cpu.registers.write16(r16, value);

    return 12;
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
pub fn load_indirect_a16_r16(cpu: &mut CPU, r16_source: &Reg16) -> u8 {
    // Read 16-bit address
    let a16_addr = cpu.fetch16();

    // Split 16-bit register to low/high
    let r16_value = cpu.registers.read16(r16_source);
    let r16_high: u8 = ((r16_value & 0xFF00) >> 8) as u8;
    let r16_low: u8 = (r16_value & 0x00FF) as u8;

    // Write the two bytes to memory
    cpu.mmu.write8(a16_addr, r16_low);
    cpu.mmu.write8(a16_addr + 1, r16_high);

    return 20;
}

/// Load memory, using an 8-bit value added to 0xFF00 as an address, with the A register value.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// LDH ($DA), A ; memory[0xFFDA] <- A
/// ```
pub fn load_high_mem_d8_reg_a(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let address = 0xFF00 + cpu.fetch8() as u16;

    // Write the byte to memory
    cpu.mmu.write8(address, cpu.registers.a);

    return 12;
}

/// Load the A register with a value from memory, using an 8-bit value added to 0xFF00 as an
/// address.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// LDH A, ($DA) ; A <- memory[0xFFDA]
/// ```
pub fn load_reg_a_high_mem_d8(cpu: &mut CPU) -> u8 {
    // Address is offset plus 8-bit data
    let addr = 0xFF00 + cpu.fetch8() as u16;

    // Read 8-bit value
    let value = cpu.mmu.read8(addr);

    cpu.registers.a = value;

    return 12;
}

/// Load memory, using the register C value added to 0xFF00 as an address, with the A register
/// value.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD (C), A ; memory[0xFF00 + C] <- A
/// ```
pub fn load_high_mem_reg_c_reg_a(cpu: &mut CPU) -> u8 {
    let address = 0xFF00 + cpu.registers.c as u16;

    // Write the byte to memory
    cpu.mmu.write8(address, cpu.registers.a);

    return 8;
}

/// Load the A register with a value from memory, using the value of register C added to 0xFF00 as
/// an address.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LDH A, (C) ; A <- memory[0xFF00 + C]
/// ```
pub fn load_reg_a_high_mem_reg_c(cpu: &mut CPU) -> u8 {
    let address = 0xFF00 + cpu.registers.c as u16;

    cpu.registers.a = cpu.mmu.read8(address);

    return 8;
}

/// Load memory, using a 16-bit value address, with the A register value.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// LD ($0150), A ; memory[0x0150] <- A
/// ```
pub fn load_a16_reg_a(cpu: &mut CPU) -> u8 {
    // Read 16-bit address value
    let addr = cpu.fetch16();

    // Write the byte to memory
    cpu.mmu.write8(addr, cpu.registers.a);

    return 16;
}

/// Load the A register with a value from memory, using a 16-bit value as an address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, ($0150) ; A <- memory[0x0150]
/// ```
pub fn load_reg_a_a16(cpu: &mut CPU) -> u8 {
    let addr = cpu.fetch16();

    // Read 8-bit value
    let value = cpu.mmu.read8(addr);

    cpu.registers.a = value;

    return 16;
}

/// Load the HL register with the value of the SP register added to an 8-bit value.
///
/// Takes 12 cycles.
///
/// # Examples
///
/// ```asm
/// LD HL, SP+d8 ; HL <- SP + d8
/// ```
pub fn load_reg_hl_reg_sp_d8(cpu: &mut CPU) -> u8 {
    // TODO - Could combine logic with add_sp_d8
    // Read 8-bit value
    let value = cpu.fetch8() as u16;

    let combined = cpu.registers.sp.wrapping_add(value);

    cpu.registers.write16(&Reg16::HL, combined);

    cpu.registers.f.set(RegisterFlags::ZERO, false);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        ((cpu.registers.sp & 0x0F00) + (value & 0x0F00)) > 0x0F00,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        ((cpu.registers.sp as u32) + (value as u32)) > 0xFFFF,
    );

    return 12;
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

/// Jump to a different address using 16-bit data as an address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// JP $0150 ; PC <- 0x0150
/// ```
pub fn jump_d16(cpu: &mut CPU) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Jump PC to that target address
    cpu.registers.pc = jump_target;

    return 16;
}

/// Jump to a different address using 16-bit data as an address if a given flag status condition
/// matches.
///
/// Takes 16 cycles if condition matches, 12 if it doesn't match.
///
/// # Examples
///
/// ```asm
/// JP NZ $0150 ; IF !Flags::RegisterFlags::ZERO { PC <- 0x0150 }
/// ```
pub fn jump_conditional_d16(cpu: &mut CPU, opcode: u8) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Test if the condition matches and if we need to jump
    if test_jump_condition(cpu, opcode) {
        // Jump PC to that target address
        cpu.registers.pc = jump_target;

        return 16;
    } else {
        return 12;
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
pub fn jump_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Set PC to whatever the 16-bit register is
    cpu.registers.pc = cpu.registers.read16(r16);

    return 4;
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
pub fn jump_relative_d8(cpu: &mut CPU) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.fetch8() as i8;

    // Jump PC to that target address
    cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

    return 16;
}

/// Jump to a different relative address by adding the 8-bit operand to the current PC register if
/// a given flag status condition matches.
///
/// Takes 12 cycles if condition matches, 8 if it doesn't match.
///
/// # Examples
///
/// ```asm
/// JR NZ $DA ; IF !Flags::RegisterFlags::ZERO { PC <- PC + $DA }
/// ```
pub fn jump_relative_conditional_d8(cpu: &mut CPU, opcode: u8) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.fetch8() as i8;

    // Test if the condition matches and if we need to jump
    if test_jump_condition(cpu, opcode) {
        // Jump PC to that target address
        cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

        return 12;
    } else {
        return 8;
    }
}

/// Push an 8-bit value to the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
fn push_stack_d8(cpu: &mut CPU, d8: u8) -> () {
    // Decrement stack pointer
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);

    // Write byte to stack
    cpu.mmu.write8(cpu.registers.sp, d8);
}

/// Push a 16-bit value to the stack.
/// Pushing the high byte of the value first, then the low byte.
fn push_stack_d16(cpu: &mut CPU, d16: u16) -> () {
    // Write high byte
    push_stack_d8(cpu, ((d16 >> 8) & 0xFF) as u8);
    // Write low byte
    push_stack_d8(cpu, (d16 & 0xFF) as u8);
}

/// Pop an 8-bit value off the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
fn pop_stack_d8(cpu: &mut CPU) -> u8 {
    // Read byte from stack
    let value = cpu.mmu.read8(cpu.registers.sp);

    // Increment stack pointer
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);

    return value;
}

/// Pop a 16-bit value off the stack.
/// Pushing the high byte of the value first, then the low byte.
fn pop_stack_d16(cpu: &mut CPU) -> u16 {
    let mut value: u16;
    // Pop low byte
    value = pop_stack_d8(cpu) as u16;
    // Pop high byte
    value |= (pop_stack_d8(cpu) as u16) << 8;

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
pub fn push_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.registers.read16(r16);
    push_stack_d16(cpu, value);

    return 16;
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
pub fn pop_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = pop_stack_d16(cpu);
    cpu.registers.write16(r16, value);

    return 12;
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
pub fn call_d16(cpu: &mut CPU) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    return 24;
}

/// Jump to a different address using 16-bit data as an address after first pushing the current PC
/// to the stack if a given flag status condition matches.
///
/// Takes 24 cycles if condition matches, 12 cycles if it doesn't match.
///
/// # Examples
///
/// ```asm
/// CALL NZ $0150 ; IF !Flags::RegisterFlags::ZERO { STACK <<- PC; PC <- 0x0150 }
/// ```
pub fn call_conditional_d16(cpu: &mut CPU, opcode: u8) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    if test_jump_condition(cpu, opcode) {
        // Push current PC to the stack
        let current_pc = cpu.registers.pc;
        push_stack_d16(cpu, current_pc);

        // Jump PC to the target address
        cpu.registers.pc = jump_target;

        return 24;
    } else {
        return 12;
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
pub fn ret(cpu: &mut CPU) -> u8 {
    // Read 16-bit jump target address
    let jump_target: u16 = pop_stack_d16(cpu);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    return 16;
}

/// Called internally by the CPU to jump to a different address using 16-bit interrupt handler
/// address after first pushing the current PC.
///
/// Takes 12 cycles.
pub fn call_interrupt(cpu: &mut CPU, addr: u16) -> u8 {
    // Disable further interrupts
    cpu.ime = false;

    // Save current PC on the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Jump to handler
    cpu.registers.pc = addr;

    return 12;
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
pub fn ret_interrupt(cpu: &mut CPU) -> u8 {
    cpu.ime = true;

    return ret(cpu);
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
pub fn ret_conditional(cpu: &mut CPU, opcode: u8) -> u8 {
    if test_jump_condition(cpu, opcode) {
        ret(cpu);

        return 20;
    } else {
        return 8;
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
pub fn reset(cpu: &mut CPU, opcode: u8) -> u8 {
    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Derive target address from opcode bits
    let jump_target = opcode & 0b00111000;

    // Jump PC to the target address
    cpu.registers.pc = jump_target as u16;

    return 16;
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
pub fn add_hl_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.registers.read16(r16);
    let original_hl = cpu.registers.read16(&Reg16::HL);

    let combined = original_hl.wrapping_add(value);

    cpu.registers.write16(&Reg16::HL, combined);

    // No change for RegisterFlags::ZERO
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        ((original_hl & 0x0F00) + (value & 0x0F00)) > 0x0F00,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        ((original_hl as u32) + (value as u32)) > 0xFFFF,
    );

    return 8;
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
pub fn add_sp_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8() as u16;

    let original_sp = cpu.registers.sp;

    cpu.registers.sp = original_sp.wrapping_add(value);

    cpu.registers.f.set(RegisterFlags::ZERO, false);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        ((original_sp & 0x0F00) + (value & 0x0F00)) > 0x0F00,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        ((original_sp as u32) + (value as u32)) > 0xFFFF,
    );

    return 16;
}

/// Add 8-bit value with register A, storing the result in A.
/// If use_carry is true this will add the content of the carry flag along with the value and take
/// that into account when updating flags too.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always unset
///
/// ## RegisterFlags::HALF_CARRY
///
/// Set if the lower nibble of the value added to the lower nibble of A was too large to fit in a u4
///
/// ## RegisterFlags::CARRY
///
/// Set if the value added to A would have been too large to fit in a u8
///
fn alu_add_8bit(cpu: &mut CPU, d8: u8, use_carry: bool) -> () {
    let original_a = cpu.registers.a;

    let cy = if use_carry && cpu.registers.f.contains(RegisterFlags::CARRY) {
        1
    } else {
        0
    };

    cpu.registers.a = original_a.wrapping_add(d8).wrapping_add(cy);

    cpu.registers.f.set(RegisterFlags::ZERO, original_a == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        ((original_a & 0x0F) + (d8 & 0x0F) + cy) > 0x0F,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        ((original_a as u16) + (d8 as u16) + (cy as u16)) > 0xFF,
    );
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
pub fn add_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_add_8bit(cpu, value, false);

    return 4;
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
pub fn add_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_add_8bit(cpu, value, false);

    return 8;
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
pub fn add_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_add_8bit(cpu, value, false);

    return 8;
}

/// ADD 8-bit register plus the carry flag with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// ADC B ; A <- A + B + Flag::RegisterFlags::CARRY
/// ```
pub fn adc_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_add_8bit(cpu, value, true);

    return 4;
}

/// ADD 8-bit register plus the carry flag with register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADC $DA ; A <- A + 0x0DA + Flag::RegisterFlags::CARRY
/// ```
pub fn adc_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_add_8bit(cpu, value, true);

    return 8;
}

/// ADD memory addressed indirectly by a 16-bit register plus the carry flag with register A,
/// storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// ADC (HL) ; A <- A + memory[HL] + Flag::RegisterFlags::CARRY
/// ```
pub fn adc_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_add_8bit(cpu, value, true);

    return 8;
}

/// Subtract 8-bit value from register A, storing the result in A.
/// If use_carry is true this will subtract the content of the carry flag along with the value and
/// take that into account when updating flags too.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always set
///
/// ## RegisterFlags::HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## RegisterFlags::CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
fn alu_sub_8bit(cpu: &mut CPU, d8: u8, use_carry: bool) -> () {
    let original_a = cpu.registers.a;

    let cy = if use_carry && cpu.registers.f.contains(RegisterFlags::CARRY) {
        1
    } else {
        0
    };

    cpu.registers.a = original_a.wrapping_sub(d8).wrapping_sub(cy);

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, true);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        (original_a & 0x0F) < (d8 & 0x0F) + cy,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        (original_a as u16) < (d8 as u16) + (cy as u16),
    );
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
pub fn sub_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_sub_8bit(cpu, value, false);

    return 4;
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
pub fn sub_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_sub_8bit(cpu, value, false);

    return 8;
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
pub fn sub_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_sub_8bit(cpu, value, false);

    return 8;
}

/// Subtract 8-bit register plus the carry flag from register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SBC B ; A <- A - B - Flag::RegisterFlags::CARRY
/// ```
pub fn sbc_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_sub_8bit(cpu, value, true);

    return 4;
}

/// Subtract 8-bit value plus the carry flag from register A, storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SBC $DA ; A <- A - 0xDA - Flag::RegisterFlags::CARRY
/// ```
pub fn sbc_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_sub_8bit(cpu, value, true);

    return 8;
}

/// Subtract memory addressed indirectly by a 16-bit register plus the carry flag from register A,
/// storing the result in A.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// SBC (HL) ; A <- A - memory[HL] - Flag::RegisterFlags::CARRY
/// ```
pub fn sbc_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_sub_8bit(cpu, value, true);

    return 8;
}

/// Logically AND 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always reset
///
/// ## RegisterFlags::HALF_CARRY
///
/// Always set
///
/// ## RegisterFlags::CARRY
///
/// Always reset
///
fn alu_and_8bit(cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a & d8;

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, true);
    cpu.registers.f.set(RegisterFlags::CARRY, false);
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
pub fn and_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_and_8bit(cpu, value);

    return 4;
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
pub fn and_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_and_8bit(cpu, value);

    return 8;
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
pub fn and_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_and_8bit(cpu, value);

    return 8;
}

/// Logically XOR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always reset
///
/// ## RegisterFlags::HALF_CARRY
///
/// Always reset
///
/// ## RegisterFlags::CARRY
///
/// Always reset
///
fn alu_xor_8bit(cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a ^ d8;

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, false);
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
pub fn xor_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_xor_8bit(cpu, value);

    return 4;
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
pub fn xor_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_xor_8bit(cpu, value);

    return 8;
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
pub fn xor_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_xor_8bit(cpu, value);

    return 8;
}

/// Logically OR 8-bit value from register A, storing the result in A.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always reset
///
/// ## RegisterFlags::HALF_CARRY
///
/// Always reset
///
/// ## RegisterFlags::CARRY
///
/// Always reset
///
fn alu_or_8bit(cpu: &mut CPU, d8: u8) -> () {
    cpu.registers.a = cpu.registers.a | d8;

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, false);
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
pub fn or_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_or_8bit(cpu, value);

    return 4;
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
pub fn or_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_or_8bit(cpu, value);

    return 8;
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
pub fn or_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_or_8bit(cpu, value);

    return 8;
}

/// Compare an 8-bit value with register A by subtracting the two but not storing the result, only
/// the flags.
///
/// Update flags:
///
/// ## RegisterFlags::ZERO
///
/// Set if the result equals zero.
///
/// ## RegisterFlags::SUBTRACT
///
/// Always set
///
/// ## RegisterFlags::HALF_CARRY
///
/// Set if the lower nibble of the value subtracted from the lower nibble of A would have attempted
/// to borrow a bit. i.e. the lower nibble of the value is larger than the lower nibble of A.
///
/// ## RegisterFlags::CARRY
///
/// Set if the value subtracted from A would have required a borrow, otherwise reset
///
fn alu_cp_8bit(cpu: &mut CPU, d8: u8) -> () {
    cpu.registers
        .f
        .set(RegisterFlags::ZERO, cpu.registers.a.wrapping_sub(d8) == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, true);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        (cpu.registers.a & 0x0F) < (d8 & 0x0F),
    );
    cpu.registers
        .f
        .set(RegisterFlags::CARRY, cpu.registers.a < d8);
}

/// Subtract 8-bit register from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit operand register were equal.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CP B ; Flag::RegisterFlags::ZERO true if A == B, Flag::RegisterFlags::CARRY true if A < B
/// ```
pub fn cp_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    alu_cp_8bit(cpu, value);

    return 4;
}

/// Subtract 8-bit value from register A, but don't store the result. Zero flag will be set if
/// register A and the 8-bit value were equal.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// CP $DA ; Flag::RegisterFlags::ZERO true if A == 0xDA, Flag::RegisterFlags::CARRY true if A < 0xDA
/// ```
pub fn cp_d8(cpu: &mut CPU) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    alu_cp_8bit(cpu, value);

    return 8;
}

/// Subtract memory addressed indirectly by a 16-bit register from register A, but don't store the
/// result. Zero flag will be set if register A and the 8-bit value were equal.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// CP (HL) ; Flag::RegisterFlags::ZERO true if A == memory[HL], Flag::RegisterFlags::CARRY true if A < memory[HL]
/// ```
pub fn cp_indirect_r16(cpu: &mut CPU, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    alu_cp_8bit(cpu, value);

    return 8;
}

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
fn alu_rotate_left(cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(RegisterFlags::CARRY) {
        1
    } else {
        0
    };
    let high_bit = (d8 & 0x80) >> 7;
    let new_low_bit = if through_carry { cy } else { high_bit };
    let rotated_value = (d8 << 1) | new_low_bit;

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, high_bit != 0);

    return rotated_value;
}

/// Rotate an 8-bit register to the left.
///
/// If through_carry is true then the high bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 4 cycles if always using A, otherwise 8 cycles
///
/// # Examples
///
/// ```asm
/// ; 4 cycle
/// RLCA  ; Rotate A left (resets Flag::RegisterFlags::ZERO)
/// RLA   ; Rotate A left through the carry flag (resets Flag::RegisterFlags::ZERO)
///
/// ; 8 cycle
/// RLC B ; Rotate B left (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// RL B  ; Rotate B left through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_left_r8(cpu: &mut CPU, r8: &Reg8, through_carry: bool, reset_zero: bool) -> u8 {
    let value = cpu.registers.read8(r8);

    let rotated_value = alu_rotate_left(cpu, value, through_carry, reset_zero);

    cpu.registers.write8(r8, rotated_value);

    return if reset_zero { 4 } else { 8 };
}

/// Rotate an indirect value, taken from memory using a 16-bit register as an address to the left.
///
/// If through_carry is true then the high bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new low bit.
/// If it is not true the high bit will become the low bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RLC (HL) ; Rotate memory[hl] left (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// RL (HL)  ; Rotate memory[hl] left through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_left_indirect_hl(cpu: &mut CPU, through_carry: bool, reset_zero: bool) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.mmu.write8(a16_addr, rotated_value);

    return 16;
}

/// Rotate an 8-bit value to the right.
///
/// If through_carry is true then the low bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
fn alu_rotate_right(cpu: &mut CPU, d8: u8, through_carry: bool, reset_zero: bool) -> u8 {
    let cy = if cpu.registers.f.contains(RegisterFlags::CARRY) {
        1
    } else {
        0
    };
    let low_bit = d8 & 0x01;
    let new_high_bit = if through_carry { cy } else { low_bit };

    let rotated_value = (d8 >> 1) | (new_high_bit << 7);

    cpu.registers
        .f
        .set(RegisterFlags::ZERO, rotated_value == 0 && !reset_zero);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, low_bit != 0);

    return rotated_value;
}

/// Rotate an 8-bit register to the right.
///
/// If through_carry is true then the low bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 4 cycles if always using A, otherwise 8 cycles
///
/// # Examples
///
/// ```asm
/// ; 4 cycle
/// RRCA  ; Rotate A right (resets Flag::RegisterFlags::ZERO)
/// RRA   ; Rotate A right through the carry flag (resets Flag::RegisterFlags::ZERO)
///
/// ; 8 cycle
/// RRC B ; Rotate B right (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// RR B  ; Rotate B right through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_right_r8(cpu: &mut CPU, r8: &Reg8, through_carry: bool, reset_zero: bool) -> u8 {
    let value = cpu.registers.read8(r8);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.registers.write8(r8, rotated_value);

    return if reset_zero { 4 } else { 8 };
}

/// Rotate an indirect value, taken from memory using a 16-bit register as an address to the right.
///
/// If through_carry is true then the low bit will go into the RegisterFlags::CARRY flag and the old value of the
/// RegisterFlags::CARRY flag will become the new high bit.
/// If it is not true the low bit will become the high bit as well as going into the RegisterFlags::CARRY flag.
///
/// If reset_zero is true the RegisterFlags::ZERO flag will always be reset.
/// If it is not true the RegisterFlags::ZERO flag will be set only if the rotated value equals zero.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RRC (HL) ; Rotate memory[hl] right (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// RR (HL)  ; Rotate memory[hl] right through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
///
/// ```
pub fn rotate_right_indirect_hl(cpu: &mut CPU, through_carry: bool, reset_zero: bool) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let rotated_value = alu_rotate_right(cpu, value, through_carry, reset_zero);

    cpu.mmu.write8(a16_addr, rotated_value);

    return 16;
}

/// Shift an 8-bit value to the left.
///
fn alu_shift_left(cpu: &mut CPU, d8: u8) -> u8 {
    let high_bit = d8 & 0x80;
    let shifted_value = d8 << 1;

    cpu.registers.f.set(RegisterFlags::ZERO, shifted_value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, high_bit != 0);

    return shifted_value;
}

/// Shift an 8-bit register to the left.
///
/// Takes 8 cycles
///
/// # Examples
///
/// ```asm
/// SLA B  ; Shift B left through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// ```
pub fn shift_left_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let shifted_value = alu_shift_left(cpu, value);

    cpu.registers.write8(r8, shifted_value);

    return 8;
}

/// Shift an indirect value, taken from memory using a 16-bit register as an address to the left.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// SLA (HL) ; Shift memory[hl] left (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// ```
pub fn shift_left_indirect_hl(cpu: &mut CPU) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let shifted_value = alu_shift_left(cpu, value);

    cpu.mmu.write8(a16_addr, shifted_value);

    return 16;
}

/// Shift an 8-bit value to the right.
///
fn alu_shift_right(cpu: &mut CPU, d8: u8, reset_high_bit: bool) -> u8 {
    let high_bit = if reset_high_bit { 0 } else { d8 & 0x80 };
    let low_bit = d8 & 0x01;
    let shifted_value = (d8 >> 1) | high_bit;

    cpu.registers.f.set(RegisterFlags::ZERO, shifted_value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, low_bit != 0);

    return shifted_value;
}

/// Shift an 8-bit register to the right.
///
/// If reset_high_bit is true the highest bit after the shift will always be reset.
/// If it is not true the highest bit after the shift will be left as its original value.
///
/// Takes 8 cycles
///
/// # Examples
///
/// ```asm
/// SRA B  ; Shift B right through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// SRL B  ; Shift B right through the carry flag (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// ```
pub fn shift_right_r8(cpu: &mut CPU, r8: &Reg8, reset_high_bit: bool) -> u8 {
    let value = cpu.registers.read8(r8);

    let shifted_value = alu_shift_right(cpu, value, reset_high_bit);

    cpu.registers.write8(r8, shifted_value);

    return 8;
}

/// Shift an indirect value, taken from memory using a 16-bit register as an address to the right.
///
/// If reset_high_bit is true the highest bit after the shift will always be reset.
/// If it is not true the highest bit after the shift will be left as its original value.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// SRA (HL) ; Shift memory[hl] right (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// SRL (HL) ; Shift memory[hl] right (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// ```
pub fn shift_right_indirect_hl(cpu: &mut CPU, reset_high_bit: bool) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let shifted_value = alu_shift_right(cpu, value, reset_high_bit);

    cpu.mmu.write8(a16_addr, shifted_value);

    return 16;
}

/// Swap high and low bits of an 8-bit value.
///
fn alu_swap(cpu: &mut CPU, d8: u8) -> u8 {
    let swapped_value = (d8 & 0x0F << 4) & (d8 & 0xF0 >> 4);

    cpu.registers.f.set(RegisterFlags::ZERO, swapped_value == 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, false);

    return swapped_value;
}

/// Swap high and low bits of an 8-bit register.
///
/// Takes 8 cycles
///
/// # Examples
///
/// ```asm
/// SWAP B  ; B = (B & 0x0F << 4) & (B & 0xF0 >> 4)
/// ```
pub fn swap_r8(cpu: &mut CPU, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let swapped_value = alu_swap(cpu, value);

    cpu.registers.write8(r8, swapped_value);

    return 8;
}

/// Swap high and low bits of an indirect value, taken from memory using a 16-bit register as an
/// address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// SLA (HL) ; Shift memory[hl] left (sets Flag::RegisterFlags::ZERO if rotated result == 0)
/// ```
pub fn swap_indirect_hl(cpu: &mut CPU) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let swapped_value = alu_swap(cpu, value);

    cpu.mmu.write8(a16_addr, swapped_value);

    return 16;
}

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
    let register = opcode & 0b00000111;
    let bit_index = (opcode & 0b00111000) >> 3;

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

    cpu.registers.f.set(RegisterFlags::ZERO, tested_value != 0);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(RegisterFlags::HALF_CARRY, false);
    cpu.registers.f.set(RegisterFlags::CARRY, false);

    return duration;
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
    let register = opcode & 0b00000111;
    let bit_index = (opcode & 0b00111000) >> 3;

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
        value = value | (0x01 << bit_index);
    } else {
        value = value & !(0x01 << bit_index);
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

    return duration;
}
