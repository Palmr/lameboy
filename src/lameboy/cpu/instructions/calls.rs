use crate::lameboy::cpu::instructions::opcode_flag_test;
use crate::lameboy::cpu::instructions::stack::push_stack_d16;
use crate::lameboy::cpu::Cpu;

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
pub fn call_d16(cpu: &mut Cpu) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    24
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
pub fn call_conditional_d16(cpu: &mut Cpu, opcode: u8) -> u8 {
    // Read 16-bit jump target address
    let jump_target = cpu.fetch16();

    if opcode_flag_test(opcode, cpu.registers.f) {
        // Push current PC to the stack
        let current_pc = cpu.registers.pc;
        push_stack_d16(cpu, current_pc);

        // Jump PC to the target address
        cpu.registers.pc = jump_target;

        24
    } else {
        12
    }
}

/// Called internally by the CPU to jump to a different address using 16-bit interrupt handler
/// address after first pushing the current PC.
///
/// Takes 12 cycles.
pub fn call_interrupt(cpu: &mut Cpu, addr: u16) -> u8 {
    // Disable further interrupts
    cpu.ime = false;

    // Save current PC on the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Jump to handler
    cpu.registers.pc = addr;

    12
}
