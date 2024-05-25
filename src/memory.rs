use cpu_mips3::vr4300::AccessSize;
use util::Word;

pub struct Memory {
    pub dram: Vec<Word>,
    pub dmem: Vec<Word>,
    pub imem: Vec<Word>,
    pub pif_rom: Vec<Word>,
}
impl Memory {
    pub fn init() -> Self {
        Self {
            dram: vec![Word::zero(); DRAM_WORDS],
            dmem: vec![Word::zero(); DMEM_WORDS],
            imem: vec![Word::zero(); IMEM_WORDS],
            pif_rom: Self::preprocess_pif_rom(),
        }
    }
    fn preprocess_pif_rom() -> Vec<Word> {
        let mut words = Vec::with_capacity(PIF_ROM_WORDS);
        for i in (0..PIF_ROM_BYTES).step_by(4) {
            let b0 = PIF_ROM[i + 0];
            let b1 = PIF_ROM[i + 1];
            let b2 = PIF_ROM[i + 2];
            let b3 = PIF_ROM[i + 3];
            words.push(Word([b0, b1, b2, b3]));
        }

        words
    }

    pub fn read_for_cpu(&self, address: u32, size: AccessSize) -> Option<[Word; 2]> {
        match address {
            DRAM_FIRST..=DRAM_LAST => Some(Self::read_word_for_cpu(
                &self.dram,
                (address - DRAM_FIRST) as usize,
                size,
            )),
            DMEM_FIRST..=DMEM_LAST => Some(Self::read_word_for_cpu(
                &self.dmem,
                (address - DMEM_FIRST) as usize,
                size,
            )),
            IMEM_FIRST..=IMEM_LAST => Some(Self::read_word_for_cpu(
                &self.imem,
                (address - IMEM_FIRST) as usize,
                size,
            )),
            PIF_ROM_FIRST..=PIF_ROM_LAST => Some(Self::read_word_for_cpu(
                &self.pif_rom,
                (address - PIF_ROM_FIRST) as usize,
                size,
            )),
            _ => None,
        }
    }
    pub fn write_for_cpu(&mut self, address: u32, size: AccessSize, data: [Word; 2]) -> bool {
        match address {
            DRAM_FIRST..=DRAM_LAST => {
                Self::write_word_for_cpu(
                    &mut self.dram,
                    (address - DRAM_FIRST) as usize,
                    size,
                    data,
                );
                true
            }
            DMEM_FIRST..=DMEM_LAST => {
                Self::write_word_for_cpu(
                    &mut self.dmem,
                    (address - DMEM_FIRST) as usize,
                    size,
                    data,
                );
                true
            }
            IMEM_FIRST..=IMEM_LAST => {
                Self::write_word_for_cpu(
                    &mut self.imem,
                    (address - IMEM_FIRST) as usize,
                    size,
                    data,
                );
                true
            }
            PIF_ROM_FIRST..=PIF_ROM_LAST => {
                Self::write_word_for_cpu(
                    &mut self.pif_rom,
                    (address - IMEM_FIRST) as usize,
                    size,
                    data,
                );
                true
            }

            _ => false,
        }
    }

    fn read_word_for_cpu(words: &[Word], byte_offset: usize, size: AccessSize) -> [Word; 2] {
        let word_offset = byte_offset / 4;
        let lo_word = words[word_offset];
        let hi_word = if size == AccessSize::Eight {
            words[word_offset + 1]
        } else {
            Word::zero()
        };

        [lo_word, hi_word]
    }
    fn write_word_for_cpu(
        words: &mut [Word],
        byte_offset: usize,
        size: AccessSize,
        data: [Word; 2],
    ) {
        let word_offset = byte_offset / 4;

        if size == AccessSize::Eight {
            words[word_offset + 0] = data[0];
            words[word_offset + 1] = data[1];
        } else {
            let suboffset = (byte_offset % 4) as u8;
            words[word_offset].overwrite(data[0], suboffset, size.bytes());
        }
    }

    pub fn read_debug(&self, address: u32) -> Option<Word> {
        self.read_for_cpu(address, AccessSize::Four).map(|d| d[0])
    }
}

const DRAM_FIRST: u32 = 0;
const DRAM_LAST: u32 = 0x03EF_FFFF;

const DMEM_FIRST: u32 = 0x04000000;
const DMEM_LAST: u32 = 0x04000FFF;

const IMEM_FIRST: u32 = 0x04001000;
const IMEM_LAST: u32 = 0x04001FFF;

const PIF_ROM_FIRST: u32 = 0x1FC00000;
const PIF_ROM_LAST: u32 = 0x1FC007BF;

pub const DRAM_WORDS: usize = DRAM_BYTES / 4;
pub const DRAM_BYTES: usize = 8 * 1024 * 1024;
pub const DRAM_ACCESS_MASK: usize = DRAM_BYTES - 1;

pub const DMEM_WORDS: usize = DMEM_BYTES / 4;
pub const DMEM_BYTES: usize = 4096;

pub const IMEM_WORDS: usize = IMEM_BYTES / 4;
pub const IMEM_BYTES: usize = 4096;

const PIF_ROM_WORDS: usize = PIF_ROM_BYTES / 4;
const PIF_ROM_BYTES: usize = 1984; // Literally 1984
const PIF_ROM: &[u8; PIF_ROM_BYTES] = include_bytes!("./PIF_ROM_NTSC.bin");
