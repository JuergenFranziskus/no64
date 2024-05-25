use std::ops::RangeInclusive;

use util::{sext_32, Word};

use crate::{
    core::{BusCore, Happy, RawCore},
    instruction::Instr,
};

use self::cop0::Cop0;

mod cop0;

pub struct Vr4300 {
    pc: u64,
    gp: [u64; 31],
    branch: Option<u64>,
    cop0: Cop0,
}
impl Vr4300 {
    pub fn init() -> Self {
        Self {
            pc: RESET_VECTOR,
            gp: [0; 31],
            branch: None,
            cop0: Cop0::init(),
        }
    }

    pub fn cycle(&mut self, bus: &mut impl SysAd) {
        let Ok(instr) = self.fetch(bus) else {
            return;
        };

        let branch = self.branch.take();

        let Ok(()) = self.do_instr(instr, bus) else {
            return;
        };

        if let Some(tgt) = branch {
            self.pc = tgt;
        } else {
            self.pc += 4;
        }
    }
    fn fetch(&mut self, bus: &mut impl SysAd) -> Happy<Instr> {
        let word = self.read_32(self.pc, bus)?;
        Ok(Instr(word))
    }

    fn can_use_cop0(&self) -> bool {
        let status = self.cop0.status;
        let kernel = status.is_kernel_mode();
        let enable = status.cop0_enabled();
        kernel | enable
    }

    pub fn translate_address_debug(&self, addr: u64) -> Option<u32> {
        self.translate_static_address(addr)
    }
    fn translate_address(&mut self, addr: u64) -> Happy<u32> {
        if let Some(phys) = self.translate_static_address(addr) {
            Ok(phys)
        } else {
            todo!()
        }
    }
    fn normalize_address(&self, addr: u64) -> u64 {
        if self.is_64_bit_mode() {
            addr
        } else {
            sext_32(addr as u32)
        }
    }
    fn translate_static_address(&self, addr: u64) -> Option<u32> {
        let addr = self.normalize_address(addr);
        if self.is_kernel_mode() {
            Self::translate_static_kernel_address(addr)
        } else {
            todo!()
        }
    }
    fn translate_static_kernel_address(addr: u64) -> Option<u32> {
        if CKSEG0.contains(&addr) {
            Some((addr - CKSEG0.start()) as u32)
        } else if CKSEG1.contains(&addr) {
            Some((addr - CKSEG1.start()) as u32)
        } else {
            None
        }
    }

    fn is_64_bit_mode(&self) -> bool {
        self.cop0.status.is_64_bit_mode()
    }
    pub fn is_big_endian(&self) -> bool {
        self.cop0.is_big_endian()
    }

    fn is_kernel_mode(&self) -> bool {
        self.cop0.status.is_kernel_mode()
    }
}
impl RawCore for Vr4300 {
    fn get_reg(&self, reg: u8) -> Happy<u64> {
        assert!(reg < 32);
        if reg == 0 {
            Ok(0)
        } else {
            Ok(self.gp[reg as usize - 1])
        }
    }

    fn set_reg(&mut self, reg: u8, val: u64) -> Happy<()> {
        assert!(reg < 32);
        if reg != 0 {
            self.gp[reg as usize - 1] = val;
        }

        Ok(())
    }

    fn program_counter(&self) -> u64 {
        self.pc
    }

    fn do_mtc0(&mut self, instr: Instr) -> Happy<()> {
        if !self.can_use_cop0() {
            todo!()
        }

        let val = self.get_reg(instr.rt())?;
        match instr.rd() {
            12 => self.cop0.status.write(val as u32),
            16 => self.cop0.config.write(val as u32),
            32.. => unreachable!(),
            or => todo!("Moving to Cop0 register {or} is not implemented"),
        }

        Ok(())
    }

    fn do_branch(&mut self, c: bool, likely: bool, tgt: u64) -> Happy<()> {
        match (c, likely) {
            (false, false) => (),
            (true, false) => self.branch = Some(tgt),
            (false, true) => self.pc += 4,
            (true, true) => self.branch = Some(tgt),
        }

        Ok(())
    }
}
impl<T: SysAd> BusCore<T> for Vr4300 {
    fn read_8(&mut self, addr: u64, bus: &mut T) -> Happy<u8> {
        todo!()
    }

    fn read_16(&mut self, addr: u64, bus: &mut T) -> Happy<u16> {
        todo!()
    }

    fn read_32(&mut self, addr: u64, bus: &mut T) -> Happy<u32> {
        if addr % 4 != 0 {
            todo!()
        }

        let phys = self.translate_address(addr)?;
        let word = bus.read(phys, AccessSize::Four)[0];

        let be = self.is_big_endian();
        Ok(word.to_u32(be))
    }

    fn read_64(&mut self, addr: u64, bus: &mut T) -> Happy<u64> {
        todo!()
    }

    fn write_8(&mut self, addr: u64, data: u8, bus: &mut T) -> Happy<()> {
        todo!()
    }

    fn write_16(&mut self, addr: u64, data: u16, bus: &mut T) -> Happy<()> {
        todo!()
    }

    fn write_32(&mut self, addr: u64, data: u32, bus: &mut T) -> Happy<()> {
        if addr % 4 != 0 {
            todo!()
        };

        let phys = self.translate_address(addr)?;
        let word = Word::from_u32(data, self.is_big_endian());
        bus.write(phys, AccessSize::Four, [word, Word::zero()]);

        Ok(())
    }

    fn write_64(&mut self, addr: u64, data: u64, bus: &mut T) -> Happy<()> {
        todo!()
    }
}

pub trait SysAd {
    fn read(&mut self, address: u32, size: AccessSize) -> [Word; 2];
    fn write(&mut self, address: u32, size: AccessSize, data: [Word; 2]);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AccessSize {
    One,
    Two,
    Four,
    Eight,
}
impl AccessSize {
    pub fn bytes(self) -> u8 {
        match self {
            Self::One => 1,
            Self::Two => 2,
            Self::Four => 4,
            Self::Eight => 8,
        }
    }
}

const RESET_VECTOR: u64 = 0xFFFF_FFFF_BFC0_0000;

const XUSEG: RangeInclusive<u64> = 0x0..=0xFF_FFFF_FFFF;

const XSUSEG: RangeInclusive<u64> = XUSEG;
const XSSEG: RangeInclusive<u64> = 0x4000000000000000..=0x400000FFFFFFFFFF;
const CSSEG: RangeInclusive<u64> = 0xFFFFFFFFC0000000..=0xFFFFFFFFDFFFFFFF;

const XKUSEG: RangeInclusive<u64> = XSUSEG;
const XKSSEG: RangeInclusive<u64> = XSSEG;
const XKPHYS: RangeInclusive<u64> = 0x8000000000000000..=0xBFFFFFFFFFFFFFFF;
const XKSEG: RangeInclusive<u64> = 0xC000000000000000..=0xC00000FF7FFFFFFF;
const CKSEG0: RangeInclusive<u64> = 0xFFFFFFFF80000000..=0xFFFFFFFF9FFFFFFF;
const CKSEG1: RangeInclusive<u64> = 0xFFFFFFFFA0000000..=0xFFFFFFFFBFFFFFFF;
const CKSSEG: RangeInclusive<u64> = CSSEG;
const CKSEG3: RangeInclusive<u64> = 0xFFFFFFFFE0000000..=0xFFFFFFFFFFFFFFFF;
