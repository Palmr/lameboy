use lameboy::cpu::instructions::alu::{
    alu_add_8bit, alu_and_8bit, alu_cp_8bit, alu_dec_8bit, alu_inc_8bit, alu_or_8bit, alu_sub_8bit,
    alu_xor_8bit,
};
use lameboy::cpu::registers::{Reg16, Reg8};
use lameboy::cpu::Cpu;

/// ADD 8-bit register with register A, storing the result in A.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// ADD B ; A <- A + B
/// ```
pub fn add_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn add_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn add_d8(cpu: &mut Cpu) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn adc_d8(cpu: &mut Cpu) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn adc_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn adc_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_add_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn sub_d8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch8();

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn sub_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn sub_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, false);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn sbc_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn sbc_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn sbc_d8(cpu: &mut Cpu) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    let (acc, flags) = alu_sub_8bit(cpu.registers.a, cpu.registers.f, value, true);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn and_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_and_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn and_d8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch8();

    let (acc, flags) = alu_and_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn and_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    // Read 8-bit value
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_and_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn xor_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_xor_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn xor_d8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch8();

    let (acc, flags) = alu_xor_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn xor_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_xor_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn or_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (acc, flags) = alu_or_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    4
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
pub fn or_d8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch8();

    let (acc, flags) = alu_or_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn or_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    let (acc, flags) = alu_or_8bit(cpu.registers.a, cpu.registers.f, value);
    cpu.registers.a = acc;
    cpu.registers.f = flags;

    8
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
pub fn cp_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    cpu.registers.f = alu_cp_8bit(cpu.registers.a, cpu.registers.f, value);

    4
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
pub fn cp_d8(cpu: &mut Cpu) -> u8 {
    let value = cpu.fetch8();

    cpu.registers.f = alu_cp_8bit(cpu.registers.a, cpu.registers.f, value);

    8
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
pub fn cp_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let value = cpu.mmu.read8(cpu.registers.read16(r16));

    cpu.registers.f = alu_cp_8bit(cpu.registers.a, cpu.registers.f, value);

    8
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
pub fn inc_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let d8 = cpu.registers.read8(r8);

    let (value, flags) = alu_inc_8bit(d8, cpu.registers.f);
    cpu.registers.f = flags;

    cpu.registers.write8(r8, value);

    4
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
pub fn inc_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);
    let d8 = cpu.mmu.read8(a16_addr);

    let (value, flags) = alu_inc_8bit(d8, cpu.registers.f);
    cpu.registers.f = flags;

    cpu.mmu.write8(a16_addr, value);

    12
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
#[allow(clippy::verbose_bit_mask)]
pub fn dec_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let d8 = cpu.registers.read8(r8);

    let (value, flags) = alu_dec_8bit(d8, cpu.registers.f);
    cpu.registers.f = flags;

    cpu.registers.write8(r8, value);

    4
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
pub fn dec_indirect_r16(cpu: &mut Cpu, r16: &Reg16) -> u8 {
    let a16_addr = cpu.registers.read16(r16);
    let d8 = cpu.mmu.read8(a16_addr);

    let (value, flags) = alu_dec_8bit(d8, cpu.registers.f);
    cpu.registers.f = flags;

    cpu.mmu.write8(a16_addr, value);

    12
}
