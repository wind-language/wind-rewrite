pub trait Jmp {
    fn jmp(self) -> Vec<u8>;
}

impl Jmp for isize {
    fn jmp(self) -> Vec<u8> {
        let addr_rel = self;
        let mut code = Vec::new();
        
        code.push(0xE9);
        for byte in (addr_rel-5).to_le_bytes().iter().take(4) {
            code.push(*byte);
        }

        code
    }
}