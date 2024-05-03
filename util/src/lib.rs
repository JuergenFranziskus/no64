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
    pub fn to_u32_be(self) -> u32 {
        u32::from_be_bytes(self.0)
    }
    pub fn to_u32_le(self) -> u32 {
        u32::from_le_bytes(self.0)
    }
}
