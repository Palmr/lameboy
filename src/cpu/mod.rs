pub mod registers;
use cpu::registers::*;

use self::super::cart::Cart;

pub struct CPU<'c> {
    registers: Registers,
    cart: &'c Cart
}

impl<'c> CPU<'c> {
    pub fn new(cart: &Cart) -> CPU {
        CPU {
            registers: Registers::new(),
            cart: cart
        }
    }

    pub fn cycle(&mut self) {
        // Fetch
        let op = self.cart.read(self.registers.pc);

        // Decode
        println!("Opcode[{:04X}] = {:02X}", self.registers.pc, op);

        // Execute
        self.registers.pc += 1;
    }
}
