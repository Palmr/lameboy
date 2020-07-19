use lameboy::cpu::registers::Flags as RegisterFlags;
use lameboy::cpu::registers::Reg16;
use lameboy::cpu::CPU;

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
        ((original_hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        (u32::from(original_hl) + u32::from(value)) > 0xFFFF,
    );

    8
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
    let unsigned_value = cpu.fetch8();
    let signed_value = unsigned_value as i8;

    let original_sp = cpu.registers.sp;

    cpu.registers.sp = original_sp.wrapping_add(signed_value as u16);

    cpu.registers.f.set(RegisterFlags::ZERO, false);
    cpu.registers.f.set(RegisterFlags::SUBTRACT, false);
    cpu.registers.f.set(
        RegisterFlags::HALF_CARRY,
        ((original_sp & 0x0F) + (unsigned_value as u16 & 0x0F)) > 0x0F,
    );
    cpu.registers.f.set(
        RegisterFlags::CARRY,
        ((original_sp & 0xFF) + (unsigned_value as u16 & 0xFF)) > 0xFF,
    );

    16
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

    8
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

    8
}