use lameboy::cpu::instructions::alu::{
    alu_rotate_left, alu_rotate_right, alu_shift_left, alu_shift_right,
};
use lameboy::cpu::registers::{Reg16, Reg8};
use lameboy::cpu::CPU;

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

    if reset_zero {
        4
    } else {
        8
    }
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

    16
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

    if reset_zero {
        4
    } else {
        8
    }
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

    16
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

    8
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

    16
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

    8
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

    16
}
