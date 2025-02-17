#[derive(Debug, Clone, PartialEq)]
pub struct GPR {
    pub id: u8,
    pub size: u8
}
impl GPR {
    pub fn new(id: u8, size: u8) -> GPR {
        GPR {
            id,
            size
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SEG {
    pub id: u8,
    pub size: u8
}
impl SEG {
    pub fn new(id: u8, size: u8) -> SEG {
        SEG {
            id,
            size
        }
    }
}

pub const RAX: GPR = GPR{id: 0, size: 8};
pub const RCX: GPR = GPR{id: 1, size: 8};
pub const RDX: GPR = GPR{id: 2, size: 8};
pub const RBX: GPR = GPR{id: 3, size: 8};
pub const RSP: GPR = GPR{id: 4, size: 8};
pub const RBP: GPR = GPR{id: 5, size: 8};
pub const RSI: GPR = GPR{id: 6, size: 8};
pub const RDI: GPR = GPR{id: 7, size: 8};
pub const R8: GPR = GPR{id: 8, size: 8};
pub const R9: GPR = GPR{id: 9, size: 8};
pub const R10: GPR = GPR{id: 10, size: 8};
pub const R11: GPR = GPR{id: 11, size: 8};
pub const R12: GPR = GPR{id: 12, size: 8};
pub const R13: GPR = GPR{id: 13, size: 8};
pub const R14: GPR = GPR{id: 14, size: 8};
pub const R15: GPR = GPR{id: 15, size: 8};
pub const RIP: GPR = GPR{id: 16, size: 8};

pub const EAX: GPR = GPR{id: 0, size: 4};
pub const ECX: GPR = GPR{id: 1, size: 4};
pub const EDX: GPR = GPR{id: 2, size: 4};
pub const EBX: GPR = GPR{id: 3, size: 4};
pub const ESP: GPR = GPR{id: 4, size: 4};
pub const EBP: GPR = GPR{id: 5, size: 4};
pub const ESI: GPR = GPR{id: 6, size: 4};
pub const EDI: GPR = GPR{id: 7, size: 4};
pub const R8D: GPR = GPR{id: 8, size: 4};
pub const R9D: GPR = GPR{id: 9, size: 4};
pub const R10D: GPR = GPR{id: 10, size: 4};
pub const R11D: GPR = GPR{id: 11, size: 4};
pub const R12D: GPR = GPR{id: 12, size: 4};
pub const R13D: GPR = GPR{id: 13, size: 4};
pub const R14D: GPR = GPR{id: 14, size: 4};
pub const R15D: GPR = GPR{id: 15, size: 4};

pub const AX: GPR = GPR{id: 0, size: 2};
pub const CX: GPR = GPR{id: 1, size: 2};
pub const DX: GPR = GPR{id: 2, size: 2};
pub const BX: GPR = GPR{id: 3, size: 2};
pub const SP: GPR = GPR{id: 4, size: 2};
pub const BP: GPR = GPR{id: 5, size: 2};
pub const SI: GPR = GPR{id: 6, size: 2};
pub const DI: GPR = GPR{id: 7, size: 2};
pub const R8W: GPR = GPR{id: 8, size: 2};
pub const R9W: GPR = GPR{id: 9, size: 2};
pub const R10W: GPR = GPR{id: 10, size: 2};
pub const R11W: GPR = GPR{id: 11, size: 2};
pub const R12W: GPR = GPR{id: 12, size: 2};
pub const R13W: GPR = GPR{id: 13, size: 2};
pub const R14W: GPR = GPR{id: 14, size: 2};
pub const R15W: GPR = GPR{id: 15, size: 2};

pub const AL: GPR = GPR{id: 0, size: 1};
pub const CL: GPR = GPR{id: 1, size: 1};
pub const DL: GPR = GPR{id: 2, size: 1};
pub const BL: GPR = GPR{id: 3, size: 1};
pub const SPL: GPR = GPR{id: 4, size: 1};
pub const BPL: GPR = GPR{id: 5, size: 1};
pub const SIL: GPR = GPR{id: 6, size: 1};
pub const DIL: GPR = GPR{id: 7, size: 1};
pub const R8B: GPR = GPR{id: 8, size: 1};
pub const R9B: GPR = GPR{id: 9, size: 1};
pub const R10B: GPR = GPR{id: 10, size: 1};
pub const R11B: GPR = GPR{id: 11, size: 1};
pub const R12B: GPR = GPR{id: 12, size: 1};
pub const R13B: GPR = GPR{id: 13, size: 1};
pub const R14B: GPR = GPR{id: 14, size: 1};
pub const R15B: GPR = GPR{id: 15, size: 1};

pub const CS: SEG = SEG{id: 0, size: 8};
pub const DS: SEG = SEG{id: 1, size: 8};
pub const SS: SEG = SEG{id: 2, size: 8};
pub const ES: SEG = SEG{id: 3, size: 8};
pub const FS: SEG = SEG{id: 4, size: 8};
pub const GS: SEG = SEG{id: 5, size: 8};