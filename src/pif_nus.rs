use cpu_mips3::{core::MipsErr, word::Word};

pub struct PifNus {
    rom: Vec<Word>,
}
impl PifNus {
    pub fn init() -> Self {
        Self {
            rom: Self::preprocess_rom(),
        }
    }
    fn preprocess_rom() -> Vec<Word> {
        let mut words = Vec::with_capacity(PIF_ROM_WORDS);
        for chunk in PIF_ROM.chunks(4) {
            words.push(Word([chunk[0], chunk[1], chunk[2], chunk[3]]))
        }

        words
    }

    pub fn read_word_for_cpu(&self, addr: u32) -> Result<Option<Word>, MipsErr> {
        match addr {
            PIF_ROM_FIRST..=PIF_ROM_LAST => Ok(Some(self.rom[(addr - PIF_ROM_FIRST) as usize / 4])),
            PIF_RAM_FIRST..=PIF_RAM_LAST => Err(MipsErr::new("reading from PIFNUS ram is not implemented")),
            _ => Ok(None)
        }
    }
}

const PIF_ROM_FIRST: u32 = 0x1FC00000;
const PIF_ROM_LAST: u32 = 0x1FC007BF;

const PIF_RAM_FIRST: u32 = 0x1FC007C0;
const PIF_RAM_LAST: u32 = 0x1FC007FF;

const PIF_RAM_BYTES: usize = 64;

const PIF_ROM_BYTES: usize = 1984;
const PIF_ROM: &[u8; PIF_ROM_BYTES] = include_bytes!("pifrom.NTSC.bin");
const PIF_ROM_WORDS: usize = PIF_ROM_BYTES / 4;
