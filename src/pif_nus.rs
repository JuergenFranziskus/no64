use cpu_mips3::{
    vr4300::{read_word, read_word_from_bytes, write_word_to_bytes, AccessSize},
    word::Word,
};

pub struct PifNus {
    rom: Vec<Word>,
    ram: Vec<u8>,
}
impl PifNus {
    pub fn init() -> Self {
        Self {
            rom: Self::preprocess_rom(),
            ram: vec![0; PIF_RAM_BYTES],
        }
    }
    fn preprocess_rom() -> Vec<Word> {
        let mut words = Vec::with_capacity(PIF_ROM_WORDS);
        for chunk in PIF_ROM.chunks(4) {
            words.push(Word([chunk[0], chunk[1], chunk[2], chunk[3]]))
        }

        words
    }

    pub fn read_debug(&self, addr: u32) -> Option<Word> {
        match addr {
            PIF_ROM_FIRST..=PIF_ROM_LAST => {
                Some(read_word(&self.rom, addr - PIF_ROM_FIRST, AccessSize::Four)[0])
            }
            PIF_RAM_FIRST..=PIF_RAM_LAST => {
                Some(read_word_from_bytes(&self.ram, addr - PIF_RAM_FIRST, AccessSize::Four)[0])
            }
            _ => None,
        }
    }
    pub fn read_for_cpu(&self, addr: u32, size: AccessSize) -> Option<[Word; 2]> {
        match addr {
            PIF_ROM_FIRST..=PIF_ROM_LAST => Some(read_word(&self.rom, addr - PIF_ROM_FIRST, size)),
            PIF_RAM_FIRST..=PIF_RAM_LAST => Some(read_word_from_bytes(
                &self.ram,
                addr - PIF_RAM_FIRST,
                AccessSize::Four,
            )),
            _ => None,
        }
    }
    pub fn write_for_cpu(&mut self, addr: u32, size: AccessSize, data: [Word; 2]) -> bool {
        if addr < PIF_RAM_FIRST || addr > PIF_RAM_LAST {
            return false;
        };
        let offset = addr - PIF_RAM_FIRST;
        write_word_to_bytes(&mut self.ram, offset, size, data);

        if offset == 0x3F {
            todo!(
                "Writing to last byte of PIF ram initiates unimplemented PIF_NUS command response"
            );
        }

        true
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
