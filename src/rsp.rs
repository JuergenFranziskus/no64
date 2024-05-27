use cpu_mips3::{
    vr4300::{read_word, write_word, AccessSize},
    word::Word,
};

pub struct Rsp {
    dmem: Vec<Word>,
    imem: Vec<Word>,
}
impl Rsp {
    pub fn init() -> Self {
        Self {
            dmem: vec![Word::zero(); DMEM_WORDS],
            imem: vec![Word::zero(); IMEM_WORDS],
        }
    }

    pub fn read_debug(&self, addr: u32) -> Option<Word> {
        match addr {
            DMEM_FIRST..=DMEM_LAST => {
                Some(read_word(&self.dmem, addr - DMEM_FIRST, AccessSize::Four)[0])
            }
            IMEM_FIRST..=IMEM_LAST => {
                Some(read_word(&self.imem, addr - IMEM_FIRST, AccessSize::Four)[0])
            }
            _ => None,
        }
    }
    pub fn read_for_cpu(&mut self, addr: u32, size: AccessSize) -> Option<[Word; 2]> {
        match addr {
            DMEM_FIRST..=DMEM_LAST => Some(read_word(&self.dmem, addr - DMEM_FIRST, size)),
            IMEM_FIRST..=IMEM_LAST => Some(read_word(&self.imem, addr - IMEM_FIRST, size)),
            REGS_FIRST..=REGS_LAST => Some([self.read_reg_for_cpu(addr), Word::zero()]),
            _ => None,
        }
    }
    fn read_reg_for_cpu(&mut self, addr: u32) -> Word {
        match addr {
            SP_STATUS => Word::from_u32_be(1),
            SP_DMA_BUSY => Word::zero(),
            or => panic!("Reading from RSP register {or} is not implemented"),
        }
    }
    pub fn write_for_cpu(&mut self, addr: u32, size: AccessSize, data: [Word; 2]) -> bool {
        match addr {
            DMEM_FIRST..=DMEM_LAST => {
                write_word(&mut self.dmem, addr - DMEM_FIRST, size, data);
                true
            }
            IMEM_FIRST..=IMEM_LAST => {
                write_word(&mut self.imem, addr - IMEM_FIRST, size, data);
                true
            }
            _ => false,
        }
    }
}

pub const DMEM_FIRST: u32 = 0x04000000;
pub const DMEM_LAST: u32 = 0x04000FFF;
pub const IMEM_FIRST: u32 = 0x04001000;
pub const IMEM_LAST: u32 = 0x04001FFF;
pub const DMEM_WORDS: usize = DMEM_BYTES / 4;
pub const DMEM_BYTES: usize = 4096;
pub const IMEM_WORDS: usize = IMEM_BYTES / 4;
pub const IMEM_BYTES: usize = 4096;

pub const REGS_FIRST: u32 = 0x04040000;
pub const REGS_LAST: u32 = 0x040BFFFF;

pub const SP_DMA_SPADDR: u32 = 0x0404_0000;
pub const SP_DMA_RAMADDR: u32 = 0x0404_0004;
pub const SP_DMA_RDLEN: u32 = 0x0404_0008;
pub const SP_DMA_WRLEN: u32 = 0x0404_000C;
pub const SP_STATUS: u32 = 0x0404_0010;
pub const SP_DMA_FULL: u32 = 0x0404_0014;
pub const SP_DMA_BUSY: u32 = 0x0404_0018;
pub const SP_SEMAPHORE: u32 = 0x0404_001C;
pub const SP_PC: u32 = 0x0408_0000;
