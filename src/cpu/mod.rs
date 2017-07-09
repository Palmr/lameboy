pub mod registers;
use cpu::registers::*;

pub mod instructions;
use cpu::instructions::*;

use self::super::cart::Cart;

use self::super::mmu::MMU;

pub struct CPU<'c> {
    pub registers: Registers,
    pub mmu: MMU<'c>
}

impl<'c> CPU<'c> {
    pub fn new(cart: &Cart) -> CPU {
        CPU {
            registers: Registers::new(),
            mmu: MMU::new(cart),
        }
    }

    pub fn post_boot_reset(&mut self) {
        self.registers.post_boot_reset();
    }

    pub fn cycle(&mut self) {
        // Fetch
        let op = self.mmu.read8(self.registers.pc);
        println!("Opcode[{:04X}] = {:02X}", self.registers.pc, op);

        // Move PC
        self.registers.pc += 1;

        // Decode & Execute
        let duration = match op {
            0x00 => {println!("NOP"); nop(self)},
            0x01 => {println!("LD BC, d16"); load_r16_d16(self, &Reg16::BC)},
            0x02 => {println!("LD (BC), A"); load_indirect_r16_r8(self, &Reg16::BC, &Reg8::A)},
            0x03 => {println!("INC BC"); inc_r16(self, &Reg16::BC)},
            0x04 => {println!("INC B"); inc_r8(self, &Reg8::B)},
            0x05 => {println!("DEC B"); dec_r8(self, &Reg8::B)},
            0x06 => {println!("LD B, d8"); load_r8_d8(self, &Reg8::B)},
            0x07 => {println!("RLCA"); rotate_left_r8(self, &Reg8::A, false, true)},
            0x08 => {println!("LD (a16), SP"); load_indirect_a16_r16(self, &Reg16::SP)},
            0x09 => {println!("ADD HL, BC"); add_hl_r16(self, &Reg16::BC)},
            0x0A => {println!("LD A, (BC)"); load_r8_indirect_r16(self, &Reg8::A, &Reg16::BC)},
            0x0B => {println!("DEC BC"); dec_r16(self, &Reg16::BC)},
            0x0C => {println!("INC C"); inc_r8(self, &Reg8::C)},
            0x0D => {println!("DEC C"); dec_r8(self, &Reg8::C)},
            0x0E => {println!("LD C, d8"); load_r8_d8(self, &Reg8::C)},
            0x0F => {println!("RRCA"); rotate_right_r8(self, &Reg8::A, false, true)},

            0x10 => {println!("STOP 0"); 4},
            0x11 => {println!("LD DE, d16"); load_r16_d16(self, &Reg16::DE)},
            0x12 => {println!("LD (DE), A"); load_indirect_r16_r8(self, &Reg16::DE, &Reg8::A)},
            0x13 => {println!("INC DE"); inc_r16(self, &Reg16::DE)},
            0x14 => {println!("INC D"); inc_r8(self, &Reg8::D)},
            0x15 => {println!("DEC D"); dec_r8(self, &Reg8::D)},
            0x16 => {println!("LD D, d8"); load_r8_d8(self, &Reg8::D)},
            0x17 => {println!("RLA");  rotate_left_r8(self, &Reg8::A, true, true)},
            0x18 => {println!("JR r8"); jump_relative_d8(self)},
            0x19 => {println!("ADD HL, DE"); add_hl_r16(self, &Reg16::DE)},
            0x1A => {println!("LD A, (DE)"); load_r8_indirect_r16(self, &Reg8::A, &Reg16::DE)},
            0x1B => {println!("DEC DE"); dec_r16(self, &Reg16::DE)},
            0x1C => {println!("INC E"); inc_r8(self, &Reg8::E)},
            0x1D => {println!("DEC E"); dec_r8(self, &Reg8::E)},
            0x1E => {println!("LD E, d8"); load_r8_d8(self, &Reg8::E)},
            0x1F => {println!("RRA"); rotate_right_r8(self, &Reg8::A, true, true)},

            0x20 => {println!("JR NZ, r8"); jump_relative_conditional_d8(self, op)},
            0x21 => {println!("LD HL, d16"); load_r16_d16(self, &Reg16::HL)},
            0x22 => {println!("LD (HL+), A"); load_indirect_r16_increment_r8(self, &Reg16::HL, &Reg8::A)},
            0x23 => {println!("INC HL"); inc_r16(self, &Reg16::HL)},
            0x24 => {println!("INC H"); inc_r8(self, &Reg8::H)},
            0x25 => {println!("DEC H"); dec_r8(self, &Reg8::H)},
            0x26 => {println!("LD H ,d8"); load_r8_d8(self, &Reg8::H)},
            0x27 => {println!("DAA"); 4},
            0x28 => {println!("JR Z, r8"); jump_relative_conditional_d8(self, op)},
            0x29 => {println!("ADD HL, HL"); add_hl_r16(self, &Reg16::HL)},
            0x2A => {println!("LD A, (HL+)"); load_r8_indirect_r16_increment(self, &Reg8::A, &Reg16::HL)},
            0x2B => {println!("DEC HL"); dec_r16(self, &Reg16::HL)},
            0x2C => {println!("INC L"); inc_r8(self, &Reg8::L)},
            0x2D => {println!("DEC L"); dec_r8(self, &Reg8::L)},
            0x2E => {println!("LD L, d8"); load_r8_d8(self, &Reg8::L)},
            0x2F => {println!("CPL"); 4},

            0x30 => {println!("JR NC, r8"); jump_relative_conditional_d8(self, op)},
            0x31 => {println!("LD SP, d16"); load_r16_d16(self, &Reg16::SP)},
            0x32 => {println!("LD (HL-), A"); load_indirect_r16_decrement_r8(self, &Reg16::HL, &Reg8::A)},
            0x33 => {println!("INC SP"); inc_r16(self, &Reg16::SP)},
            0x34 => {println!("INC (HL)"); inc_indirect_r16(self, &Reg16::HL)},
            0x35 => {println!("DEC (HL)"); dec_indirect_r16(self, &Reg16::HL)},
            0x36 => {println!("LD (HL), d8"); load_indirect_r16_d8(self, &Reg16::HL)},
            0x37 => {println!("SCF"); 4},
            0x38 => {println!("JR C, r8"); jump_relative_conditional_d8(self, op)},
            0x39 => {println!("ADD HL, SP"); add_hl_r16(self, &Reg16::SP)},
            0x3A => {println!("LD A, (HL-)"); load_r8_indirect_r16_decrement(self, &Reg8::A, &Reg16::HL)},
            0x3B => {println!("DEC SP"); dec_r16(self, &Reg16::SP)},
            0x3C => {println!("INC A"); inc_r8(self, &Reg8::A)},
            0x3D => {println!("DEC A"); dec_r8(self, &Reg8::A)},
            0x3E => {println!("LD A, d8"); load_r8_d8(self, &Reg8::A)},
            0x3F => {println!("CCF"); 4},

            // Loads
            0x40 => {println!("LD B, B"); load_r8_r8(self, &Reg8::B, &Reg8::B)},
            0x41 => {println!("LD B, C"); load_r8_r8(self, &Reg8::B, &Reg8::C)},
            0x42 => {println!("LD B, D"); load_r8_r8(self, &Reg8::B, &Reg8::D)},
            0x43 => {println!("LD B, E"); load_r8_r8(self, &Reg8::B, &Reg8::E)},
            0x44 => {println!("LD B, H"); load_r8_r8(self, &Reg8::B, &Reg8::H)},
            0x45 => {println!("LD B, L"); load_r8_r8(self, &Reg8::B, &Reg8::L)},
            0x46 => {println!("LD B, (HL)"); load_r8_indirect_r16(self, &Reg8::B, &Reg16::HL)},
            0x47 => {println!("LD B, A"); load_r8_r8(self, &Reg8::B, &Reg8::A)},
            0x48 => {println!("LD C, B"); load_r8_r8(self, &Reg8::C, &Reg8::B)},
            0x49 => {println!("LD C, C"); load_r8_r8(self, &Reg8::C, &Reg8::C)},
            0x4A => {println!("LD C, D"); load_r8_r8(self, &Reg8::C, &Reg8::D)},
            0x4B => {println!("LD C, E"); load_r8_r8(self, &Reg8::C, &Reg8::E)},
            0x4C => {println!("LD C, H"); load_r8_r8(self, &Reg8::C, &Reg8::H)},
            0x4D => {println!("LD C, L"); load_r8_r8(self, &Reg8::C, &Reg8::L)},
            0x4E => {println!("LD C, (HL)"); load_r8_indirect_r16(self, &Reg8::C, &Reg16::HL)},
            0x4F => {println!("LD C, A"); load_r8_r8(self, &Reg8::C, &Reg8::A)},

            0x50 => {println!("LD D, B"); load_r8_r8(self, &Reg8::D, &Reg8::B)},
            0x51 => {println!("LD D, C"); load_r8_r8(self, &Reg8::D, &Reg8::C)},
            0x52 => {println!("LD D, D"); load_r8_r8(self, &Reg8::D, &Reg8::D)},
            0x53 => {println!("LD D, E"); load_r8_r8(self, &Reg8::D, &Reg8::E)},
            0x54 => {println!("LD D, H"); load_r8_r8(self, &Reg8::D, &Reg8::H)},
            0x55 => {println!("LD D, L"); load_r8_r8(self, &Reg8::D, &Reg8::L)},
            0x56 => {println!("LD D, (HL)"); load_r8_indirect_r16(self, &Reg8::D, &Reg16::HL)},
            0x57 => {println!("LD D, A"); load_r8_r8(self, &Reg8::D, &Reg8::A)},
            0x58 => {println!("LD E, B"); load_r8_r8(self, &Reg8::E, &Reg8::B)},
            0x59 => {println!("LD E, C"); load_r8_r8(self, &Reg8::E, &Reg8::C)},
            0x5A => {println!("LD E, D"); load_r8_r8(self, &Reg8::E, &Reg8::D)},
            0x5B => {println!("LD E, E"); load_r8_r8(self, &Reg8::E, &Reg8::E)},
            0x5C => {println!("LD E, H"); load_r8_r8(self, &Reg8::E, &Reg8::H)},
            0x5D => {println!("LD E, L"); load_r8_r8(self, &Reg8::E, &Reg8::L)},
            0x5E => {println!("LD E, (HL)"); load_r8_indirect_r16(self, &Reg8::E, &Reg16::HL)},
            0x5F => {println!("LD E, A"); load_r8_r8(self, &Reg8::E, &Reg8::A)},

            0x60 => {println!("LD H, B"); load_r8_r8(self, &Reg8::H, &Reg8::B)},
            0x61 => {println!("LD H, C"); load_r8_r8(self, &Reg8::H, &Reg8::C)},
            0x62 => {println!("LD H, D"); load_r8_r8(self, &Reg8::H, &Reg8::D)},
            0x63 => {println!("LD H, E"); load_r8_r8(self, &Reg8::H, &Reg8::E)},
            0x64 => {println!("LD H, H"); load_r8_r8(self, &Reg8::H, &Reg8::H)},
            0x65 => {println!("LD H, L"); load_r8_r8(self, &Reg8::H, &Reg8::L)},
            0x66 => {println!("LD H, (HL)"); load_r8_indirect_r16(self, &Reg8::H, &Reg16::HL)},
            0x67 => {println!("LD H, A"); load_r8_r8(self, &Reg8::H, &Reg8::A)},
            0x68 => {println!("LD L, B"); load_r8_r8(self, &Reg8::L, &Reg8::B)},
            0x69 => {println!("LD L, C"); load_r8_r8(self, &Reg8::L, &Reg8::C)},
            0x6A => {println!("LD L, D"); load_r8_r8(self, &Reg8::L, &Reg8::D)},
            0x6B => {println!("LD L, E"); load_r8_r8(self, &Reg8::L, &Reg8::E)},
            0x6C => {println!("LD L, H"); load_r8_r8(self, &Reg8::L, &Reg8::H)},
            0x6D => {println!("LD L, L"); load_r8_r8(self, &Reg8::L, &Reg8::L)},
            0x6E => {println!("LD L, (HL)"); load_r8_indirect_r16(self, &Reg8::L, &Reg16::HL)},
            0x6F => {println!("LD L, A"); load_r8_r8(self, &Reg8::L, &Reg8::A)},

            0x70 => {println!("LD (HL), B"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::B)},
            0x71 => {println!("LD (HL), C"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::C)},
            0x72 => {println!("LD (HL), D"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::D)},
            0x73 => {println!("LD (HL), E"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::E)},
            0x74 => {println!("LD (HL), H"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::H)},
            0x75 => {println!("LD (HL), L"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::L)},
            0x76 => {println!("HALT"); 4},
            0x77 => {println!("LD (HL), A"); load_indirect_r16_r8(self, &Reg16::HL, &Reg8::A)},
            0x78 => {println!("LD A, B"); load_r8_r8(self, &Reg8::A, &Reg8::A)},
            0x79 => {println!("LD A, C"); load_r8_r8(self, &Reg8::A, &Reg8::B)},
            0x7A => {println!("LD A, D"); load_r8_r8(self, &Reg8::A, &Reg8::C)},
            0x7B => {println!("LD A, E"); load_r8_r8(self, &Reg8::A, &Reg8::D)},
            0x7C => {println!("LD A, H"); load_r8_r8(self, &Reg8::A, &Reg8::E)},
            0x7D => {println!("LD A, L"); load_r8_r8(self, &Reg8::A, &Reg8::H)},
            0x7E => {println!("LD A, (HL)"); load_r8_indirect_r16(self, &Reg8::A, &Reg16::HL)},
            0x7F => {println!("LD A, A"); load_r8_r8(self, &Reg8::A, &Reg8::A)},

            // ADDs
            0x80 => {println!("ADD A, B"); add_r8(self, &Reg8::B)},
            0x81 => {println!("ADD A, C"); add_r8(self, &Reg8::C)},
            0x82 => {println!("ADD A, D"); add_r8(self, &Reg8::D)},
            0x83 => {println!("ADD A, E"); add_r8(self, &Reg8::E)},
            0x84 => {println!("ADD A, H"); add_r8(self, &Reg8::H)},
            0x85 => {println!("ADD A, L"); add_r8(self, &Reg8::L)},
            0x86 => {println!("ADD A, (HL)"); add_indirect_r16(self, &Reg16::HL)},
            0x87 => {println!("ADD A, A"); add_r8(self, &Reg8::A)},
            0x88 => {println!("ADC A, B"); adc_r8(self, &Reg8::B)},
            0x89 => {println!("ADC A, C"); adc_r8(self, &Reg8::C)},
            0x8A => {println!("ADC A, D"); adc_r8(self, &Reg8::D)},
            0x8B => {println!("ADC A, E"); adc_r8(self, &Reg8::E)},
            0x8C => {println!("ADC A, H"); adc_r8(self, &Reg8::H)},
            0x8D => {println!("ADC A, L"); adc_r8(self, &Reg8::L)},
            0x8E => {println!("ADC A, (HL)"); adc_indirect_r16(self, &Reg16::HL)},
            0x8F => {println!("ADC A, A"); adc_r8(self, &Reg8::A)},

            // SUBs
            0x90 => {println!("SUB B"); sub_r8(self, &Reg8::B)},
            0x91 => {println!("SUB C"); sub_r8(self, &Reg8::C)},
            0x92 => {println!("SUB D"); sub_r8(self, &Reg8::D)},
            0x93 => {println!("SUB E"); sub_r8(self, &Reg8::E)},
            0x94 => {println!("SUB H"); sub_r8(self, &Reg8::H)},
            0x95 => {println!("SUB L"); sub_r8(self, &Reg8::L)},
            0x96 => {println!("SUB (HL)"); sub_indirect_r16(self, &Reg16::HL)},
            0x97 => {println!("SUB A"); sub_r8(self, &Reg8::A)},
            0x98 => {println!("SBC A, B"); sbc_r8(self, &Reg8::B)},
            0x99 => {println!("SBC A, C"); sbc_r8(self, &Reg8::C)},
            0x9A => {println!("SBC A, D"); sbc_r8(self, &Reg8::D)},
            0x9B => {println!("SBC A, E"); sbc_r8(self, &Reg8::E)},
            0x9C => {println!("SBC A, H"); sbc_r8(self, &Reg8::H)},
            0x9D => {println!("SBC A, L"); sbc_r8(self, &Reg8::L)},
            0x9E => {println!("SBC A, (HL)"); sbc_indirect_r16(self, &Reg16::HL)},
            0x9F => {println!("SBC A, A"); sbc_r8(self, &Reg8::A)},

            // ANDs & XORs
            0xA0 => {println!("AND B"); and_r8(self, &Reg8::B)},
            0xA1 => {println!("AND C"); and_r8(self, &Reg8::C)},
            0xA2 => {println!("AND D"); and_r8(self, &Reg8::D)},
            0xA3 => {println!("AND E"); and_r8(self, &Reg8::E)},
            0xA4 => {println!("AND H"); and_r8(self, &Reg8::H)},
            0xA5 => {println!("AND L"); and_r8(self, &Reg8::L)},
            0xA6 => {println!("AND (HL)"); and_indirect_r16(self, &Reg16::HL)},
            0xA7 => {println!("AND A"); and_r8(self, &Reg8::A)},
            0xA8 => {println!("XOR B"); xor_r8(self, &Reg8::B)},
            0xA9 => {println!("XOR C"); xor_r8(self, &Reg8::C)},
            0xAA => {println!("XOR D"); xor_r8(self, &Reg8::D)},
            0xAB => {println!("XOR E"); xor_r8(self, &Reg8::E)},
            0xAC => {println!("XOR H"); xor_r8(self, &Reg8::H)},
            0xAD => {println!("XOR L"); xor_r8(self, &Reg8::L)},
            0xAE => {println!("XOR (HL)"); xor_indirect_r16(self, &Reg16::HL)},
            0xAF => {println!("XOR A"); xor_r8(self, &Reg8::A)},

            // ORs & CPs
            0xB0 => {println!("OR B"); or_r8(self, &Reg8::B)},
            0xB1 => {println!("OR C"); or_r8(self, &Reg8::C)},
            0xB2 => {println!("OR D"); or_r8(self, &Reg8::D)},
            0xB3 => {println!("OR E"); or_r8(self, &Reg8::E)},
            0xB4 => {println!("OR H"); or_r8(self, &Reg8::H)},
            0xB5 => {println!("OR L"); or_r8(self, &Reg8::L)},
            0xB6 => {println!("OR (HL)"); or_indirect_r16(self, &Reg16::HL)},
            0xB7 => {println!("OR A"); or_r8(self, &Reg8::A)},
            0xB8 => {println!("CP B"); cp_r8(self, &Reg8::B)},
            0xB9 => {println!("CP C"); cp_r8(self, &Reg8::C)},
            0xBA => {println!("CP D"); cp_r8(self, &Reg8::D)},
            0xBB => {println!("CP E"); cp_r8(self, &Reg8::E)},
            0xBC => {println!("CP H"); cp_r8(self, &Reg8::H)},
            0xBD => {println!("CP L"); cp_r8(self, &Reg8::L)},
            0xBE => {println!("CP (HL)"); cp_indirect_r16(self, &Reg16::HL)},
            0xBF => {println!("CP A"); cp_r8(self, &Reg8::A)},

            0xC0 => {println!("RET NZ"); 20/*/8*/},
            0xC1 => {println!("POP BC"); 12},
            0xC2 => {println!("JP NZ, a16"); jump_conditional_d16(self, op)},
            0xC3 => {println!("JP a16"); jump_d16(self)},
            0xC4 => {println!("CALL NZ, a16"); 24/*/12*/},
            0xC5 => {println!("PUSH BC"); 16},
            0xC6 => {println!("ADD A, d8"); add_d8(self)},
            0xC7 => {println!("RST 00H"); 16},
            0xC8 => {println!("RET Z"); 20/*/8*/},
            0xC9 => {println!("RET"); 16},
            0xCA => {println!("JP Z, a16"); jump_conditional_d16(self, op)},
            0xCB => {println!("PREFIX CB"); self.cb_prefix()},
            0xCC => {println!("CALL Z, a16"); 24/*/12*/},
            0xCD => {println!("CALL a16"); 24},
            0xCE => {println!("ADC A, d8"); adc_d8(self)},
            0xCF => {println!("RST 08H"); 16},

            0xD0 => {println!("RET NC"); 20/*/8*/},
            0xD1 => {println!("POP DE"); 12},
            0xD2 => {println!("JP NC, a16"); jump_conditional_d16(self, op)},
            0xD3 => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xD4 => {println!("CALL NC, a16"); 24/*/12*/},
            0xD5 => {println!("PUSH DE"); 16},
            0xD6 => {println!("SUB d8"); sub_d8(self)},
            0xD7 => {println!("RST 10H"); 16},
            0xD8 => {println!("RET C"); 20/*/8*/},
            0xD9 => {println!("RETI"); 16},
            0xDA => {println!("JP C, a16"); jump_conditional_d16(self, op)},
            0xDB => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xDC => {println!("CALL C, a16"); 24/*/12*/},
            0xDD => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xDE => {println!("SBC A, d8"); sbc_d8(self)},
            0xDF => {println!("RST 18H"); 16},

            0xE0 => {println!("LDH (a8) ,A"); 12},
            0xE1 => {println!("POP HL"); 12},
            0xE2 => {println!("LD (C), A"); 8},
            0xE3 => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xE4 => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xE5 => {println!("PUSH HL"); 16},
            0xE6 => {println!("AND d8"); and_d8(self)},
            0xE7 => {println!("RST 20H"); 16},
            0xE8 => {println!("ADD SP, d8"); add_sp_d8(self, &Reg16::SP)},
            0xE9 => {println!("JP (HL)"); jump_r16(self, &Reg16::HL)},
            0xEA => {println!("LD (a16), A"); 16},
            0xEB => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xEC => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xED => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xEE => {println!("XOR d8"); xor_d8(self)},
            0xEF => {println!("RST 28H"); 16},

            0xF0 => {println!("LDH A, (a8)"); 12},
            0xF1 => {println!("POP AF"); 12},
            0xF2 => {println!("LD A, (C)"); 8},
            0xF3 => {println!("DI"); 4},
            0xF4 => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xF5 => {println!("PUSH AF"); 16},
            0xF6 => {println!("OR d8"); or_d8(self)},
            0xF7 => {println!("RST 30H"); 16},
            0xF8 => {println!("LD HL, SP+r8"); 12},
            0xF9 => {println!("LD SP, HL"); load_r16_r16(self, &Reg16::SP, &Reg16::HL)},
            0xFA => {println!("LD A, (a16)"); 16},
            0xFB => {println!("EI"); 4},
            0xFC => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xFD => {println!("!!!UNDEFINED OPCODE!!!"); 255},  // TODO - Handle Undefined
            0xFE => {println!("CP d8"); cp_d8(self)},
            0xFF => {println!("RST 38H"); 16},

            _ => {println!("Unhandled Op: {:02X}", op); 0}
        };
        println!("Took {} cycles", duration);
    }

    fn cb_prefix(&mut self) -> u8 {
        // Fetch
        let op = self.mmu.read8(self.registers.pc);
        println!("CB Opcode[{:04X}] = {:02X}", self.registers.pc, op);

        // Move PC
        self.registers.pc += 1;

        // Derive the register the opcode will be using
        let reg8_vec = vec![Reg8::B, Reg8::C, Reg8::D, Reg8::E, Reg8::H, Reg8::L, Reg8::A];
        let register_key = op & 0b00000111;
        let register = &reg8_vec[register_key as usize];

        // Derive the bit the opcode will affect
        let bit = (op >> 3) & 0b00111000;

        // Decode & Execute
        match op {
            0x00...0x05 | 0x07 => {println!("RLC {:?}", register); rotate_left_r8(self, &register, false, false)},
            0x06 => {println!("RLC (HL)"); rotate_left_indirect_hl(self, false, false)},
            0x08...0x0D | 0x0F => {println!("RRC {:?}", register); rotate_right_r8(self, &register, false, false)},
            0x0E => {println!("RRC (HL)"); rotate_right_indirect_hl(self, false, false)},
            0x10...0x15 | 0x17 => {println!("RLC {:?}", register); rotate_left_r8(self, &register, true, false)},
            0x16 => {println!("RLC (HL)"); rotate_left_indirect_hl(self, true, false)},
            0x18...0x1D | 0x1F => {println!("RRC {:?}", register); rotate_right_r8(self, &register, true, false)},
            0x1E => {println!("RRC (HL)"); rotate_right_indirect_hl(self, true, false)},

            _ => {println!("Unhandled CB Op: {:02X}", op); 0}
        }
    }
}
