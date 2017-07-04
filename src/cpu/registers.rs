bitflags! {
    pub struct Flags: u8 {
        const ZERO          = 0b_1000_0000;
        const SUBTRACT      = 0b_0100_0000;
        const HALF_CARRY    = 0b_0010_0000;
        const CARRY         = 0b_0001_0000;
    }
}

pub enum Reg8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L
}

pub enum Reg16 {
    AF,
    BC,
    DE,
    HL,
    SP
}

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
            sp: 0
        }
    }

    pub fn read16(&self, reg: &Reg16) -> u16 {
        use self::Reg16::*;
        match reg {
            &AF => ((self.a as u16) << 8) | (self.f.bits() as u16),
            &BC => ((self.b as u16) << 8) | (self.c as u16),
            &DE => ((self.d as u16) << 8) | (self.e as u16),
            &HL => ((self.h as u16) << 8) | (self.l as u16),
            &SP => self.sp,
        }
    }

    pub fn write16(&mut self, reg: &Reg16, value: u16) {
        use self::Reg16::*;
        match reg {
            &AF => {
                self.a = (value >> 8) as u8;
                self.f = Flags::from_bits_truncate(value as u8)
            }
            &BC => {
                self.b = (value >> 8) as u8;
                self.c = value as u8
            }
            &DE => {
                self.d = (value >> 8) as u8;
                self.e = value as u8
            }
            &HL => {
                self.h = (value >> 8) as u8;
                self.l = value as u8
            }
            &SP => self.sp = value
        }
    }
}
