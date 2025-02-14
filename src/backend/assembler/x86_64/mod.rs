pub mod regs;
pub mod mem;
pub mod enc;
pub mod instr;

#[allow(dead_code)]
pub mod x86 {
    pub use super::regs::*;
    pub use super::mem::*;
    pub use super::enc::*;
    pub use super::instr::*;
}