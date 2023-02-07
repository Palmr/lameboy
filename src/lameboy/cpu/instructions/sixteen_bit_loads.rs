use lameboy::cpu::instructions::stack::{pop_stack_d16, push_stack_d16};
use lameboy::cpu::registers::{Flags, Reg16};
use lameboy::cpu::Cpu;

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
pub fn load_r16_d16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    // Read 16-bit value
    let value: u16 = cpu.fetch16();

    // Write it to the register
    cpu.registers.write16(r16, value);

    12
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
pub fn load_r16_r16(cpu: &mut Cpu, r16_target: &Reg16, r16_source: &Reg16) -> u8 {
    // Copy from source register to target register
    let value = cpu.registers.read16(r16_source);
    cpu.registers.write16(r16_target, value);

    8
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
pub fn load_reg_hl_reg_sp_d8(cpu: &mut Cpu) -> u8 {
    // TODO - Could combine logic with add_sp_d8
    // Read 8-bit value
    let unsigned_value = cpu.fetch8();
    let signed_value = unsigned_value as i8;

    let combined = cpu.registers.sp.wrapping_add(signed_value as u16);

    cpu.registers.write16(&Reg16::HL, combined);

    cpu.registers.f.set(Flags::ZERO, false);
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(
        Flags::HALF_CARRY,
        ((cpu.registers.sp & 0x0F) + (unsigned_value as u16 & 0x0F)) > 0x0F,
    );
    cpu.registers.f.set(
        Flags::CARRY,
        ((cpu.registers.sp & 0xFF) + (unsigned_value as u16 & 0xFF)) > 0xFF,
    );

    12
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
pub fn load_indirect_a16_r16(cpu: &mut Cpu, r16_source: &Reg16) -> u8 {
    // Read 16-bit address
    let a16_addr = cpu.fetch16();

    // Split 16-bit register to low/high
    let r16_value = cpu.registers.read16(r16_source);
    let r16_high = ((r16_value & 0xFF00) >> 8) as u8;
    let r16_low = (r16_value & 0x00FF) as u8;

    // Write the two bytes to memory
    cpu.mmu.write8(a16_addr, r16_low);
    cpu.mmu.write8(a16_addr + 1, r16_high);

    20
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
pub fn push_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.registers.read16(r16);
    push_stack_d16(cpu, value);

    16
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
pub fn pop_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = pop_stack_d16(cpu);
    cpu.registers.write16(r16, value);

    12
}
