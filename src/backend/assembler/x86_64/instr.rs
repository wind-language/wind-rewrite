pub mod instructions {
    use super::super::{x86::opmod, regs::*, mem::*};

    pub trait Mov {
        fn mov(self) -> Vec<u8>;
    }
    
    macro_rules! push_rex {
        ($code:expr, $dst:expr, $src:expr) => {
            if $dst.size == 8 || $dst.id > 7 || $src.size == 8 || $src.id > 7 {
                let rex = 0x40 
                    | ((($src.id > 7) as u8) << 2) 
                    | ((($dst.size == 8) as u8) << 3) 
                    | (($dst.id > 7) as u8);
                $code.push(rex);
            }
        };
        ($code:expr, $dst:expr) => {
            if $dst.size == 8 || $dst.id > 7 {
                let rex = 0x40 
                    | ((($dst.size == 8) as u8) << 3) 
                    | (($dst.id > 7) as u8);
                $code.push(rex);
            }
        };
    }

    impl Mov for (GPR, GPR) {
        fn mov(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();

            if dst.size != src.size {
                panic!("Error: src and dst registers are not the same size");
            }

            push_rex!(code, dst, src);

            const OPCODE: [u8; 4] = [0x88, 0x89, 0x89, 0x89];
            code.push(OPCODE[dst.size.trailing_zeros() as usize]);
            code.push(
                opmod::encode(opmod::REG, src.id, dst.id)
            );
            code
        }
    }

    impl Mov for (GPR, isize) {
        fn mov(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();

            push_rex!(code, dst);

            if src > (1 << (dst.size*8))-1 {
                panic!("Error: imm src is too large for the destination register");
            }

            const OPCODE: [u8; 4] = [0xB0, 0xB8, 0xB8, 0xB8];
            code.push(OPCODE[dst.size.trailing_zeros() as usize] | dst.id);
            
            for byte in src.to_le_bytes().iter().take(dst.size as usize) {
                code.push(*byte);
            }

            code
        }
    }

    impl Mov for (GPR, RegPtr) {
        fn mov(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();

            if dst.size != src.reg.size {
                panic!("Error: mem base and dst registers are not the same size");
            }
            
            push_rex!(code, src.reg, dst);

            const OPCODE: [u8; 4] = [0x8A, 0x8B, 0x8B, 0x8B];
            code.push(OPCODE[dst.size.trailing_zeros() as usize] | dst.id);

            let d32: bool = src.offset > i8::MAX as i32 || src.offset < i8::MIN as i32;
            
            code.push(
                opmod::encode(
                    if src.offset == 0 {opmod::MEM_ADDR} else if d32 {opmod::MEM_ADDR_DISP32} else {opmod::MEM_ADDR_DISP8},
                    dst.id,
                    src.reg.id
                )
            );

            if src.offset != 0 {
                // signed 8-bit displacement or signed 32-bit displacement
                if d32 {
                    for byte in src.offset.to_le_bytes().iter() {
                        code.push(*byte);
                    }
                }
                else {
                    code.push(src.offset as u8);
                }
            }

            code
        }
    }

}