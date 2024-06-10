use std::ops::RangeInclusive;

use cop0::{Config, Cop0, Mode, Status};
use util::sext_32;

use crate::{
    core::{MipsCore, MipsErr, MipsResult, RawCore},
    instruction::{Instr, Reg},
    word::Word,
};

mod cop0;

pub struct Vr4300 {
    cycle: u64,
    pc: u64,
    gp: [i64; 31],
    lohi: [i64; 2],

    branch: Option<u64>,
    cop0: Cop0,
}
impl Vr4300 {
    pub fn init() -> Self {
        Self {
            cycle: 0,
            pc: RESET_VECTOR,
            gp: [0; 31],
            lohi: [0; 2],

            branch: None,
            cop0: Cop0::init(),
        }
    }
    pub fn step_forward(&mut self, bus: &mut impl SysAd) -> Result<(), MipsErr> {
        fn inner(cpu: &mut Vr4300, bus: &mut impl SysAd) -> MipsResult<()> {
            let instr = cpu.fetch(bus)?;
            let branch = cpu.branch.take();
            cpu.do_instruction(instr, bus)?;

            cpu.pc = branch.unwrap_or(cpu.pc + 4);
            Ok(())
        }

        if let Err(Some(e)) = inner(self, bus) {
            return Err(e)
        }
        self.cycle += 1;
        Ok(())
    }

    pub fn cycle(&self) -> u64 {
        self.cycle
    }
    pub fn program_counter(&self) -> u64 {
        self.pc
    }
    pub fn translate_address_debug(&self, addr: u64) -> Option<TranslatedAddr> {
        let addr = if self.is_64_bit_mode() { addr } else { sext_32(addr as u32) };
        match self.cop0.mode() {
            Mode::Kernel => self.translate_static_kernel_addr(addr).ok().flatten(),
            Mode::Supervisor => None,
            Mode::User => None,
        }
    }
    pub fn is_big_endian(&self) -> bool {
        self.cop0.is_big_endian()
    }

    fn fetch(&mut self, bus: &mut impl SysAd) -> MipsResult<Instr> {
        if self.pc % 4 != 0 {
            return Err(Some(MipsErr::new("handling of unaligned instruction fetches is not implemented")));
        }

        let phys = self.translate_address(self.pc)?;
        if phys.cached {
            return Err(Some(MipsErr::new("handling of cached instruction fetches is not implemented")));
        }
        else {
            let word = bus.read_word(phys.addr)?;
            let val = word.to_u32(self.cop0.is_big_endian());
            Ok(Instr(val))
        }
    }

    fn translate_address(&mut self, addr: u64) -> MipsResult<TranslatedAddr> {
        let addr = if self.is_64_bit_mode() { addr } else { sext_32(addr as u32) };

        match self.cop0.mode() {
            Mode::Kernel => self.translate_kernel_addr(addr),
            Mode::Supervisor => Err(Some(MipsErr::new("translating supervisor addresses is unimplemented"))),
            Mode::User => Err(Some(MipsErr::new("translating user addresses is unimplemented"))),
        }
    }
    fn translate_kernel_addr(&mut self, addr: u64) -> MipsResult<TranslatedAddr> {
        if let Some(phys) = self.translate_static_kernel_addr(addr)? {
            Ok(phys)
        }
        else {
            Err(Some(MipsErr::new("translating TLB mapped kernel addresses is unimplemented")))
        }
    }
    fn translate_static_kernel_addr(&self, addr: u64) -> MipsResult<Option<TranslatedAddr>> {
        if XKPHYS.contains(&addr) {
            Err(Some(MipsErr::new("translating XKPHYS addresses is unimplemented")))
        }
        else if CKSEG0.contains(&addr) {
            let addr = (addr - CKSEG0.start()) as u32;
            let cached = self.cop0.is_ksg0_cached();
            Ok(Some(TranslatedAddr {
                addr,
                cached,
            }))
        }
        else if CKSEG1.contains(&addr) {
            let addr = (addr - CKSEG1.start()) as u32;
            Ok(Some(TranslatedAddr {
                addr,
                cached: false,
            }))
        }
        else {
            Ok(None)
        }
    }

    fn mem_addr(&mut self, instr: Instr) -> MipsResult<TranslatedAddr> {
        let base = self.get_reg_unatural(instr.base())?;
        let offset = instr.immi() as i64;
        let virt = base.wrapping_add_signed(offset);
        self.translate_address(virt)
    }

    fn do_mtc0(&mut self, instr: Instr) -> MipsResult<()> {
        let data = self.get_reg_u32(instr.rt())?;
        match instr.rd().0 {
            12 => self.cop0.status = Status::from_bits(data),
            16 => self.cop0.config = Config::from_bits(data),
            31.. => unreachable!(),
            or => return Err(Some(MipsErr::new(format!("Writing to CP0 register {or} is not implemented")))),
        }
        Ok(())
    }

}
impl RawCore for Vr4300 {
    fn do_bczf(&mut self, instr: Instr, likely: bool, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_bczt(&mut self, instr: Instr, likely: bool, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_break(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_cfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_copz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ctcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_dmfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_dmtcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_j(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_jal(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_jarl(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_jr(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_mfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_mtcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        match cop {
            0 => self.do_mtc0(instr),
            1 => Err(Some(MipsErr::new(format!("MTC1 is not implemented")))),
            2 => Err(Some(MipsErr::new(format!("MTC2 is not implemented")))),
            _ => unreachable!(),
        }
    }

    fn do_sync(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_syscall(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_teq(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_teqi(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tge(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tgei(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tgeiu(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tgeu(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tlt(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tlti(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tltiu(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tltu(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tne(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_tnei(&mut self, instr: Instr) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_branch_and_link(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_branch(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn get_reg_i64(&self, reg: crate::instruction::Reg) -> MipsResult<i64> {
        let reg = reg.0 as usize;
        if reg == 0 {
            Ok(0)
        } else {
            Ok(self.gp[reg - 1])
        }
    }

    fn set_reg_i64(&mut self, reg: crate::instruction::Reg, to: i64) -> MipsResult<()> {
        let reg = reg.0 as usize;
        if reg != 0 {
            self.gp[reg - 1] = to;
        }
        Ok(())
    }

    fn set_lo_i64(&mut self, to: i64) -> MipsResult<()> {
        self.lohi[0] = to;
        Ok(())
    }

    fn set_hi_i64(&mut self, to: i64) -> MipsResult<()> {
        self.lohi[1] = to;
        Ok(())
    }

    fn get_lo_natural(&mut self) -> MipsResult<i64> {
        if self.is_64_bit_mode() {
            Ok(self.lohi[0])
        }
        else {
            Ok(self.lohi[0] as i32 as i64)
        }
    }

    fn get_hi_natural(&mut self) -> MipsResult<i64> {
        if self.is_64_bit_mode() {
            Ok(self.lohi[1])
        }
        else {
            Ok(self.lohi[1] as i32 as i64)
        }
    }

    fn is_64_bit_mode(&self) -> bool {
        self.cop0.is_64_bit_mode()
    }

    fn reserved_instruction(&mut self, instr: Instr) -> MipsResult<()> {
        Err(Some(MipsErr::new(format!("handling of reserved instr {:x} is unimplemented", instr.0))))
    }

    fn integer_overflow(&mut self, instr: Instr) -> MipsResult<()> {
        Err(Some(MipsErr::new(format!("handling of integer overflow in instruction {:x} is unimplemented", instr.0))))
    }

    fn dword_operation(&mut self, instr: Instr) -> MipsResult<()> {
        if self.is_64_bit_mode() || self.cop0.mode() == Mode::Kernel {
            Ok(())
        }
        else {
            Err(Some(MipsErr::new(format!("handling of disallowed dword instruction {:x} is unimplemented", instr.0))))
        }
    }
}
impl<T: SysAd> MipsCore<T> for Vr4300 {
    fn do_lb(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lbu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ld(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ldcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ldl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ldr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lh(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lhu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_ll(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lld(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lw(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        let phys = self.mem_addr(instr)?;
        if phys.cached {
            return Err(Some(MipsErr::new("cached data accesses are not implemented")));
        }
        if phys.addr % 4 != 0 {
            return Err(Some(MipsErr::new("handling of unaligned address exceptions is not implemented")));
        }

        let word = bus.read_word(phys.addr)?;
        let be = self.is_big_endian();
        let value = word.to_u32(be);

        self.set_reg_u32(instr.rt(), value)?;

        Ok(())
    }

    fn do_lwcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lwl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lwr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_lwu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sb(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sc(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_scd(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sd(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sdcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sdl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sdr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sh(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_sw(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_swcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_swl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }

    fn do_swr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        return Err(Some(MipsErr::new("unimplemented")))
    }
}

pub trait SysAd {
    fn read_word(&mut self, addr: u32) -> Result<Word, MipsErr>;
    fn write_word(
        &mut self,
        addr: u32,
        size: WriteSize,
        data: Word,
    ) -> Result<(), MipsErr>;
    fn write_dword(&mut self, addr: u32, data: [Word; 2]) -> Result<(), MipsErr>;

    fn read_cached_data(&mut self, addr: u32) -> Result<[Word; 4], MipsErr>;
    fn read_cached_inst(&mut self, addr: u32) -> Result<[Word; 8], MipsErr>;
    fn write_cached_data(&mut self, addr: u32, data: [Word; 4]) -> Result<(), MipsErr>;
}


pub enum WriteSize {
    One,
    Two,
    Three,
    Four,
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

pub struct TranslatedAddr {
    pub addr: u32,
    pub cached: bool,
}
