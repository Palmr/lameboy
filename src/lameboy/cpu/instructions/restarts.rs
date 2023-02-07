use lameboy::cpu::instructions::stack::push_stack_d16;
use lameboy::cpu::Cpu;

/// Push the current PC to the stack and then jump to one of 8 positions in the zero page.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RST 1 ; STACK <<- PC; PC <- 0x0008
/// ```
pub fn restart(cpu: &mut Cpu, opcode: u8) -> u8 {
    // Push current PC to the stack
    let current_pc = cpu.registers.pc;
    push_stack_d16(cpu, current_pc);

    // Derive target address from opcode bits
    let jump_target = opcode & 0b0011_1000;

    // Jump PC to the target address
    cpu.registers.pc = u16::from(jump_target);

    16
}
