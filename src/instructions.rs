use core::fmt;

#[derive(PartialEq)]
pub struct Address {
    pub high: u8,
    pub middle: u8,
    pub low: u8,
}

impl Address {
    pub fn to_u16(&self) -> u16 {
        ((self.high as u16) << 8) + ((self.middle as u16) << 4) + self.low as u16
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "0x{:X}", self.to_u16())
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Invalid,
    /// 0x00E0 - CLS
    Clear,
    /// 0x00EE - RET
    Return,
    /// 0x1nnn - JP addr
    JumpTo(Address),
    /// 0x2nnn - CALL addr
    Call(Address),
    /// 0x3xkk - SE Vx, byte
    SkipIfEqualByte {
        vx: usize,
        byte: u8,
    },
    /// 0x4xkk - SNE Vx, byte
    SkipIfNotEqualByte {
        vx: usize,
        byte: u8,
    },
    /// 0x5xy0 - SE Vx, Vy
    SkipIfEqualReg {
        vx: usize,
        vy: usize,
    },
    /// 0x6xkk - LD Vx, byte
    LoadByte {
        vx: usize,
        byte: u8,
    },
    /// 0x7xkk - ADD Vx, byte
    AddByte {
        vx: usize,
        byte: u8,
    },
    /// 0x8xy0 - LD Vx, Vy
    LoadReg {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy1 - OR Vx, Vy
    Or {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy2 - AND Vx, Vy
    And {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy3 - XOR Vx, Vy
    Xor {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy4 - ADD Vx, Vy
    AddReg {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy5 - SUB Vx, Vy
    Subtract {
        vx: usize,
        vy: usize,
    },
    /// 0x8xy6 - SHR Vx {, Vy}
    ShiftRight {
        vx: usize,
    },
    /// 0x8xy7 - SUBN Vx, Vy
    SubtractReverse {
        vx: usize,
        vy: usize,
    },
    /// 0x8xyE - SHL Vx {, Vy}
    ShiftLeft {
        vx: usize,
    },
    /// 0x9xy0 - SNE Vx, Vy
    SkipIfNotEqualReg {
        vx: usize,
        vy: usize,
    },
    /// 0xAnnn - LD I, addr
    LoadAddress(Address),
    /// 0xBnnn - JP V0, addr
    JumpOffset(Address),
    /// 0xCxkk - RND Vx, byte
    Random {
        vx: usize,
        byte: u8,
    },
    /// 0xDxyn - DRW Vx, Vy, nibble
    Draw {
        vx: usize,
        vy: usize,
        nibble: u8,
    },
    /// 0xEx9E - SKP Vx
    SkipIfKeyPressed {
        vx: usize,
    },
    /// 0xExA1 - SKNP Vx
    SkipIfNotKeyPressed {
        vx: usize,
    },
    /// 0xFx07 - LD Vx, DT
    LoadDelay {
        vx: usize,
    },
    /// 0xFx0A - LD Vx, K
    LoadKeyPressed {
        vx: usize,
    },
    /// 0xFx15 - LD DT, Vx
    SetDelay {
        vx: usize,
    },
    /// 0xFx18 - LD ST, Vx
    SetSound {
        vx: usize,
    },
    /// 0xFx1E - ADD I, Vx
    AddAddressOffset {
        vx: usize,
    },
    /// 0xFx29 - LD F, Vx
    LoadSprite {
        vx: usize,
    },
    /// 0xFx33 - LD B, Vx
    SetBCD {
        vx: usize,
    },
    /// 0xFx55 - LD [I], Vx
    LoadRegisters {
        vx: usize,
    },
    /// 0xFx65 - LD Vx, [I]
    ReadRegisters {
        vx: usize,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_debug() {
        let address = Address {
            high: 0x02,
            middle: 0x01,
            low: 0x08,
        };
        assert_eq!(format!("{:?}", address), "0x218")
    }
}
