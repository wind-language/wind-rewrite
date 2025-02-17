pub trait Call {
    fn call(self) -> Vec<u8>;
}

impl Call for isize {
    fn call(self) -> Vec<u8> {
        let addr_rel = self;
        let mut code = Vec::new();
        
        code.push(0xE8);
        for byte in (addr_rel-5).to_le_bytes().iter().take(4) {
            code.push(*byte);
        }

        code
    }
}