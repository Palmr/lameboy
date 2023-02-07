use crate::lameboy::cpu::registers::{Reg16, Reg8};
use crate::lameboy::cpu::Cpu;

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
pub fn load_r8_d8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.fetch8();

    cpu.registers.write8(r8, value);

    8
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
pub fn load_reg_a_a16(cpu: &mut Cpu) -> u8 {
    let addr = cpu.fetch16();

    // Read 8-bit value
    let value = cpu.mmu.read8(addr);

    cpu.registers.a = value;

    16
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
pub fn load_r8_r8(cpu: &mut Cpu, r8_target: &Reg8, r8_source: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8_source);
    cpu.registers.write8(r8_target, value);

    4
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
pub fn load_indirect_r16_d8(cpu: &mut Cpu, r16_indirect_addr: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    // Copy from source register to memory using indirect register
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);
    cpu.mmu.write8(indirect_addr, value);

    12
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
pub fn load_r8_indirect_r16(cpu: &mut Cpu, r8_target: &Reg8, r16_indirect_addr: &Reg16) -> u8 {
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);
    let value = cpu.mmu.read8(indirect_addr);
    cpu.registers.write8(r8_target, value);

    8
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
pub fn load_indirect_r16_r8(cpu: &mut Cpu, r16_indirect_addr: &Reg16, r8_source: &Reg8) -> u8 {
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);
    let register_val = cpu.registers.read8(r8_source);
    cpu.mmu.write8(indirect_addr, register_val);

    8
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
pub fn load_a16_reg_a(cpu: &mut Cpu) -> u8 {
    // Read 16-bit address value
    let addr = cpu.fetch16();

    // Write the byte to memory
    cpu.mmu.write8(addr, cpu.registers.a);

    16
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
pub fn load_reg_a_high_mem_reg_c(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 + u16::from(cpu.registers.c);

    cpu.registers.a = cpu.mmu.read8(address);

    8
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
pub fn load_high_mem_reg_c_reg_a(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 + u16::from(cpu.registers.c);

    // Write the byte to memory
    cpu.mmu.write8(address, cpu.registers.a);

    8
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
    cpu: &mut Cpu,
    r8_target: &Reg8,
    r16_indirect_addr: &Reg16,
) -> u8 {
    // Copy from memory using 16-bit register value as address
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(indirect_addr);
    cpu.registers.write8(r8_target, value);

    // Decrement the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, indirect_addr.wrapping_sub(1));

    8
}

/// Load an 8-bit register with an indirect value, taken from memory using a 16-bit register as an
/// address. Then decrement that 16-bit register.
///
/// Takes 8 cycles.
///
/// # Examples
///
/// ```asm
/// LD A, (HL-) ; memory[HL] <- A; HL--
/// ```
pub fn load_indirect_r16_decrement_r8(cpu: &mut Cpu, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);

    // Write to memory using 16-bit register value as address
    cpu.mmu.write8(indirect_addr, value);

    // Decrement the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, indirect_addr.wrapping_sub(1));

    8
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
pub fn load_indirect_r16_increment_r8(cpu: &mut Cpu, r16_indirect_addr: &Reg16, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    // Copy from memory using 16-bit register value as address
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);

    cpu.mmu.write8(indirect_addr, value);

    // Increment the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, indirect_addr.wrapping_add(1));

    8
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
    cpu: &mut Cpu,
    r8_target: &Reg8,
    r16_indirect_addr: &Reg16,
) -> u8 {
    // Copy from memory using 16-bit register value as address
    let indirect_addr = cpu.registers.read16(r16_indirect_addr);

    let value = cpu.mmu.read8(indirect_addr);
    cpu.registers.write8(r8_target, value);

    // Increment the 16-bit indirect address register
    cpu.registers
        .write16(r16_indirect_addr, indirect_addr.wrapping_add(1));

    8
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
pub fn load_reg_a_high_mem_d8(cpu: &mut Cpu) -> u8 {
    // Address is offset plus 8-bit data
    let addr = 0xFF00 + u16::from(cpu.fetch8());

    // Read 8-bit value
    let value = cpu.mmu.read8(addr);

    cpu.registers.a = value;

    12
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
pub fn load_high_mem_d8_reg_a(cpu: &mut Cpu) -> u8 {
    // Read 8-bit value
    let address = 0xFF00 + u16::from(cpu.fetch8());

    // Write the byte to memory
    cpu.mmu.write8(address, cpu.registers.a);

    12
}
