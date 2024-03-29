mod add;
mod and;
mod compare;
mod increment_decrement;
mod or;
mod rotate;
mod shift;
mod sub;
mod swap;
mod xor;

pub use crate::lameboy::cpu::instructions::alu::add::alu_add_8bit;
pub use crate::lameboy::cpu::instructions::alu::and::alu_and_8bit;
pub use crate::lameboy::cpu::instructions::alu::compare::alu_cp_8bit;
pub use crate::lameboy::cpu::instructions::alu::increment_decrement::{alu_dec_8bit, alu_inc_8bit};
pub use crate::lameboy::cpu::instructions::alu::or::alu_or_8bit;
pub use crate::lameboy::cpu::instructions::alu::rotate::alu_rotate_left;
pub use crate::lameboy::cpu::instructions::alu::rotate::alu_rotate_right;
pub use crate::lameboy::cpu::instructions::alu::shift::alu_shift_left;
pub use crate::lameboy::cpu::instructions::alu::shift::alu_shift_right;
pub use crate::lameboy::cpu::instructions::alu::sub::alu_sub_8bit;
pub use crate::lameboy::cpu::instructions::alu::swap::alu_swap_8bit;
pub use crate::lameboy::cpu::instructions::alu::xor::alu_xor_8bit;
