pub mod common;

pub mod mov;
pub mod add;
pub mod sub;

pub mod instructions {
    pub use super::super::{x86::opmod, regs::*, mem::*, enc};

    pub use super::mov::Mov;
    pub use super::add::Add;
    pub use super::sub::Sub;
}