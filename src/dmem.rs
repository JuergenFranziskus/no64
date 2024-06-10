use cpu_mips3::{core::MipsErr, word::Word};


pub struct DMem(Vec<Word>);
impl DMem {
    pub fn init() -> Self {
        Self(vec![Word::zero(); DMEM_WORDS])
    }

    pub fn read_word_for_cpu(&self, addr: u32) -> Result<Option<Word>, MipsErr> {
        match addr {
            DMEM_FIRST..=DMEM_LAST => Ok(Some(self.0[(addr - DMEM_FIRST) as usize / 4])),
            _ => Ok(None)
        }
    }
}

pub const DMEM_FIRST: u32 = 0x04000000;
pub const DMEM_LAST: u32 = 0x04000FFF;
pub const DMEM_WORDS: usize = DMEM_BYTES / 4;
pub const DMEM_BYTES: usize = 4096;
