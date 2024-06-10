use cpu_mips3::{core::MipsErr, word::Word};


pub struct Rsp {
}
impl Rsp {
    pub fn init() -> Self {
        Self {

        }
    }
    
    pub fn read_word_for_cpu(&self, addr: u32) -> Result<Option<Word>, MipsErr> {
        if !(REGS_FIRST..=REGS_LAST).contains(&addr) { return Ok(None) }
        let reg = (addr - REGS_FIRST) / 4;
        match reg {
            4 => Ok(Some(Word::from_u32_le(1))),
            or => Err(MipsErr::new(format!("reading from RSP register {or} is not implemented"))),
        }
    }
}


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
