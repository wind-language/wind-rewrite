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
}