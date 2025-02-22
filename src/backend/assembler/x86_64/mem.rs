use super::regs::*;



#[derive(Debug, Clone, PartialEq)]
pub struct RegPtr{
    pub reg: GPR,
    pub offset: i32,
    pub size: u8
}

pub fn ptr(reg: GPR, offset: i32, size: u8) -> RegPtr {
    RegPtr {
        reg,
        offset,
        size
    }
}