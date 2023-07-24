fn build_u32(bs: [u8; 4]) -> u32 {
    u32::from_le_bytes(bs)
}

pub struct Inst {
    inst: u32,
}

impl Inst {
    pub fn new(inst: [u8; 4]) -> Self {
        Self {
            inst: build_u32(inst),
        }
    }

    pub fn rd(&self) -> usize {
        ((self.inst & 0xF80) >> 7) as usize
    }

    pub fn imm_u(&self) -> u32 {
        self.inst & 0xFFFFF000
    }

    pub fn funct3(&self) -> u8 {
        ((self.inst & 0x7000) >> 12) as u8
    }

    pub fn funct7(&self) -> u8 {
        ((self.inst & 0x40000000) >> 25) as u8
    }

    pub fn rs1(&self) -> usize {
        ((self.inst & 0x001F000) >> 15) as usize
    }

    pub fn rs2(&self) -> usize {
        ((self.inst & 0x01F0000) >> 20) as usize
    }

    pub fn imm_sb(&self) -> u32 {
        ((self.inst & 0x80000000) >> 19)
            | ((self.inst & 0x7C000000) >> 10)
            | ((self.inst & 0xF00) >> 7)
            | ((self.inst & 80) >> 11)
    }

    pub fn imm_i(&self) -> u32 {
        self.inst >> 20
    }

    pub fn imm_i_symbol(&self) -> i32 {
        (self.inst & 0x80000000) as i32 >> 20 | (self.inst >> 20) as i32
    }

    pub fn imm_uj_symbol(&self) -> i32 {
        (((self.inst & 0x80000000) as i32 >> 11) as u32
            | ((self.inst & 0x100000) >> 9)
            | ((self.inst & 0x7FE00000) >> 20)
            | (self.inst & 0xFF000)) as i32
    }
}

#[cfg(test)]
mod test {
    use super::Inst;

    #[test]
    fn test_inst_u() {
        let inst = [0x37, 0x85, 0x0b, 0x00];
        let i = Inst::new(inst);

        assert_eq!(i.rd(), 10);
        assert_eq!(i.imm_u(), 0xb8 << 12);
    }

    #[test]
    fn test_inst_uj() {
        let _ = env_logger::builder().is_test(true).try_init();

        let inst = [0x6f, 0xf0, 0x1f, 0xfa];
        let i = Inst::new(inst);

        assert_eq!(i.rd(), 0);
        assert_eq!(i.imm_uj_symbol(), -96);
    }

    #[test]
    fn test_inst_i() {
        let inst = [0x93, 0x05, 0x80, 0x04];
        let i = Inst::new(inst);

        assert_eq!(i.rd(), 11);
        assert_eq!(i.funct3(), 0);
        assert_eq!(i.rs1(), 0);
        assert_eq!(i.imm_i(), 72);
    }

    /*     #[test]
    fn test_inst_s() {
        let inst = [0x23, 0x00, 0xb5, 0x00];
        let i = Inst::new(inst);

        assert_eq!(i.funct3(), 0);
        assert_eq!(i.rs1(), 11);
        assert_eq!(i.rs2(), 10);
        assert_eq!(i.imm_sb(), 0xb8000);
    } */
}
