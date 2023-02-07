bitflags! {
    pub struct Flags: u8 {
        const ZERO          = 0b_1000_0000;
        const SUBTRACT      = 0b_0100_0000;
        const HALF_CARRY    = 0b_0010_0000;
        const CARRY         = 0b_0001_0000;
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Register {
    Reg8(Reg8),
    Reg16(Reg16),
}

#[derive(Debug, Eq, PartialEq)]
pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

/// Registers for the CPU core
pub struct Registers {
    pub a: u8,
    pub f: Flags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            f: Flags::empty(),
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
            pc: 0,
            sp: 0,
        }
    }

    pub fn reset(&mut self) {
        self.a = 0x01;
        self.f = Flags::ZERO | Flags::HALF_CARRY | Flags::CARRY;
        self.b = 0x00;
        self.c = 0x13;
        self.d = 0x00;
        self.e = 0xD8;
        self.h = 0x01;
        self.l = 0x4D;
        self.pc = 0x0100;
        self.sp = 0xFFFE;
    }

    pub fn read8(&self, r8: &Reg8) -> u8 {
        use self::Reg8::*;
        match r8 {
            A => self.a,
            B => self.b,
            C => self.c,
            D => self.d,
            E => self.e,
            H => self.h,
            L => self.l,
        }
    }

    pub fn write8(&mut self, r8: &Reg8, value: u8) {
        use self::Reg8::*;
        match r8 {
            A => self.a = value,
            B => self.b = value,
            C => self.c = value,
            D => self.d = value,
            E => self.e = value,
            H => self.h = value,
            L => self.l = value,
        }
    }

    pub fn read16(&self, r16: &Reg16) -> u16 {
        use self::Reg16::*;
        match r16 {
            AF => (u16::from(self.a) << 8) | (u16::from(self.f.bits())),
            BC => (u16::from(self.b) << 8) | (u16::from(self.c)),
            DE => (u16::from(self.d) << 8) | (u16::from(self.e)),
            HL => (u16::from(self.h) << 8) | (u16::from(self.l)),
            SP => self.sp,
        }
    }

    pub fn write16(&mut self, r16: &Reg16, value: u16) {
        use self::Reg16::*;
        match r16 {
            AF => {
                self.a = (value >> 8) as u8;
                self.f = Flags::from_bits_truncate(value as u8)
            }
            BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8
            }
            DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8
            }
            HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8
            }
            SP => self.sp = value,
        }
    }
}
