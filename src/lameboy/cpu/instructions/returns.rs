use crate::lameboy::cpu::instructions::opcode_flag_test;
use crate::lameboy::cpu::instructions::stack::pop_stack_d16;
use crate::lameboy::cpu::Cpu;

/// Return to an address that was pushed to the stack.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// RET ; PC <<- STACK;
/// ```
pub fn ret(cpu: &mut Cpu) -> u8 {
    // Read 16-bit jump target address
    let jump_target: u16 = pop_stack_d16(cpu);

    // Jump PC to the target address
    cpu.registers.pc = jump_target;

    16
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
pub fn ret_conditional(cpu: &mut Cpu, opcode: u8) -> u8 {
    if opcode_flag_test(opcode, cpu.registers.f) {
        ret(cpu);

        20
    } else {
        8
    }
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
pub fn ret_interrupt(cpu: &mut Cpu) -> u8 {
    cpu.ime = true;

    ret(cpu)
}
