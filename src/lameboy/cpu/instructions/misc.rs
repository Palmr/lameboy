use crate::lameboy::cpu::instructions::alu::alu_swap_8bit;
use crate::lameboy::cpu::registers::{Flags, Reg16, Reg8};
use crate::lameboy::cpu::{Cpu, InterruptFlagDelayStatus};

/// Swap high and low bits of an 8-bit register.
///
/// Takes 8 cycles
///
/// # Examples
///
/// ```asm
/// SWAP B  ; B = (B & 0x0F << 4) & (B & 0xF0 >> 4)
/// ```
pub fn swap_r8(cpu: &mut Cpu, r8: &Reg8) -> u8 {
    let value = cpu.registers.read8(r8);

    let (swapped_value, flags) = alu_swap_8bit(value, cpu.registers.f);
    cpu.registers.write8(r8, swapped_value);
    cpu.registers.f = flags;

    8
}

/// Swap high and low bits of an indirect value, taken from memory using a 16-bit register as an
/// address.
///
/// Takes 16 cycles.
///
/// # Examples
///
/// ```asm
/// SLA (HL) ; Shift memory[hl] left (sets Flags::ZERO if rotated result == 0)
/// ```
pub fn swap_indirect_hl(cpu: &mut Cpu) -> u8 {
    let a16_addr = cpu.registers.read16(&Reg16::HL);
    let value = cpu.mmu.read8(a16_addr);

    let (swapped_value, flags) = alu_swap_8bit(value, cpu.registers.f);
    cpu.mmu.write8(a16_addr, swapped_value);
    cpu.registers.f = flags;

    16
}

/// This instruction conditionally adjusts the accumulator for BCD addition and subtraction
/// operations.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// DAA
/// ```
pub fn decimal_adjust(cpu: &mut Cpu) -> u8 {
    let mut carry = false;

    if !cpu.registers.f.contains(Flags::SUBTRACT) {
        if cpu.registers.f.contains(Flags::CARRY) || cpu.registers.a > 0x99 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x60);
            carry = true;
        }
        if cpu.registers.f.contains(Flags::HALF_CARRY) || cpu.registers.a & 0x0F > 0x09 {
            cpu.registers.a = cpu.registers.a.wrapping_add(0x06);
        }
    } else if cpu.registers.f.contains(Flags::CARRY) {
        carry = true;
        cpu.registers.a =
            cpu.registers
                .a
                .wrapping_add(if cpu.registers.f.contains(Flags::HALF_CARRY) {
                    0x9A
                } else {
                    0xA0
                });
    } else if cpu.registers.f.contains(Flags::HALF_CARRY) {
        cpu.registers.a = cpu.registers.a.wrapping_add(0xFA);
    }

    cpu.registers.f.set(Flags::ZERO, cpu.registers.a == 0);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, carry);

    4
}

/// Complement A register, flipping all bits.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CPL
/// ```
pub fn complement(cpu: &mut Cpu) -> u8 {
    let value = cpu.registers.a;
    cpu.registers.a = !value;

    cpu.registers.f.set(Flags::SUBTRACT, true);
    cpu.registers.f.set(Flags::HALF_CARRY, true);

    4
}

/// Complement the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// CCF ; Flags::CARRY = !Flags::CARRY
/// ```
pub fn complement_carry_flag(cpu: &mut Cpu) -> u8 {
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.toggle(Flags::CARRY);

    4
}

/// Set the carry flag.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// SCF ; Flags::CARRY = 1
/// ```
pub fn set_carry_flag(cpu: &mut Cpu) -> u8 {
    cpu.registers.f.set(Flags::SUBTRACT, false);
    cpu.registers.f.set(Flags::HALF_CARRY, false);
    cpu.registers.f.set(Flags::CARRY, true);

    4
}

/// No operation instruction, does nothing.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// NOP
/// ```
pub fn nop(_: &Cpu) -> u8 {
    // Do nothing
    4
}

/// Halt the system clock, though let the oscillator and LCD controller continue to run.
///
/// Halt mode can be cancelled by an interrupt or reset signal.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// HALT
/// ```
pub fn halt(cpu: &mut Cpu) -> u8 {
    cpu.halt = true;

    4
}

/// Stop the system clock and oscillator circuit to stop the CPU and LCD controller.
///
/// Stop mode can be cancelled by a reset signal.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// STOP
/// ```
pub fn stop(cpu: &mut Cpu) -> u8 {
    // Read 8-bit value
    let value = cpu.fetch8();

    if value != 0 {
        panic!(
            "Stop instruction should be followed by a zero but found: 0x{:02X} at pc=0x{:04X}",
            value, cpu.registers.pc
        );
    }

    // TODO - Halt the CPU & LCD display until a button is pressed
    debug!("Stop called...");

    4
}

/// Enable or disable interrupts.
///
/// Takes 4 cycles.
///
/// # Examples
///
/// ```asm
/// DI
/// EI
/// ```
pub fn interrupts(cpu: &mut Cpu, enabled: bool) -> u8 {
    if enabled {
        cpu.ie_delay_state = InterruptFlagDelayStatus::ChangeScheduled;
    } else {
        cpu.de_delay_state = InterruptFlagDelayStatus::ChangeScheduled;
    }

    4
}

pub fn undefined(cpu: &Cpu, opcode: u8) -> u8 {
    panic!(
        "Undefined opcode 0x{:02X} at pc=0x{:04X}",
        opcode, cpu.registers.pc
    )
}
