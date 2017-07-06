pub mod registers;
use cpu::registers::*;

pub mod instructions;
use cpu::instructions::*;

use self::super::cart::Cart;

pub struct CPU<'c> {
    pub registers: Registers,
    cart: &'c Cart
}

impl<'c> CPU<'c> {
    pub fn new(cart: &Cart) -> CPU {
        CPU {
            registers: Registers::new(),
            cart: cart
        }
    }

    pub fn post_boot_reset(&mut self) {
        self.registers.post_boot_reset();
    }

    pub fn cycle(&mut self) {
        // Fetch
        let op = self.cart.read(self.registers.pc);
        println!("Opcode[{:04X}] = {:02X}", self.registers.pc, op);

        // Move PC
        self.registers.pc += 1;

        // Decode & Execute
        let duration = match op {
            0x00 => {println!("NOP"); nop(self)},
            0x01 => {println!("LD BC, d16"); 12},
            0x02 => {println!("LD (BC), A"); 8},
            0x03 => {println!("INC BC"); inc_r16(self, &Reg16::BC)},
            0x04 => {println!("INC B"); inc_r8(self, &Reg8::B)},
            0x05 => {println!("DEC B"); dec_r8(self, &Reg8::B)},
            0x06 => {println!("LD B, d8"); 8},
            0x07 => {println!("RLCA"); 4},
            0x08 => {println!("LD (a16), SP"); 20},
            0x09 => {println!("ADD HL, BC"); 8},
            0x0A => {println!("LD A, (BC)"); 8},
            0x0B => {println!("DEC BC"); 8},
            0x0C => {println!("INC C"); inc_r8(self, &Reg8::C)},
            0x0D => {println!("DEC C"); dec_r8(self, &Reg8::C)},
            0x0E => {println!("LD C, d8"); 8},
            0x0F => {println!("RRCA"); 4},

            0x10 => {println!("STOP 0"); 4},
            0x11 => {println!("LD DE, d16"); 12},
            0x12 => {println!("LD (DE), A"); 8},
            0x13 => {println!("INC DE"); inc_r16(self, &Reg16::DE)},
            0x14 => {println!("INC D"); inc_r8(self, &Reg8::D)},
            0x15 => {println!("DEC D"); dec_r8(self, &Reg8::D)},
            0x16 => {println!("LD D, d8"); 8},
            0x17 => {println!("RLA"); 4},
            0x18 => {println!("JR r8"); 12},
            0x19 => {println!("ADD HL, DE"); 8},
            0x1A => {println!("LD A, (DE)"); 8},
            0x1B => {println!("DEC DE"); 8},
            0x1C => {println!("INC E"); inc_r8(self, &Reg8::E)},
            0x1D => {println!("DEC E"); dec_r8(self, &Reg8::E)},
            0x1E => {println!("LD E, d8"); 8},
            0x1F => {println!("RRA"); 4},

            0x20 => {println!("JR NZ, r8"); 12/*/8*/},
            0x21 => {println!("LD HL, d16"); 12},
            0x22 => {println!("LD (HL+), A"); 8},
            0x23 => {println!("INC HL"); inc_r16(self, &Reg16::HL)},
            0x24 => {println!("INC H"); inc_r8(self, &Reg8::H)},
            0x25 => {println!("DEC H"); dec_r8(self, &Reg8::H)},
            0x26 => {println!("LD H ,d8"); 8},
            0x27 => {println!("DAA"); 4},
            0x28 => {println!("JR Z, r8"); 12/*/8*/},
            0x29 => {println!("ADD HL, HL"); 8},
            0x2A => {println!("LD A, (HL+)"); 8},
            0x2B => {println!("DEC HL"); 8},
            0x2C => {println!("INC L"); inc_r8(self, &Reg8::L)},
            0x2D => {println!("DEC L"); dec_r8(self, &Reg8::L)},
            0x2E => {println!("LD L, d8"); 8},
            0x2F => {println!("CPL"); 4},

            0x30 => {println!("JR NC, r8"); 12/*/8*/},
            0x31 => {println!("LD SP, d16"); 12},
            0x32 => {println!("LD (HL-), A"); 8},
            0x33 => {println!("INC SP"); inc_r16(self, &Reg16::SP)},
            0x34 => {println!("INC (HL)"); 12},
            0x35 => {println!("DEC (HL)"); 12},
            0x36 => {println!("LD (HL), d8"); 12},
            0x37 => {println!("SCF"); 4},
            0x38 => {println!("JR C, r8"); 12/*/8*/},
            0x39 => {println!("ADD HL, SP"); 8},
            0x3A => {println!("LD A, (HL-)"); 8},
            0x3B => {println!("DEC SP"); 8},
            0x3C => {println!("INC A"); inc_r8(self, &Reg8::A)},
            0x3D => {println!("DEC A"); dec_r8(self, &Reg8::A)},
            0x3E => {println!("LD A, d8"); 8},
            0x3F => {println!("CCF"); 4},

            0x40 => {println!("TODO"); 1},
            0x41 => {println!("TODO"); 1},
            0x42 => {println!("TODO"); 1},
            0x43 => {println!("TODO"); 1},
            0x44 => {println!("TODO"); 1},
            0x45 => {println!("TODO"); 1},
            0x46 => {println!("TODO"); 1},
            0x47 => {println!("TODO"); 1},
            0x48 => {println!("TODO"); 1},
            0x49 => {println!("TODO"); 1},
            0x4A => {println!("TODO"); 1},
            0x4B => {println!("TODO"); 1},
            0x4C => {println!("TODO"); 1},
            0x4D => {println!("TODO"); 1},
            0x4E => {println!("TODO"); 1},
            0x4F => {println!("TODO"); 1},

            0x50 => {println!("TODO"); 1},
            0x51 => {println!("TODO"); 1},
            0x52 => {println!("TODO"); 1},
            0x53 => {println!("TODO"); 1},
            0x54 => {println!("TODO"); 1},
            0x55 => {println!("TODO"); 1},
            0x56 => {println!("TODO"); 1},
            0x57 => {println!("TODO"); 1},
            0x58 => {println!("TODO"); 1},
            0x59 => {println!("TODO"); 1},
            0x5A => {println!("TODO"); 1},
            0x5B => {println!("TODO"); 1},
            0x5C => {println!("TODO"); 1},
            0x5D => {println!("TODO"); 1},
            0x5E => {println!("TODO"); 1},
            0x5F => {println!("TODO"); 1},

            0x60 => {println!("TODO"); 1},
            0x61 => {println!("TODO"); 1},
            0x62 => {println!("TODO"); 1},
            0x63 => {println!("TODO"); 1},
            0x64 => {println!("TODO"); 1},
            0x65 => {println!("TODO"); 1},
            0x66 => {println!("TODO"); 1},
            0x67 => {println!("TODO"); 1},
            0x68 => {println!("TODO"); 1},
            0x69 => {println!("TODO"); 1},
            0x6A => {println!("TODO"); 1},
            0x6B => {println!("TODO"); 1},
            0x6C => {println!("TODO"); 1},
            0x6D => {println!("TODO"); 1},
            0x6E => {println!("TODO"); 1},
            0x6F => {println!("TODO"); 1},

            0x70 => {println!("TODO"); 1},
            0x71 => {println!("TODO"); 1},
            0x72 => {println!("TODO"); 1},
            0x73 => {println!("TODO"); 1},
            0x74 => {println!("TODO"); 1},
            0x75 => {println!("TODO"); 1},
            0x76 => {println!("TODO"); 1},
            0x77 => {println!("TODO"); 1},
            0x78 => {println!("TODO"); 1},
            0x79 => {println!("TODO"); 1},
            0x7A => {println!("TODO"); 1},
            0x7B => {println!("TODO"); 1},
            0x7C => {println!("TODO"); 1},
            0x7D => {println!("TODO"); 1},
            0x7E => {println!("TODO"); 1},
            0x7F => {println!("TODO"); 1},

            0x80 => {println!("TODO"); 1},
            0x81 => {println!("TODO"); 1},
            0x82 => {println!("TODO"); 1},
            0x83 => {println!("TODO"); 1},
            0x84 => {println!("TODO"); 1},
            0x85 => {println!("TODO"); 1},
            0x86 => {println!("TODO"); 1},
            0x87 => {println!("TODO"); 1},
            0x88 => {println!("TODO"); 1},
            0x89 => {println!("TODO"); 1},
            0x8A => {println!("TODO"); 1},
            0x8B => {println!("TODO"); 1},
            0x8C => {println!("TODO"); 1},
            0x8D => {println!("TODO"); 1},
            0x8E => {println!("TODO"); 1},
            0x8F => {println!("TODO"); 1},

            _ => {println!("Unhandled Op: {:02X}", op); 0}
        };
        println!("Took {} cycles", duration);
    }
}
