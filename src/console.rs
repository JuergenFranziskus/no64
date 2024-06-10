use cpu_mips3::vr4300::{SysAd, Vr4300};

use crate::{dmem::DMem, imem::IMem, pif_nus::PifNus, rsp::Rsp};
use cpu_mips3::core::MipsErr;
use cpu_mips3::word::Word;
use cpu_mips3::vr4300::WriteSize;

pub struct Console {
    pub cpu: Vr4300,
    pub rsp: Rsp,
    pub dmem: DMem,
    pub imem: IMem,
    pub pif_nus: PifNus,
}
impl Console {
    pub fn init() -> Self {
        Self {
            cpu: Vr4300::init(),
            rsp: Rsp::init(),
            dmem: DMem::init(),
            imem: IMem::init(),
            pif_nus: PifNus::init(),
        }
    }

    pub fn read_debug(&self, addr: u32) -> Option<Word> {
        if let Ok(Some(word)) = self.pif_nus.read_word_for_cpu(addr) {
            Some(word)
        }
        else if let Ok(Some(word)) = self.dmem.read_word_for_cpu(addr) {
            Some(word)
        }
        else if let Ok(Some(word)) = self.imem.read_word_for_cpu(addr) {
            Some(word)
        }
        else if let Ok(Some(word)) = self.rsp.read_word_for_cpu(addr) {
            Some(word)
        }
        else {
            None
        }
    }

    pub fn cpu_and_bus(&mut self) -> (&mut Vr4300, CpuBus) {
        let bus = CpuBus {
            rsp: &mut self.rsp,
            dmem: &mut self.dmem,
            imem: &mut self.imem,
            pif_nus: &mut self.pif_nus,
        };
        (&mut self.cpu, bus)
    }
}



pub struct CpuBus<'a> {
    rsp: &'a mut Rsp,
    dmem: &'a mut DMem,
    imem: &'a mut IMem,
    pif_nus: &'a mut PifNus,
}
impl<'a> SysAd for CpuBus<'a> {
    fn read_word(&mut self, addr: u32) -> Result<Word, MipsErr> {
        if let Some(word) = self.pif_nus.read_word_for_cpu(addr)? {
            Ok(word)
        }
        else if let Some(word) = self.dmem.read_word_for_cpu(addr)? {
            Ok(word)
        }
        else if let Some(word) = self.imem.read_word_for_cpu(addr)? {
            Ok(word)
        }
        else if let Some(word) = self.rsp.read_word_for_cpu(addr)? {
            Ok(word)
        }
        else {
            Ok(Word::zero())
        }
    }

    fn write_word(
        &mut self,
        addr: u32,
        size: WriteSize,
        data: Word,
    ) -> Result<(), MipsErr> {
        return Err(MipsErr::new("unimplemented"))
    }

    fn write_dword(&mut self, addr: u32, data: [Word; 2]) -> Result<(), MipsErr> {
        return Err(MipsErr::new("unimplemented"))
    }

    fn read_cached_data(&mut self, addr: u32) -> Result<[Word; 4], MipsErr> {
        return Err(MipsErr::new("unimplemented"))
    }

    fn read_cached_inst(&mut self, addr: u32) -> Result<[Word; 8], MipsErr> {
        return Err(MipsErr::new("unimplemented"))
    }

    fn write_cached_data(&mut self, addr: u32, data: [Word; 4]) -> Result<(), MipsErr> {
        return Err(MipsErr::new("unimplemented")) 
    }
}
