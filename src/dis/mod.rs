pub use crate::dis::instructions::decode_instruction;
pub use crate::dis::memory_locations::get_memory_comment;
use std::fmt;

mod instructions;
mod memory_locations;

pub struct Instruction {
    pub name: &'static str,
    pub arg: Option<InstructionArg>,
}

pub enum InstructionArg {
    Data8,
    Data16,
}

impl Instruction {
    pub fn new(name: &'static str, arg: Option<InstructionArg>) -> Instruction {
        Instruction { name, arg }
    }

    pub fn get_length(&self) -> u8 {
        match &self.arg {
            None => 1,
            Some(data) => match data {
                InstructionArg::Data8 => 2,
                InstructionArg::Data16 => 3,
            },
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
