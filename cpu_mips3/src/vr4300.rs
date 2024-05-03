use std::ops::RangeInclusive;

use util::{sext_32, Word};

use crate::{
    core::{Happy, MipsCore},
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

    pub fn cycle(&mut self, bus: &mut impl SysAd) -> (u64, bool, Option<Instr>, Option<u64>) {
        let pc = self.pc;
        let Ok(instr) = self.fetch(bus) else {
            return (pc, true, None, None);
        };

        let branch = self.branch.take();

        let Ok(()) = self.do_instr(instr, bus) else {
            return (pc, true, Some(instr), None);
        };

        if let Some(tgt) = branch {
            self.pc = tgt;
        }

        (pc, false, Some(instr), branch)
    }
    fn fetch(&mut self, bus: &mut impl SysAd) -> Happy<Instr> {
        let word = self.read_32(self.pc, bus)?;
        self.pc += 4;
        Ok(Instr(word))
    }

    fn can_use_cop0(&self) -> bool {
        let status = self.cop0.status;
        let kernel = status.is_kernel_mode();
        let enable = status.cop0_enabled();
        kernel | enable
    }

    fn translate_address(&mut self, addr: u64) -> Happy<u32> {
        let addr = self.normalize_address(addr);

        if self.is_kernel_mode() {
            self.translate_kernel_address(addr)
        } else if self.is_supervisor_mode() {
            self.translate_supervisor_address(addr)
        } else if self.is_user_mode() {
            self.translate_user_address(addr)
        } else {
            unreachable!()
        }
    }
    fn normalize_address(&self, addr: u64) -> u64 {
        if self.is_64_bit_mode() {
            addr
        } else {
            sext_32(addr as u32)
        }
    }
    fn translate_user_address(&mut self, addr: u64) -> Happy<u32> {
        todo!()
    }
    fn translate_supervisor_address(&mut self, addr: u64) -> Happy<u32> {
        todo!()
    }
    fn translate_kernel_address(&mut self, addr: u64) -> Happy<u32> {
        if XKUSEG.contains(&addr) {
            todo!()
        } else if XKSSEG.contains(&addr) {
            todo!()
        } else if XKPHYS.contains(&addr) {
            todo!()
        } else if XKSEG.contains(&addr) {
            todo!()
        } else if CKSEG0.contains(&addr) {
            Ok((addr - CKSEG0.start()) as u32)
        } else if CKSEG1.contains(&addr) {
            Ok((addr - CKSEG1.start()) as u32)
        } else if CKSSEG.contains(&addr) {
            todo!()
        } else if CKSEG3.contains(&addr) {
            todo!()
        } else {
            todo!()
        }
    }

    fn is_64_bit_mode(&self) -> bool {
        self.cop0.status.is_64_bit_mode()
    }
    fn is_big_endian(&self) -> bool {
        self.cop0.is_big_endian()
    }

    fn is_kernel_mode(&self) -> bool {
        self.cop0.status.is_kernel_mode()
    }
    fn is_supervisor_mode(&self) -> bool {
        self.cop0.status.is_supervisor_mode()
    }
    fn is_user_mode(&self) -> bool {
        self.cop0.status.is_user_mode()
    }
}
impl<T: SysAd> MipsCore<T> for Vr4300 {
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
        let word = bus.read(phys, AccessSize::Four);

        if self.is_big_endian() {
            Ok(word.to_u32_be())
        } else {
            Ok(word.to_u32_le())
        }
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
        todo!()
    }

    fn write_64(&mut self, addr: u64, data: u64, bus: &mut T) -> Happy<()> {
        todo!()
    }

    fn do_mtc0(&mut self, instr: Instr) -> Happy<()> {
        if !self.can_use_cop0() {
            todo!()
        }

        let val = <Self as MipsCore<T>>::get_reg(self, instr.rt())?;
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

pub trait SysAd {
    fn read(&mut self, address: u32, size: AccessSize) -> Word;
    fn write(&mut self, address: u32, size: AccessSize, data: Word);
}

pub enum AccessSize {
    One,
    Two,
    Four,
}

const RESET_VECTOR: u64 = 0xFFFF_FFFF_BFC0_0000;

const XUSEG: RangeInclusive<u64> = 0x0..=0xFF_FFFF_FFFF;

const XSUSEG: RangeInclusive<u64> = XUSEG;
const XSSEG: RangeInclusive<u64> = 0x4000000000000000..=0x400000FFFFFFFFFF;
const CSSEG: RangeInclusive<u64> = 0xFFFFFFFFC0000000..=0xFFFFFFFFDFFFFFFF;

const XKUSEG: RangeInclusive<u64> = XUSEG;
const XKSSEG: RangeInclusive<u64> = XSSEG;
const XKPHYS: RangeInclusive<u64> = 0x8000000000000000..=0xBFFFFFFFFFFFFFFF;
const XKSEG: RangeInclusive<u64> = 0xC000000000000000..=0xC00000FF7FFFFFFF;
const CKSEG0: RangeInclusive<u64> = 0xFFFFFFFF80000000..=0xFFFFFFFF9FFFFFFF;
const CKSEG1: RangeInclusive<u64> = 0xFFFFFFFFA0000000..=0xFFFFFFFFBFFFFFFF;
const CKSSEG: RangeInclusive<u64> = 0xFFFFFFFFC0000000..=0xFFFFFFFFDFFFFFFF;
const CKSEG3: RangeInclusive<u64> = 0xFFFFFFFFE0000000..=0xFFFFFFFFFFFFFFFF;
