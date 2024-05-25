pub const fn bitmask_32(at: u32, width: u32) -> u32 {
    let mut mask = 0;

    // TODO: this is dumb, but should get inlined and const-evaluated in all uses
    let mut i = 0;
    while i < width {
        mask <<= 1;
        mask |= 1;
        i += 1;
    }

    mask << at
}

pub fn set_field_32(data: &mut u32, at: u32, width: u32, value: u32) {
    let mask = bitmask_32(at, width);
    *data &= !mask;
    *data |= (value << at) & mask;
}
pub fn get_field_32(data: u32, at: u32, width: u32) -> u32 {
    let mask = bitmask_32(at, width);
    (data & mask) >> at
}

pub fn set_flag_32(data: &mut u32, at: u32, val: bool) {
    let mask = bitmask_32(at, 1);
    *data &= !mask;
    *data |= if val { mask } else { 0 };
}
pub fn get_flag_32(data: u32, at: u32) -> bool {
    let mask = bitmask_32(at, 1);
    data & mask != 0
}

pub fn sext_16(val: u16) -> u64 {
    val as i16 as i64 as u64
}
pub fn sext_32(val: u32) -> u64 {
    val as i32 as i64 as u64
}

pub fn sign_bit_32(val: u32) -> bool {
    (val as i32) < 0
}

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
