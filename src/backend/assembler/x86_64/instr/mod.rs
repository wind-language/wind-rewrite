pub mod common;

pub mod mov;
pub mod add;
pub mod sub;
pub mod jmp;
pub mod call;
pub mod lea;

pub mod instructions {
    pub use super::super::{x86::opmod, regs::*, mem::*, enc};

    pub use super::mov::Mov;
    pub use super::add::Add;
    pub use super::sub::Sub;
    pub use super::jmp::Jmp;
    pub use super::call::Call;
    pub use super::lea::Lea;
}