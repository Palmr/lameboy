use lameboy::cpu::instructions::*;
use lameboy::cpu::registers::*;
use lameboy::interrupts::*;
use lameboy::mmu::MMU;

pub mod instructions;
pub mod registers;

mod debug;

enum InterruptFlagDelayStatus {
    Waiting,
    ChangeScheduled,
    FinishedDelay,
}

pub struct CPU {
    pub registers: Registers,
    pub mmu: MMU,
    ie_delay_state: InterruptFlagDelayStatus,
    de_delay_state: InterruptFlagDelayStatus,
    ime: bool,
    halt: bool,
    pub pc_history: Vec<u16>,
    pub pc_history_pointer: usize,
}

impl CPU {
    pub fn new(mmu: MMU) -> CPU {
        let pc_history = vec![0x00; 200];

        CPU {
            registers: Registers::new(),
            mmu,
            ie_delay_state: InterruptFlagDelayStatus::Waiting,
            de_delay_state: InterruptFlagDelayStatus::Waiting,
            ime: true,
            halt: false,
            pc_history,
            pc_history_pointer: 0,
        }
    }

    /// Set the registers and memory up as if the DMG boot rom had just finished loading and handed
    /// execution to the game.
    pub fn reset(&mut self) {
        self.registers.reset();
        //self.ime = true;
        self.halt = false;
    }

    /// Read an 8-bit value using the PC register as the address, then move the PC register forward
    /// by one.
    pub fn fetch8(&mut self) -> u8 {
        // Read 16-bit value
        let value: u8 = self.mmu.read8(self.registers.pc);

        // Move PC on
        self.registers.pc = self.registers.pc.wrapping_add(1);

        value
    }

    /// Read a 16-bit value using the PC register as the address, then move the PC register forward
    /// by two.
    pub fn fetch16(&mut self) -> u16 {
        // Read 16-bit value
        let low = self.fetch8();
        let high = self.fetch8();

        u16::from(high) << 8 | u16::from(low)
    }

    /// Run a fetch, decode, and execute cycle on the CPU
    pub fn cycle(&mut self) -> u8 {
        self.pc_history[self.pc_history_pointer as usize] = self.registers.pc;
        self.pc_history_pointer = self.pc_history_pointer.wrapping_add(1) % self.pc_history.len();

        self.handle_ime_delay();

        let mut duration = self.handle_instruction();
        duration += self.handle_interrupt();

        duration
    }

    /// EI/DI toggles IME the cycle after
    fn handle_ime_delay(&mut self) {
        match self.ie_delay_state {
            InterruptFlagDelayStatus::Waiting => {}
            InterruptFlagDelayStatus::ChangeScheduled => {
                self.ie_delay_state = InterruptFlagDelayStatus::FinishedDelay
            }
            InterruptFlagDelayStatus::FinishedDelay => {
                self.ime = true;
                self.ie_delay_state = InterruptFlagDelayStatus::Waiting;
            }
        }
        match self.de_delay_state {
            InterruptFlagDelayStatus::Waiting => {}
            InterruptFlagDelayStatus::ChangeScheduled => {
                self.de_delay_state = InterruptFlagDelayStatus::FinishedDelay
            }
            InterruptFlagDelayStatus::FinishedDelay => {
                self.ime = false;
                self.de_delay_state = InterruptFlagDelayStatus::Waiting;
            }
        }
    }

    fn halt(&mut self) {
        // if interrupt
        self.halt = false;
        // else
        // ??
    }

    pub fn handle_interrupt(&mut self) -> u8 {
        let int_enable_mask = self.mmu.read8(0xFFFF);
        let mut int_flags = self.mmu.read8(0xFF0F);
        if self.ime && int_enable_mask > 0 && int_flags > 0 {
            self.halt = false;
            self.ime = false;

            let duration = match int_enable_mask & int_flags {
                INT_VBLANK => {
                    int_flags &= !INT_VBLANK;
                    call_interrupt(self, 0x0040)
                }
                INT_LCD_STAT => {
                    int_flags &= !INT_LCD_STAT;
                    call_interrupt(self, 0x0048)
                }
                INT_TIME => {
                    int_flags &= !INT_TIME;
                    call_interrupt(self, 0x0050)
                }
                INT_SERIAL => {
                    int_flags &= !INT_SERIAL;
                    call_interrupt(self, 0x0058)
                }
                INT_JOYPAD => {
                    int_flags &= !INT_JOYPAD;
                    call_interrupt(self, 0x0060)
                }
                _ => {
                    self.ime = true;
                    0
                }
            };

            self.mmu.write8(0xFF0F, int_flags);
            return duration;
        }

        0
    }

    fn handle_instruction(&mut self) -> u8 {
        // Fetch
        let op = self.fetch8();

        // Decode & Execute
        match op {
            0x00 => nop(self),
            0x01 => load_r16_d16(self, &Reg16::BC),
            0x02 => load_indirect_r16_r8(self, &Reg16::BC, &Reg8::A),
            0x03 => inc_r16(self, &Reg16::BC),
            0x04 => inc_r8(self, &Reg8::B),
            0x05 => dec_r8(self, &Reg8::B),
            0x06 => load_r8_d8(self, &Reg8::B),
            0x07 => rotate_left_r8(self, &Reg8::A, false, true),
            0x08 => load_indirect_a16_r16(self, &Reg16::SP),
            0x09 => add_hl_r16(self, &Reg16::BC),
            0x0A => load_r8_indirect_r16(self, &Reg8::A, &Reg16::BC),
            0x0B => dec_r16(self, &Reg16::BC),
            0x0C => inc_r8(self, &Reg8::C),
            0x0D => dec_r8(self, &Reg8::C),
            0x0E => load_r8_d8(self, &Reg8::C),
            0x0F => rotate_right_r8(self, &Reg8::A, false, true),

            0x10 => stop(self),
            0x11 => load_r16_d16(self, &Reg16::DE),
            0x12 => load_indirect_r16_r8(self, &Reg16::DE, &Reg8::A),
            0x13 => inc_r16(self, &Reg16::DE),
            0x14 => inc_r8(self, &Reg8::D),
            0x15 => dec_r8(self, &Reg8::D),
            0x16 => load_r8_d8(self, &Reg8::D),
            0x17 => rotate_left_r8(self, &Reg8::A, true, true),
            0x18 => jump_relative_d8(self),
            0x19 => add_hl_r16(self, &Reg16::DE),
            0x1A => load_r8_indirect_r16(self, &Reg8::A, &Reg16::DE),
            0x1B => dec_r16(self, &Reg16::DE),
            0x1C => inc_r8(self, &Reg8::E),
            0x1D => dec_r8(self, &Reg8::E),
            0x1E => load_r8_d8(self, &Reg8::E),
            0x1F => rotate_right_r8(self, &Reg8::A, true, true),

            0x20 => jump_relative_conditional_d8(self, op),
            0x21 => load_r16_d16(self, &Reg16::HL),
            0x22 => load_indirect_r16_increment_r8(self, &Reg16::HL, &Reg8::A),
            0x23 => inc_r16(self, &Reg16::HL),
            0x24 => inc_r8(self, &Reg8::H),
            0x25 => dec_r8(self, &Reg8::H),
            0x26 => load_r8_d8(self, &Reg8::H),
            0x27 => decimal_adjust(self),
            0x28 => jump_relative_conditional_d8(self, op),
            0x29 => add_hl_r16(self, &Reg16::HL),
            0x2A => load_r8_indirect_r16_increment(self, &Reg8::A, &Reg16::HL),
            0x2B => dec_r16(self, &Reg16::HL),
            0x2C => inc_r8(self, &Reg8::L),
            0x2D => dec_r8(self, &Reg8::L),
            0x2E => load_r8_d8(self, &Reg8::L),
            0x2F => complement(self),

            0x30 => jump_relative_conditional_d8(self, op),
            0x31 => load_r16_d16(self, &Reg16::SP),
            0x32 => load_indirect_r16_decrement_r8(self, &Reg16::HL, &Reg8::A),
            0x33 => inc_r16(self, &Reg16::SP),
            0x34 => inc_indirect_r16(self, &Reg16::HL),
            0x35 => dec_indirect_r16(self, &Reg16::HL),
            0x36 => load_indirect_r16_d8(self, &Reg16::HL),
            0x37 => set_carry_flag(self),
            0x38 => jump_relative_conditional_d8(self, op),
            0x39 => add_hl_r16(self, &Reg16::SP),
            0x3A => load_r8_indirect_r16_decrement(self, &Reg8::A, &Reg16::HL),
            0x3B => dec_r16(self, &Reg16::SP),
            0x3C => inc_r8(self, &Reg8::A),
            0x3D => dec_r8(self, &Reg8::A),
            0x3E => load_r8_d8(self, &Reg8::A),
            0x3F => complement_carry_flag(self),

            // Loads
            0x40 => load_r8_r8(self, &Reg8::B, &Reg8::B),
            0x41 => load_r8_r8(self, &Reg8::B, &Reg8::C),
            0x42 => load_r8_r8(self, &Reg8::B, &Reg8::D),
            0x43 => load_r8_r8(self, &Reg8::B, &Reg8::E),
            0x44 => load_r8_r8(self, &Reg8::B, &Reg8::H),
            0x45 => load_r8_r8(self, &Reg8::B, &Reg8::L),
            0x46 => load_r8_indirect_r16(self, &Reg8::B, &Reg16::HL),
            0x47 => load_r8_r8(self, &Reg8::B, &Reg8::A),
            0x48 => load_r8_r8(self, &Reg8::C, &Reg8::B),
            0x49 => load_r8_r8(self, &Reg8::C, &Reg8::C),
            0x4A => load_r8_r8(self, &Reg8::C, &Reg8::D),
            0x4B => load_r8_r8(self, &Reg8::C, &Reg8::E),
            0x4C => load_r8_r8(self, &Reg8::C, &Reg8::H),
            0x4D => load_r8_r8(self, &Reg8::C, &Reg8::L),
            0x4E => load_r8_indirect_r16(self, &Reg8::C, &Reg16::HL),
            0x4F => load_r8_r8(self, &Reg8::C, &Reg8::A),

            0x50 => load_r8_r8(self, &Reg8::D, &Reg8::B),
            0x51 => load_r8_r8(self, &Reg8::D, &Reg8::C),
            0x52 => load_r8_r8(self, &Reg8::D, &Reg8::D),
            0x53 => load_r8_r8(self, &Reg8::D, &Reg8::E),
            0x54 => load_r8_r8(self, &Reg8::D, &Reg8::H),
            0x55 => load_r8_r8(self, &Reg8::D, &Reg8::L),
            0x56 => load_r8_indirect_r16(self, &Reg8::D, &Reg16::HL),
            0x57 => load_r8_r8(self, &Reg8::D, &Reg8::A),
            0x58 => load_r8_r8(self, &Reg8::E, &Reg8::B),
            0x59 => load_r8_r8(self, &Reg8::E, &Reg8::C),
            0x5A => load_r8_r8(self, &Reg8::E, &Reg8::D),
            0x5B => load_r8_r8(self, &Reg8::E, &Reg8::E),
            0x5C => load_r8_r8(self, &Reg8::E, &Reg8::H),
            0x5D => load_r8_r8(self, &Reg8::E, &Reg8::L),
            0x5E => load_r8_indirect_r16(self, &Reg8::E, &Reg16::HL),
            0x5F => load_r8_r8(self, &Reg8::E, &Reg8::A),

            0x60 => load_r8_r8(self, &Reg8::H, &Reg8::B),
            0x61 => load_r8_r8(self, &Reg8::H, &Reg8::C),
            0x62 => load_r8_r8(self, &Reg8::H, &Reg8::D),
            0x63 => load_r8_r8(self, &Reg8::H, &Reg8::E),
            0x64 => load_r8_r8(self, &Reg8::H, &Reg8::H),
            0x65 => load_r8_r8(self, &Reg8::H, &Reg8::L),
            0x66 => load_r8_indirect_r16(self, &Reg8::H, &Reg16::HL),
            0x67 => load_r8_r8(self, &Reg8::H, &Reg8::A),
            0x68 => load_r8_r8(self, &Reg8::L, &Reg8::B),
            0x69 => load_r8_r8(self, &Reg8::L, &Reg8::C),
            0x6A => load_r8_r8(self, &Reg8::L, &Reg8::D),
            0x6B => load_r8_r8(self, &Reg8::L, &Reg8::E),
            0x6C => load_r8_r8(self, &Reg8::L, &Reg8::H),
            0x6D => load_r8_r8(self, &Reg8::L, &Reg8::L),
            0x6E => load_r8_indirect_r16(self, &Reg8::L, &Reg16::HL),
            0x6F => load_r8_r8(self, &Reg8::L, &Reg8::A),

            0x70 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::B),
            0x71 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::C),
            0x72 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::D),
            0x73 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::E),
            0x74 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::H),
            0x75 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::L),
            0x76 => halt(self),
            0x77 => load_indirect_r16_r8(self, &Reg16::HL, &Reg8::A),
            0x78 => load_r8_r8(self, &Reg8::A, &Reg8::B),
            0x79 => load_r8_r8(self, &Reg8::A, &Reg8::C),
            0x7A => load_r8_r8(self, &Reg8::A, &Reg8::D),
            0x7B => load_r8_r8(self, &Reg8::A, &Reg8::E),
            0x7C => load_r8_r8(self, &Reg8::A, &Reg8::H),
            0x7D => load_r8_r8(self, &Reg8::A, &Reg8::L),
            0x7E => load_r8_indirect_r16(self, &Reg8::A, &Reg16::HL),
            0x7F => load_r8_r8(self, &Reg8::A, &Reg8::A),

            // ADDs
            0x80 => add_r8(self, &Reg8::B),
            0x81 => add_r8(self, &Reg8::C),
            0x82 => add_r8(self, &Reg8::D),
            0x83 => add_r8(self, &Reg8::E),
            0x84 => add_r8(self, &Reg8::H),
            0x85 => add_r8(self, &Reg8::L),
            0x86 => add_indirect_r16(self, &Reg16::HL),
            0x87 => add_r8(self, &Reg8::A),
            0x88 => adc_r8(self, &Reg8::B),
            0x89 => adc_r8(self, &Reg8::C),
            0x8A => adc_r8(self, &Reg8::D),
            0x8B => adc_r8(self, &Reg8::E),
            0x8C => adc_r8(self, &Reg8::H),
            0x8D => adc_r8(self, &Reg8::L),
            0x8E => adc_indirect_r16(self, &Reg16::HL),
            0x8F => adc_r8(self, &Reg8::A),

            // SUBs
            0x90 => sub_r8(self, &Reg8::B),
            0x91 => sub_r8(self, &Reg8::C),
            0x92 => sub_r8(self, &Reg8::D),
            0x93 => sub_r8(self, &Reg8::E),
            0x94 => sub_r8(self, &Reg8::H),
            0x95 => sub_r8(self, &Reg8::L),
            0x96 => sub_indirect_r16(self, &Reg16::HL),
            0x97 => sub_r8(self, &Reg8::A),
            0x98 => sbc_r8(self, &Reg8::B),
            0x99 => sbc_r8(self, &Reg8::C),
            0x9A => sbc_r8(self, &Reg8::D),
            0x9B => sbc_r8(self, &Reg8::E),
            0x9C => sbc_r8(self, &Reg8::H),
            0x9D => sbc_r8(self, &Reg8::L),
            0x9E => sbc_indirect_r16(self, &Reg16::HL),
            0x9F => sbc_r8(self, &Reg8::A),

            // ANDs & XORs
            0xA0 => and_r8(self, &Reg8::B),
            0xA1 => and_r8(self, &Reg8::C),
            0xA2 => and_r8(self, &Reg8::D),
            0xA3 => and_r8(self, &Reg8::E),
            0xA4 => and_r8(self, &Reg8::H),
            0xA5 => and_r8(self, &Reg8::L),
            0xA6 => and_indirect_r16(self, &Reg16::HL),
            0xA7 => and_r8(self, &Reg8::A),
            0xA8 => xor_r8(self, &Reg8::B),
            0xA9 => xor_r8(self, &Reg8::C),
            0xAA => xor_r8(self, &Reg8::D),
            0xAB => xor_r8(self, &Reg8::E),
            0xAC => xor_r8(self, &Reg8::H),
            0xAD => xor_r8(self, &Reg8::L),
            0xAE => xor_indirect_r16(self, &Reg16::HL),
            0xAF => xor_r8(self, &Reg8::A),

            // ORs & CPs
            0xB0 => or_r8(self, &Reg8::B),
            0xB1 => or_r8(self, &Reg8::C),
            0xB2 => or_r8(self, &Reg8::D),
            0xB3 => or_r8(self, &Reg8::E),
            0xB4 => or_r8(self, &Reg8::H),
            0xB5 => or_r8(self, &Reg8::L),
            0xB6 => or_indirect_r16(self, &Reg16::HL),
            0xB7 => or_r8(self, &Reg8::A),
            0xB8 => cp_r8(self, &Reg8::B),
            0xB9 => cp_r8(self, &Reg8::C),
            0xBA => cp_r8(self, &Reg8::D),
            0xBB => cp_r8(self, &Reg8::E),
            0xBC => cp_r8(self, &Reg8::H),
            0xBD => cp_r8(self, &Reg8::L),
            0xBE => cp_indirect_r16(self, &Reg16::HL),
            0xBF => cp_r8(self, &Reg8::A),

            0xC0 => ret_conditional(self, op),
            0xC1 => pop_r16(self, &Reg16::BC),
            0xC2 => jump_conditional_d16(self, op),
            0xC3 => jump_d16(self),
            0xC4 => call_conditional_d16(self, op),
            0xC5 => push_r16(self, &Reg16::BC),
            0xC6 => add_d8(self),
            0xC7 => reset(self, op),
            0xC8 => ret_conditional(self, op),
            0xC9 => ret(self),
            0xCA => jump_conditional_d16(self, op),
            0xCB => self.decode_cb_prefixed(),
            0xCC => call_conditional_d16(self, op),
            0xCD => call_d16(self),
            0xCE => adc_d8(self),
            0xCF => reset(self, op),

            0xD0 => ret_conditional(self, op),
            0xD1 => pop_r16(self, &Reg16::DE),
            0xD2 => jump_conditional_d16(self, op),
            0xD3 => undefined(self, op),
            0xD4 => call_conditional_d16(self, op),
            0xD5 => push_r16(self, &Reg16::DE),
            0xD6 => sub_d8(self),
            0xD7 => reset(self, op),
            0xD8 => ret_conditional(self, op),
            0xD9 => ret_interrupt(self),
            0xDA => jump_conditional_d16(self, op),
            0xDB => undefined(self, op),
            0xDC => call_conditional_d16(self, op),
            0xDD => undefined(self, op),
            0xDE => sbc_d8(self),
            0xDF => reset(self, op),

            0xE0 => load_high_mem_d8_reg_a(self),
            0xE1 => pop_r16(self, &Reg16::HL),
            0xE2 => load_high_mem_reg_c_reg_a(self),
            0xE3 => undefined(self, op),
            0xE4 => undefined(self, op),
            0xE5 => push_r16(self, &Reg16::HL),
            0xE6 => and_d8(self),
            0xE7 => reset(self, op),
            0xE8 => add_sp_d8(self),
            0xE9 => jump_r16(self, &Reg16::HL),
            0xEA => load_a16_reg_a(self),
            0xEB => undefined(self, op),
            0xEC => undefined(self, op),
            0xED => undefined(self, op),
            0xEE => xor_d8(self),
            0xEF => reset(self, op),

            0xF0 => load_reg_a_high_mem_d8(self),
            0xF1 => pop_r16(self, &Reg16::AF),
            0xF2 => load_reg_a_high_mem_reg_c(self),
            0xF3 => interrupts(self, false),
            0xF4 => undefined(self, op),
            0xF5 => push_r16(self, &Reg16::AF),
            0xF6 => or_d8(self),
            0xF7 => reset(self, op),
            0xF8 => load_reg_hl_reg_sp_d8(self),
            0xF9 => load_r16_r16(self, &Reg16::SP, &Reg16::HL),
            0xFA => load_reg_a_a16(self),
            0xFB => interrupts(self, true),
            0xFC => undefined(self, op),
            0xFD => undefined(self, op),
            0xFE => cp_d8(self),
            0xFF => reset(self, op),
        }
    }

    fn decode_cb_prefixed(&mut self) -> u8 {
        // Fetch
        let op = self.fetch8();

        // Decode & Execute
        let duration = match op {
            0x00 => rotate_left_r8(self, &Reg8::B, false, false),
            0x01 => rotate_left_r8(self, &Reg8::C, false, false),
            0x02 => rotate_left_r8(self, &Reg8::D, false, false),
            0x03 => rotate_left_r8(self, &Reg8::E, false, false),
            0x04 => rotate_left_r8(self, &Reg8::H, false, false),
            0x05 => rotate_left_r8(self, &Reg8::L, false, false),
            0x06 => rotate_left_indirect_hl(self, false, false),
            0x07 => rotate_left_r8(self, &Reg8::A, false, false),
            0x08 => rotate_right_r8(self, &Reg8::B, false, false),
            0x09 => rotate_right_r8(self, &Reg8::C, false, false),
            0x0A => rotate_right_r8(self, &Reg8::D, false, false),
            0x0B => rotate_right_r8(self, &Reg8::E, false, false),
            0x0C => rotate_right_r8(self, &Reg8::H, false, false),
            0x0D => rotate_right_r8(self, &Reg8::L, false, false),
            0x0E => rotate_right_indirect_hl(self, false, false),
            0x0F => rotate_right_r8(self, &Reg8::A, false, false),

            0x10 => rotate_left_r8(self, &Reg8::B, true, false),
            0x11 => rotate_left_r8(self, &Reg8::C, true, false),
            0x12 => rotate_left_r8(self, &Reg8::D, true, false),
            0x13 => rotate_left_r8(self, &Reg8::E, true, false),
            0x14 => rotate_left_r8(self, &Reg8::H, true, false),
            0x15 => rotate_left_r8(self, &Reg8::L, true, false),
            0x16 => rotate_left_indirect_hl(self, true, false),
            0x17 => rotate_left_r8(self, &Reg8::A, true, false),
            0x18 => rotate_right_r8(self, &Reg8::B, true, false),
            0x19 => rotate_right_r8(self, &Reg8::C, true, false),
            0x1A => rotate_right_r8(self, &Reg8::D, true, false),
            0x1B => rotate_right_r8(self, &Reg8::E, true, false),
            0x1C => rotate_right_r8(self, &Reg8::H, true, false),
            0x1D => rotate_right_r8(self, &Reg8::L, true, false),
            0x1E => rotate_right_indirect_hl(self, true, false),
            0x1F => rotate_right_r8(self, &Reg8::A, true, false),

            0x20 => shift_left_r8(self, &Reg8::B),
            0x21 => shift_left_r8(self, &Reg8::C),
            0x22 => shift_left_r8(self, &Reg8::D),
            0x23 => shift_left_r8(self, &Reg8::E),
            0x24 => shift_left_r8(self, &Reg8::H),
            0x25 => shift_left_r8(self, &Reg8::L),
            0x26 => shift_left_indirect_hl(self),
            0x27 => shift_left_r8(self, &Reg8::A),
            0x28 => shift_right_r8(self, &Reg8::B, false),
            0x29 => shift_right_r8(self, &Reg8::C, false),
            0x2A => shift_right_r8(self, &Reg8::D, false),
            0x2B => shift_right_r8(self, &Reg8::E, false),
            0x2C => shift_right_r8(self, &Reg8::H, false),
            0x2D => shift_right_r8(self, &Reg8::L, false),
            0x2E => shift_right_indirect_hl(self, false),
            0x2F => shift_right_r8(self, &Reg8::A, false),

            0x30 => swap_r8(self, &Reg8::B),
            0x31 => swap_r8(self, &Reg8::C),
            0x32 => swap_r8(self, &Reg8::D),
            0x33 => swap_r8(self, &Reg8::E),
            0x34 => swap_r8(self, &Reg8::H),
            0x35 => swap_r8(self, &Reg8::L),
            0x36 => swap_indirect_hl(self),
            0x37 => swap_r8(self, &Reg8::A),
            0x38 => shift_right_r8(self, &Reg8::B, true),
            0x39 => shift_right_r8(self, &Reg8::C, true),
            0x3A => shift_right_r8(self, &Reg8::D, true),
            0x3B => shift_right_r8(self, &Reg8::E, true),
            0x3C => shift_right_r8(self, &Reg8::H, true),
            0x3D => shift_right_r8(self, &Reg8::L, true),
            0x3E => shift_right_indirect_hl(self, true),
            0x3F => shift_right_r8(self, &Reg8::A, true),

            0x40..=0x7F => bit_test(self, op),

            0x80..=0xBF => bit_assign(self, op, false),
            0xC0..=0xFF => bit_assign(self, op, true),
        };

        duration + 4
    }
}
