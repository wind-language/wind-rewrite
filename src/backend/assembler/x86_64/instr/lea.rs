use super::instructions::{*, enc::opmod::push_rex};

pub trait Lea {
    fn lea(self) -> Vec<u8>;
}

impl Lea for (GPR, RegPtr) {
    fn lea(self) -> Vec<u8> {
        let (dst, src) = self;
        let mut code = Vec::new();
        push_rex!(code, dst);
        code.push(0x8D);

        // support only 32-bit displacement
        if src.reg.id == 16 {
            // RIP-relative addressing
            code.push(
                enc::opmod::encode(
                    0b00,
                    dst.id,
                    0b101
                )
            );
        } else {
            code.push(
                enc::opmod::encode(
                    0b10,
                    dst.id,
                    0b101
                )
            );
        }

        for byte in src.offset.to_le_bytes().iter().take(4) {
            code.push(*byte);
        }
        
        code
    }
}
