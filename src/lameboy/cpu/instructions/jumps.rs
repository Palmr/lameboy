use lameboy::cpu::instructions::opcode_flag_test;
use lameboy::cpu::registers::Reg16;
use lameboy::cpu::Cpu;

/// Jump to a different address using 16-bit data as an address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// JP $0150 ; PC <- 0x0150
/// ```
pub fn jump_d16(cpu: &mut Cpu) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Jump PC to that target address
    cpu.registers.pc = jump_target;

    16
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
pub fn jump_conditional_d16(cpu: &mut Cpu, opcode: u8) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Test if the condition matches and if we need to jump
    if opcode_flag_test(opcode, cpu.registers.f) {
        // Jump PC to that target address
        cpu.registers.pc = jump_target;

        16
    } else {
        12
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
pub fn jump_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    // Set PC to whatever the 16-bit register is
    cpu.registers.pc = cpu.registers.read16(r16);

    4
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
pub fn jump_relative_d8(cpu: &mut Cpu) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.fetch8() as i8;

    // Jump PC to that target address
    cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

    16
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
pub fn jump_relative_conditional_d8(cpu: &mut Cpu, opcode: u8) -> u8 {
    // Read signed 8-bit jump offset
    let jump_offset: i8 = cpu.fetch8() as i8;

    // Test if the condition matches and if we need to jump
    if opcode_flag_test(opcode, cpu.registers.f) {
        // Jump PC to that target address
        cpu.registers.pc = cpu.registers.pc.wrapping_add(jump_offset as u16);

        12
    } else {
        8
    }
}
