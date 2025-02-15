macro_rules! tb1_gprgpr_instr {
    // Original pattern
    ($name:ident, $opcodes:expr) => {
        fn $name(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();

            if dst.size != src.size {
                panic!("Error: src and dst registers are not the same size");
            }

            push_rex!(code, dst, src);

            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize]);
            code.push(opmod::encode(opmod::REG, src.id, dst.id));
            code
        }
    };

    ($name:ident, $opcodes:expr, invert=$invert:expr) => {
        fn $name(self) -> Vec<u8> {
            let (orig_dst, orig_src) = self;
            let (dst, src) = if $invert { (orig_src, orig_dst) } else { (orig_dst, orig_src) };
            let mut code = Vec::new();

            if dst.size != src.size {
                panic!("Error: src and dst registers are not the same size");
            }

            push_rex!(code, dst, src);

            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize]);
            code.push(opmod::encode(opmod::REG, src.id, dst.id));
            code
        }
    };
}
pub (crate) use tb1_gprgpr_instr;

macro_rules! tb1_gprimm_instr {
    ($name:ident, $opcodes:expr) => {
        fn $name(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();
    
            push_rex!(code, dst);
    
            if src as usize > (1 << ((dst.size*8)-1)) {
                panic!("Error: imm src is too large for the destination register");
            }
    
            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize] | dst.id);
            
            for byte in src.to_le_bytes().iter().take(dst.size as usize) {
                code.push(*byte);
            }
    
            code
        }
    };

    ($name:ident, $opcodes:expr, invert=$invert:expr) => {
        fn $name(self) -> Vec<u8> {
            let (orig_dst, orig_src) = self;
            let (dst, src) = if $invert { (orig_src, orig_dst) } else { (orig_dst, orig_src) };
            let mut code = Vec::new();
    
            push_rex!(code, dst);
    
            if src as usize > (1 << ((dst.size*8)-1)) {
                panic!("Error: imm src is too large for the destination register");
            }
    
            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize] | dst.id);
            
            for byte in src.to_le_bytes().iter().take(dst.size as usize) {
                code.push(*byte);
            }
    
            code
        }
    };
}
pub (crate) use tb1_gprimm_instr;

macro_rules! tb2_gprimm_instr {
    ($name:ident, $opcodes:expr, $opbit:expr) => {
        fn $name(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();
    
            push_rex!(code, dst);
    
            if src as usize > (1 << ((dst.size*8)-1)) {
                panic!("Error: imm src is too large for the destination register");
            }
    
            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize]);

            code.push(
                opmod::encode(
                    opmod::REG,
                    $opbit,
                    dst.id
                )
            );
            
            for byte in src.to_le_bytes().iter().take(dst.size as usize) {
                code.push(*byte);
            }
    
            code
        }
    };
}
pub (crate) use tb2_gprimm_instr;

macro_rules! tb1_gprrptr_instr {
    ($name:ident, $opcodes:expr) => {
        fn $name(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();
    
            if dst.size != src.size {
                panic!("Error: mem and dst reg are not the same size");
            }
            
            push_rex!(code, src.reg, dst);
    
            const OPCODE: [u8; 4] = $opcodes;
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
    };
}
pub (crate) use tb1_gprrptr_instr;

macro_rules! tb1_rptrgpr_instr {
    ($name:ident, $opcodes:expr) => {
        fn $name(self) -> Vec<u8> {
            let (dst, src) = self;
            let mut code = Vec::new();
    
            if dst.size != src.size {
                panic!("Error: src reg and mem are not the same size");
            }
            if dst.size == 2 { code.push(0x66); }
    
            push_rex!(code, src);
            let d32: bool = dst.offset > i8::MAX as i32 || dst.offset < i8::MIN as i32;
    
            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.reg.size.trailing_zeros() as usize]);
            code.push(
                opmod::encode(
                    if dst.offset == 0 {opmod::MEM_ADDR} else if d32 {opmod::MEM_ADDR_DISP32} else {opmod::MEM_ADDR_DISP8},
                    src.id,
                    dst.reg.id
                )
            );
    
            if dst.offset != 0 {
                if d32 {
                    for byte in dst.offset.to_le_bytes().iter() {
                        code.push(*byte);
                    }
                }
                else {
                    code.push(dst.offset as u8);
                }
            }
    
            code
        }
    };
}
pub (crate) use tb1_rptrgpr_instr;

macro_rules! tb1_rptrimm_instr {
    ($name:ident, $opcodes:expr, $opbits:expr) => {
        fn $name(self) -> Vec<u8>  {
            let (dst, src) = self;
            let mut code = Vec::new();
    
            if dst.size == 2 { code.push(0x66); }
    
            push_rex!(code, GPR{id: dst.reg.id, size: dst.size});
    
            if src as usize > (1 << ((dst.size*8)-1)) {
                panic!("Error: imm src is too large for the destination register");
            }
    
            const OPCODE: [u8; 4] = $opcodes;
            code.push(OPCODE[dst.size.trailing_zeros() as usize]);
            code.push(
                opmod::encode(
                    if dst.offset == 0 {opmod::MEM_ADDR} else {opmod::MEM_ADDR_DISP8},
                    $opbits,
                    dst.reg.id
                )
            );
    
            if dst.offset != 0 {
                code.push(dst.offset as u8);
            }
    
            for byte in src.to_le_bytes().iter().take((dst.size).min(4) as usize) {
                code.push(*byte);
            }
    
            code
        }
    };
}
pub (crate) use tb1_rptrimm_instr;