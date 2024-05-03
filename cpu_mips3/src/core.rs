//! The MIPS specification only describes the user-level instruction set.
//! Kernel mode, memory management and exception handling are left up to the implementation.

use util::{sext_16, sext_32};

use crate::instruction::Instr;

pub trait MipsCore<T> {
    fn do_instr(&mut self, instr: Instr, bus: &mut T) -> Happy<()> {
        match instr.opcode() {
            Instr::ADDIU => self.do_addiu(instr)?,
            Instr::ANDI => self.do_andi(instr)?,
            Instr::ORI => self.do_ori(instr)?,
            Instr::LUI => self.do_lui(instr)?,
            Instr::COP0 => self.do_cop0(instr)?,
            Instr::BEQL => self.do_beql(instr)?,
            Instr::LW => self.do_lw(instr, bus)?,
            _ => unimplemented!("Execution of {instr} is not implemented"),
        }

        Ok(())
    }
    fn do_addiu(&mut self, instr: Instr) -> Happy<()> {
        let imm = sext_16(instr.immediate());
        let rs = self.get_reg(instr.rs())?;
        let sum = rs.wrapping_add(imm);
        self.set_reg(instr.rt(), sum)?;
        Ok(())
    }
    fn do_andi(&mut self, instr: Instr) -> Happy<()> {
        let imm = sext_16(instr.immediate());
        let rs = self.get_reg(instr.rs())?;
        let and = rs & imm;
        self.set_reg(instr.rt(), and)?;
        Ok(())
    }
    fn do_ori(&mut self, instr: Instr) -> Happy<()> {
        let imm = sext_16(instr.immediate());
        let rs = self.get_reg(instr.rs())?;
        let or = rs | imm;
        self.set_reg(instr.rt(), or)?;

        Ok(())
    }
    fn do_lui(&mut self, instr: Instr) -> Happy<()> {
        let imm = sext_16(instr.immediate());
        let shifted = imm << 16;
        self.set_reg(instr.rt(), shifted)?;
        Ok(())
    }
    fn do_cop0(&mut self, instr: Instr) -> Happy<()> {
        match instr.rs() {
            Instr::COPZ_MT => self.do_mtc0(instr)?,
            _ => panic!(),
        }

        Ok(())
    }
    fn do_beql(&mut self, instr: Instr) -> Happy<()> {
        let offset = sext_16(instr.immediate()) << 2;
        let tgt = self.program_counter().wrapping_add(offset);
        let rs = self.get_reg(instr.rs());
        let rt = self.get_reg(instr.rt());
        let c = rs == rt;
        self.do_branch(c, true, tgt)?;
        Ok(())
    }
    fn do_lw(&mut self, instr: Instr, bus: &mut T) -> Happy<()> {
        let offset = sext_16(instr.immediate());
        let base = self.get_reg(instr.base())?;
        let addr = base.wrapping_add(offset);
        let val = self.read_32(addr, bus)?;
        self.set_reg(instr.rt(), sext_32(val))?;
        Ok(())
    }

    fn do_mtc0(&mut self, instr: Instr) -> Happy<()>;

    fn do_branch(&mut self, c: bool, likely: bool, tgt: u64) -> Happy<()>;

    fn get_reg(&self, reg: u8) -> Happy<u64>;
    fn set_reg(&mut self, reg: u8, val: u64) -> Happy<()>;

    fn program_counter(&self) -> u64;

    fn read_8(&mut self, addr: u64, bus: &mut T) -> Happy<u8>;
    fn read_16(&mut self, addr: u64, bus: &mut T) -> Happy<u16>;
    fn read_32(&mut self, addr: u64, bus: &mut T) -> Happy<u32>;
    fn read_64(&mut self, addr: u64, bus: &mut T) -> Happy<u64>;
    fn write_8(&mut self, addr: u64, data: u8, bus: &mut T) -> Happy<()>;
    fn write_16(&mut self, addr: u64, data: u16, bus: &mut T) -> Happy<()>;
    fn write_32(&mut self, addr: u64, data: u32, bus: &mut T) -> Happy<()>;
    fn write_64(&mut self, addr: u64, data: u64, bus: &mut T) -> Happy<()>;
}

pub type Happy<T> = Result<T, ()>;
