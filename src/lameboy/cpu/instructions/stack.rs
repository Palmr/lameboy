use crate::lameboy::cpu::Cpu;

/// Push an 8-bit value to the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
pub fn push_stack_d8(cpu: &mut Cpu, d8: u8) {
    // Decrement stack pointer
    cpu.registers.sp = cpu.registers.sp.wrapping_sub(1);

    // Write byte to stack
    cpu.mmu.write8(cpu.registers.sp, d8);
}

/// Push a 16-bit value to the stack.
/// Pushing the high byte of the value first, then the low byte.
pub fn push_stack_d16(cpu: &mut Cpu, d16: u16) {
    // Write high byte
    push_stack_d8(cpu, ((d16 >> 8) & 0xFF) as u8);
    // Write low byte
    push_stack_d8(cpu, (d16 & 0xFF) as u8);
}

/// Pop an 8-bit value off the stack.
/// Decrements the stack pointer and then writes the 8-bit value using the new stack pointer value.
pub fn pop_stack_d8(cpu: &mut Cpu) -> u8 {
    // Read byte from stack
    let value = cpu.mmu.read8(cpu.registers.sp);

    // Increment stack pointer
    cpu.registers.sp = cpu.registers.sp.wrapping_add(1);

    value
}

/// Pop a 16-bit value off the stack.
/// Pushing the high byte of the value first, then the low byte.
pub fn pop_stack_d16(cpu: &mut Cpu) -> u16 {
    let mut value: u16;
    // Pop low byte
    value = u16::from(pop_stack_d8(cpu));
    // Pop high byte
    value |= (u16::from(pop_stack_d8(cpu))) << 8;

    value
}
