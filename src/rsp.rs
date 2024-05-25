use cpu_mips3::vr4300::AccessSize;
use util::Word;

pub struct Rsp {}
impl Rsp {
    pub fn init() -> Self {
        Self {}
    }

    pub fn read_for_cpu(&mut self, byte_offset: usize, _size: AccessSize) -> [Word; 2] {
        let register = byte_offset / 4;

        match register {
            4 => [Word::from_u32_be(0x1), Word::zero()],
            6 => [Word::zero(); 2],
            8.. => unreachable!(),
            _ => todo!("Reading from RSP register {register} is not implemented"),
        }
    }

    pub fn write_for_cpu(&self, byte_offset: usize, size: AccessSize, data: [Word; 2]) {
        let register = byte_offset / 4;

        match register {
            4 => (),
            8.. => unreachable!(),
            _ => todo!("Writing to RSP register {register} is not implemented"),
        }
    }
}
