use super::instructions::{*, enc::opmod::push_rex};
use super::common;

pub trait Sub {
    fn sub(self) -> Vec<u8>;
}

impl Sub for (GPR, GPR) {
    common::tb1_gprgpr_instr!(sub, [0x2A, 0x2B, 0x2B, 0x2B], invert=true);
}

impl Sub for (GPR, isize) {
    common::tb2_gprimm_instr!(sub, [0x80, 0x81, 0x81, 0x81], 0b101);
}

impl Sub for (GPR, RegPtr) {
    common::tb1_gprrptr_instr!(sub, [0x2a, 0x2b, 0x2b, 0x2b]);
}

impl Sub for (RegPtr, GPR) {
    common::tb1_rptrgpr_instr!(sub, [0x28, 0x29, 0x29, 0x29]);
}

impl Sub for (RegPtr, isize) {
    common::tb1_rptrimm_instr!(sub, [0x83, 0x81, 0x81, 0x81], 0b101);
}