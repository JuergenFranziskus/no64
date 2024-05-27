use util::{sext_16, sext_32};

use crate::instruction::{Instr, Reg};

pub trait RawCore {
    fn do_srl(&mut self, instr: Instr) -> Happy<()> {
        let rt = self.get_reg(instr.rt())?;
        let word = rt as u32;
        let shift = instr.sa() as u32;
        let shifted = word >> shift;
        let extended = sext_32(shifted);
        self.set_reg(instr.rd(), extended)?;
        Ok(())
    }
    fn do_jr(&mut self, instr: Instr) -> Happy<()> {
        let tgt = self.get_reg(instr.rs())?;
        self.do_branch(true, false, tgt)?;
        Ok(())
    }
    fn do_or(&mut self, instr: Instr) -> Happy<()> {
        let rs = self.get_reg(instr.rs())?;
        let rt = self.get_reg(instr.rt())?;
        let result = rs | rt;
        self.set_reg(instr.rd(), result)?;
        Ok(())
    }

    fn do_beq(&mut self, instr: Instr) -> Happy<()> {
        self.do_c_branch(|a, b| a == b, false, instr)?;
        Ok(())
    }
    fn do_bne(&mut self, instr: Instr) -> Happy<()> {
        self.do_c_branch(|a, b| a != b, false, instr)?;
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
        match instr.rs().0 {
            Instr::COPZ_MT => self.do_mtc0(instr)?,
            _ => panic!(),
        }

        Ok(())
    }
    fn do_beql(&mut self, instr: Instr) -> Happy<()> {
        self.do_c_branch(|a, b| a == b, true, instr)?;
        Ok(())
    }
    fn do_bnel(&mut self, instr: Instr) -> Happy<()> {
        self.do_c_branch(|a, b| a != b, true, instr)?;
        Ok(())
    }

    fn do_mtc0(&mut self, instr: Instr) -> Happy<()>;

    fn do_c_branch(&mut self, c: fn(u64, u64) -> bool, likely: bool, instr: Instr) -> Happy<()> {
        let offset = sext_16(instr.immediate()) << 2;
        let tgt = self.program_counter().wrapping_add(offset) + 4;
        let rs = self.get_reg(instr.rs())?;
        let rt = self.get_reg(instr.rt())?;
        let c = c(rs, rt);
        self.do_branch(c, likely, tgt)?;
        Ok(())
    }
    fn do_branch(&mut self, c: bool, likely: bool, tgt: u64) -> Happy<()>;

    fn get_reg(&self, reg: Reg) -> Happy<u64>;
    fn set_reg(&mut self, reg: Reg, val: u64) -> Happy<()>;

    fn get_load_store_addr(&self, instr: Instr) -> Happy<u64> {
        let offset = sext_16(instr.immediate());
        let base = self.get_reg(instr.base())?;
        Ok(base.wrapping_add(offset))
    }

    fn program_counter(&self) -> u64;
    fn is_64_bit_mode(&self) -> bool;
}

pub trait BusCore<T>: RawCore {
    fn do_instr(&mut self, instr: Instr, bus: &mut T) -> Happy<()> {
        match instr.opcode() {
            Instr::SPECIAL => self.do_special(instr, bus)?,
            Instr::BEQ => self.do_beq(instr)?,
            Instr::BNE => self.do_bne(instr)?,
            Instr::ADDIU => self.do_addiu(instr)?,
            Instr::ANDI => self.do_andi(instr)?,
            Instr::ORI => self.do_ori(instr)?,
            Instr::LUI => self.do_lui(instr)?,
            Instr::COP0 => self.do_cop0(instr)?,
            Instr::BEQL => self.do_beql(instr)?,
            Instr::BNEL => self.do_bnel(instr)?,
            Instr::LW => self.do_lw(instr, bus)?,
            Instr::SW => self.do_sw(instr, bus)?,
            _ => unimplemented!("Execution of {instr} is not implemented"),
        }

        Ok(())
    }
    fn do_special(&mut self, instr: Instr, _bus: &mut T) -> Happy<()> {
        match instr.funct() {
            Instr::SPECIAL_SRL => self.do_srl(instr)?,
            Instr::SPECIAL_JR => self.do_jr(instr)?,
            Instr::SPECIAL_OR => self.do_or(instr)?,
            _ => unimplemented!("Execution of {instr} is not implemented"),
        }

        Ok(())
    }

    fn do_lw(&mut self, instr: Instr, bus: &mut T) -> Happy<()> {
        let addr = self.get_load_store_addr(instr)?;
        let val = self.read_32(addr, bus)?;
        self.set_reg(instr.rt(), sext_32(val))?;
        Ok(())
    }
    fn do_sw(&mut self, instr: Instr, bus: &mut T) -> Happy<()> {
        let addr = self.get_load_store_addr(instr)?;
        let val = self.get_reg(instr.rt())?;
        self.write_32(addr, val as u32, bus)?;
        Ok(())
    }

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
