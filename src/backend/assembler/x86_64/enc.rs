pub mod opmod {
    /*
        REX.W:
        ----------------------------------------
        |    W   |    R    |    X    |    B    |
        |    3   |    2    |    1    |    0    |
        ----------------------------------------
    */

    pub const MEM_ADDR: u8 = 0b00;
    pub const MEM_ADDR_DISP8: u8 = 0b01;
    pub const MEM_ADDR_DISP32: u8 = 0b10;
    pub const REG: u8 = 0b11;

    pub fn encode(modb: u8, reg: u8, rm: u8) -> u8 {
        (modb << 6) | ((reg & 0x7) << 3) | (rm & 0x7)
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
    pub(crate) use push_rex;
}