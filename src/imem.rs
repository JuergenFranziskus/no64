use cpu_mips3::{core::MipsErr, word::Word};


pub struct IMem(Vec<Word>);
impl IMem {
    pub fn init() -> Self {
        Self(vec![Word::zero(); IMEM_WORDS])
    }

    pub fn read_word_for_cpu(&self, addr: u32) -> Result<Option<Word>, MipsErr> {
        match addr {
            IMEM_FIRST..=IMEM_LAST => Ok(Some(self.0[(addr - IMEM_FIRST) as usize / 4])),
            _ => Ok(None)
        }
    }
}

pub const IMEM_FIRST: u32 = 0x04001000;
pub const IMEM_LAST: u32 = 0x04001FFF;
pub const IMEM_WORDS: usize = IMEM_BYTES / 4;
pub const IMEM_BYTES: usize = 4096;
