#[repr(align(4))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Word(pub [u8; 4]);
impl Word {
    pub fn zero() -> Self {
        Self::from_u32_le(0)
    }

    pub fn to_u32(self, be: bool) -> u32 {
        if be {
            self.to_u32_be()
        } else {
            self.to_u32_le()
        }
    }
    pub fn from_u32(value: u32, be: bool) -> Self {
        if be {
            Self::from_u32_be(value)
        } else {
            Self::from_u32_le(value)
        }
    }

    pub fn to_u32_be(self) -> u32 {
        u32::from_be_bytes(self.0)
    }
    pub fn to_u32_le(self) -> u32 {
        u32::from_le_bytes(self.0)
    }
    pub fn from_u32_be(value: u32) -> Self {
        Self(value.to_be_bytes())
    }
    pub fn from_u32_le(value: u32) -> Self {
        Self(value.to_le_bytes())
    }

    pub fn overwrite(&mut self, other: Word, offset: u8, size: u8) {
        let mask = match (size, offset) {
            (1, 0) => [true, false, false, false],
            (1, 1) => [false, true, false, false],
            (1, 2) => [false, false, true, false],
            (1, 3) => [false, false, false, true],
            (2, 0) => [true, true, false, false],
            (2, 2) => [false, false, true, true],
            (4, 0) => [true, true, true, true],
            (_, _) => unreachable!(),
        };

        for ((byte, mask), new) in self.0.iter_mut().zip(mask).zip(other.0) {
            if mask {
                *byte = new;
            }
        }
    }
}
