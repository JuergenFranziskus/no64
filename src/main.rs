use std::ops::RangeInclusive;

use cpu_mips3::vr4300::{SysAd, Vr4300};
use util::Word;

fn main() {
    let mut cpu = Vr4300::init();
    let mut mem = DummyMem::init();

    for _ in 0..1000 {
        let (pc, except, instr, delayed) = cpu.cycle(&mut mem);

        print!("{pc:x}: ");
        if except {
            print!(" EXCEPTION ");
        } else {
            print!("           ");
        }

        if let Some(instr) = instr {
            print!("{instr}");
            if let Some(delayed) = delayed {
                print!("\t=> 0x{delayed:x}");
            }
        }
        println!();
    }
}

pub struct DummyMem {
    pif_rom: Vec<Word>,
}
impl DummyMem {
    fn init() -> Self {
        Self {
            pif_rom: Self::preprocess_pif_rom(),
        }
    }

    fn preprocess_pif_rom() -> Vec<Word> {
        let mut words = Vec::with_capacity(PIF_ROM_WORDS);
        for i in (0..PIF_ROM_LEN).step_by(4) {
            let b0 = PIF_ROM_BYTES[i + 0];
            let b1 = PIF_ROM_BYTES[i + 1];
            let b2 = PIF_ROM_BYTES[i + 2];
            let b3 = PIF_ROM_BYTES[i + 3];
            words.push(Word([b0, b1, b2, b3]));
        }

        words
    }
}
impl SysAd for DummyMem {
    fn read(&mut self, address: u32, size: cpu_mips3::vr4300::AccessSize) -> Word {
        if PIF_ROM_RANGE.contains(&address) {
            let offset = address - PIF_ROM_RANGE.start();
            let word = offset / 4;
            self.pif_rom[word as usize]
        } else {
            Word([0; 4])
        }
    }

    fn write(&mut self, address: u32, size: cpu_mips3::vr4300::AccessSize, data: Word) {
        todo!()
    }
}

const PIF_ROM_RANGE: RangeInclusive<u32> = 0x1FC00000..=0x1FC007BF;
const PIF_ROM_WORDS: usize = PIF_ROM_LEN / 4;
const PIF_ROM_LEN: usize = 1984;
const PIF_ROM_BYTES: &[u8; PIF_ROM_LEN] = include_bytes!("./PIF_ROM_NTSC.bin");
