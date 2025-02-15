use super::instructions::{*, enc::opmod::push_rex};
use super::common;

pub trait Mov {
    fn mov(self) -> Vec<u8>;
}

impl Mov for (GPR, GPR) {
    common::tb1_gprgpr_instr!(mov, [0x88, 0x89, 0x89, 0x89]);
}

impl Mov for (GPR, isize) {
    common::tb1_gprimm_instr!(mov, [0xB0, 0xB8, 0xB8, 0xB8]);
}

impl Mov for (GPR, RegPtr) {
    common::tb1_gprrptr_instr!(mov, [0x8A, 0x8B, 0x8B, 0x8B]);
}

impl Mov for (RegPtr, GPR) {
    common::tb1_rptrgpr_instr!(mov, [0x88, 0x89, 0x89, 0x89]);
}

impl Mov for (RegPtr, isize) {
    common::tb1_rptrimm_instr!(mov, [0xC6, 0xC7, 0xC7, 0xC7], 0b00);
}