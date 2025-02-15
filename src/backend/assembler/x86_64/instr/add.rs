use super::instructions::{*, enc::opmod::push_rex};
use super::common;

pub trait Add {
    fn add(self) -> Vec<u8>;
}

impl Add for (GPR, GPR) {
    common::tb1_gprgpr_instr!(add, [0x00, 0x01, 0x01, 0x01]);
}

impl Add for (GPR, isize) {
    common::tb2_gprimm_instr!(add, [0x80, 0x81, 0x81, 0x81], 0b000);
}

impl Add for (GPR, RegPtr) {
    common::tb1_gprrptr_instr!(add, [0x02, 0x03, 0x03, 0x03]);
}

impl Add for (RegPtr, GPR) {
    common::tb1_rptrgpr_instr!(add, [0x00, 0x01, 0x01, 0x01]);
}

impl Add for (RegPtr, isize) {
    common::tb1_rptrimm_instr!(add, [0x83, 0x81, 0x81, 0x81], 0b000);
}