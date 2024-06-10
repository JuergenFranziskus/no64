use std::{backtrace::Backtrace, error::Error, fmt::Display};

use util::sext_16;

use crate::instruction::{
    Instr, Reg, OP_ADDI, OP_ADDIU, OP_ANDI, OP_BEQ, OP_BEQL, OP_BGTZ, OP_BGTZL, OP_BLEZ, OP_BLEZL,
    OP_BNE, OP_BNEL, OP_COP0, OP_COP1, OP_COP2, OP_COP_BC, OP_COP_BC_BCF, OP_COP_BC_BCFL,
    OP_COP_BC_BCT, OP_COP_BC_BCTL, OP_COP_CF, OP_COP_CT, OP_COP_DMF, OP_COP_DMT, OP_COP_MF,
    OP_COP_MT, OP_DADDI, OP_DADDIU, OP_IR_BGEZ, OP_IR_BGEZAL, OP_IR_BGEZALL, OP_IR_BGEZL,
    OP_IR_BLTZ, OP_IR_BLTZAL, OP_IR_BLTZALL, OP_IR_BLTZL, OP_IR_TEQI, OP_IR_TGEI, OP_IR_TGEIU,
    OP_IR_TLTI, OP_IR_TLTIU, OP_IR_TNEI, OP_J, OP_JAL, OP_LB, OP_LBU, OP_LD, OP_LDC1, OP_LDC2,
    OP_LDL, OP_LDR, OP_LH, OP_LHU, OP_LL, OP_LLD, OP_LUI, OP_LW, OP_LWC1, OP_LWC2, OP_LWL, OP_LWR,
    OP_LWU, OP_ORI, OP_REGIMM, OP_SB, OP_SC, OP_SCD, OP_SD, OP_SDC1, OP_SDC2, OP_SDL, OP_SDR,
    OP_SH, OP_SLTI, OP_SLTIU, OP_SPECIAL, OP_SP_ADD, OP_SP_ADDU, OP_SP_AND, OP_SP_BREAK,
    OP_SP_DADD, OP_SP_DADDU, OP_SP_DDIV, OP_SP_DDIVU, OP_SP_DIV, OP_SP_DIVU, OP_SP_DMULT,
    OP_SP_DMULTU, OP_SP_DSLL, OP_SP_DSLL32, OP_SP_DSLLV, OP_SP_DSRA, OP_SP_DSRA32, OP_SP_DSRAV,
    OP_SP_DSRL, OP_SP_DSRL32, OP_SP_DSRLV, OP_SP_DSUB, OP_SP_DSUBU, OP_SP_JALR, OP_SP_JR,
    OP_SP_MFHI, OP_SP_MFLO, OP_SP_MTHI, OP_SP_MTLO, OP_SP_MULT, OP_SP_MULTU, OP_SP_NOR, OP_SP_OR,
    OP_SP_SLL, OP_SP_SLLV, OP_SP_SLT, OP_SP_SLTU, OP_SP_SRAV, OP_SP_SRL, OP_SP_SRLV, OP_SP_SUB,
    OP_SP_SUBU, OP_SP_SYNC, OP_SP_SYSCALL, OP_SP_TEQ, OP_SP_TGE, OP_SP_TGEU, OP_SP_TLT, OP_SP_TLTU,
    OP_SP_TNE, OP_SP_XOR, OP_SW, OP_SWC1, OP_SWC2, OP_SWL, OP_SWR, OP_XORI,
};

pub trait RawCore {
    fn do_special(&mut self, instr: Instr) -> MipsResult<()> {
        match instr.funct() {
            OP_SP_SLL => self.do_sll(instr)?,
            OP_SP_SRL => self.do_srl(instr)?,
            OP_SP_SLLV => self.do_sllv(instr)?,
            OP_SP_SRLV => self.do_srlv(instr)?,
            OP_SP_SRAV => self.do_srav(instr)?,
            OP_SP_JR => self.do_jr(instr)?,
            OP_SP_JALR => self.do_jarl(instr)?,
            OP_SP_SYSCALL => self.do_syscall(instr)?,
            OP_SP_BREAK => self.do_break(instr)?,
            OP_SP_SYNC => self.do_sync(instr)?,
            OP_SP_MFHI => self.do_mfhi(instr)?,
            OP_SP_MTHI => self.do_mthi(instr)?,
            OP_SP_MFLO => self.do_mflo(instr)?,
            OP_SP_MTLO => self.do_mtlo(instr)?,
            OP_SP_DSLLV => self.do_dsllv(instr)?,
            OP_SP_DSRLV => self.do_dsrlv(instr)?,
            OP_SP_DSRAV => self.do_dsrav(instr)?,
            OP_SP_MULT => self.do_mult(instr)?,
            OP_SP_MULTU => self.do_multu(instr)?,
            OP_SP_DIV => self.do_div(instr)?,
            OP_SP_DIVU => self.do_divu(instr)?,
            OP_SP_DMULT => self.do_dmult(instr)?,
            OP_SP_DMULTU => self.do_dmultu(instr)?,
            OP_SP_DDIV => self.do_ddiv(instr)?,
            OP_SP_DDIVU => self.do_ddivu(instr)?,
            OP_SP_ADD => self.do_add(instr)?,
            OP_SP_ADDU => self.do_addu(instr)?,
            OP_SP_SUB => self.do_sub(instr)?,
            OP_SP_SUBU => self.do_subu(instr)?,
            OP_SP_AND => self.do_and(instr)?,
            OP_SP_OR => self.do_or(instr)?,
            OP_SP_XOR => self.do_xor(instr)?,
            OP_SP_NOR => self.do_nor(instr)?,
            OP_SP_SLT => self.do_slt(instr)?,
            OP_SP_SLTU => self.do_sltu(instr)?,
            OP_SP_DADD => self.do_dadd(instr)?,
            OP_SP_DADDU => self.do_daddu(instr)?,
            OP_SP_DSUB => self.do_dsub(instr)?,
            OP_SP_DSUBU => self.do_dsubu(instr)?,
            OP_SP_TGE => self.do_tge(instr)?,
            OP_SP_TGEU => self.do_tgeu(instr)?,
            OP_SP_TLT => self.do_tlt(instr)?,
            OP_SP_TLTU => self.do_tltu(instr)?,
            OP_SP_TEQ => self.do_teq(instr)?,
            OP_SP_TNE => self.do_tne(instr)?,
            OP_SP_DSLL => self.do_dsll(instr)?,
            OP_SP_DSRL => self.do_dsrl(instr)?,
            OP_SP_DSRA => self.do_dsra(instr)?,
            OP_SP_DSLL32 => self.do_dsll32(instr)?,
            OP_SP_DSRL32 => self.do_dsrl32(instr)?,
            OP_SP_DSRA32 => self.do_dsra32(instr)?,
            _ => self.reserved_instruction(instr)?,
        }

        Ok(())
    }
    fn do_regimm(&mut self, instr: Instr) -> MipsResult<()> {
        match instr.rt().0 {
            OP_IR_BLTZ => self.do_bltz(instr, false)?,
            OP_IR_BGEZ => self.do_bgez(instr, false)?,
            OP_IR_BLTZL => self.do_bltz(instr, true)?,
            OP_IR_BGEZL => self.do_bgez(instr, true)?,
            OP_IR_TGEI => self.do_tgei(instr)?,
            OP_IR_TGEIU => self.do_tgeiu(instr)?,
            OP_IR_TLTI => self.do_tlti(instr)?,
            OP_IR_TLTIU => self.do_tltiu(instr)?,
            OP_IR_TEQI => self.do_teqi(instr)?,
            OP_IR_TNEI => self.do_tnei(instr)?,
            OP_IR_BLTZAL => self.do_bltzal(instr, false)?,
            OP_IR_BGEZAL => self.do_bgezal(instr, false)?,
            OP_IR_BLTZALL => self.do_bltzal(instr, true)?,
            OP_IR_BGEZALL => self.do_bgezal(instr, true)?,
            _ => self.reserved_instruction(instr)?,
        }

        Ok(())
    }
    fn do_cop(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        match instr.rs().0 {
            OP_COP_MF => self.do_mfcz(instr, cop)?,
            OP_COP_DMF => self.do_dmfcz(instr, cop)?,
            OP_COP_CF => self.do_cfcz(instr, cop)?,
            OP_COP_MT => self.do_mtcz(instr, cop)?,
            OP_COP_DMT => self.do_dmtcz(instr, cop)?,
            OP_COP_CT => self.do_ctcz(instr, cop)?,
            OP_COP_BC => self.do_cop_bc(instr, cop)?,
            0o20.. => self.do_copz(instr, cop)?,
            _ => self.reserved_instruction(instr)?,
        }
        Ok(())
    }
    fn do_cop_bc(&mut self, instr: Instr, cop: u8) -> MipsResult<()> {
        match instr.rt().0 {
            OP_COP_BC_BCF => self.do_bczf(instr, false, cop)?,
            OP_COP_BC_BCT => self.do_bczt(instr, false, cop)?,
            OP_COP_BC_BCFL => self.do_bczf(instr, true, cop)?,
            OP_COP_BC_BCTL => self.do_bczt(instr, true, cop)?,
            _ => self.reserved_instruction(instr)?,
        }
        Ok(())
    }

    fn do_add(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_i32(instr.rs())?;
        let rt = self.get_reg_i32(instr.rt())?;
        let (sum, over) = rs.overflowing_add(rt);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i32(instr.rd(), sum)?;
        Ok(())
    }
    fn do_addi(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_i32(instr.rs())?;
        let imm = instr.immi();
        let (sum, over) = rs.overflowing_add(imm as i32);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i32(instr.rt(), sum)?;
        Ok(())
    }
    fn do_addiu(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_i32(instr.rs())?;
        let imm = instr.immi();
        let sum = rs.wrapping_add(imm as i32);
        self.set_reg_i32(instr.rt(), sum)?;
        Ok(())
    }
    fn do_addu(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_i32(instr.rs())?;
        let rt = self.get_reg_i32(instr.rt())?;
        let sum = rs.wrapping_add(rt);
        self.set_reg_i32(instr.rd(), sum)?;
        Ok(())
    }
    fn do_and(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        let and = rs & rt;
        self.set_reg_inatural(instr.rd(), and)?;

        Ok(())
    }
    fn do_andi(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let imm = instr.immi() as i64;
        let and = rs & imm;
        self.set_reg_inatural(instr.rt(), and)?;

        Ok(())
    }
    fn do_bczf(&mut self, instr: Instr, likely: bool, cop: u8) -> MipsResult<()>;
    fn do_bczt(&mut self, instr: Instr, likely: bool, cop: u8) -> MipsResult<()>;
    fn do_beq(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        if rs == rt {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_bgez(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs >= 0 {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_bgezal(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs >= 0 {
            self.do_branch_and_link(instr, likely)?;
        }

        Ok(())
    }
    fn do_bgtz(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs > 0 {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_blez(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs <= 0 {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_bltz(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs < 0 {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_bltzal(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        if rs < 0 {
            self.do_branch_and_link(instr, likely)?;
        }

        Ok(())
    }
    fn do_bne(&mut self, instr: Instr, likely: bool) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        if rs != rt {
            self.do_branch(instr, likely)?;
        }

        Ok(())
    }
    fn do_break(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_cfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_copz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_ctcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_dadd(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let rt = self.get_reg_i64(instr.rt())?;
        let (sum, over) = rs.overflowing_add(rt);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i64(instr.rd(), sum)?;
        Ok(())
    }
    fn do_daddi(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let imm = instr.immi();
        let (sum, over) = rs.overflowing_add(imm as i64);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i64(instr.rt(), sum)?;
        Ok(())
    }
    fn do_daddiu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let imm = instr.immi();
        let sum = rs.wrapping_add(imm as i64);
        self.set_reg_i64(instr.rt(), sum)?;
        Ok(())
    }
    fn do_daddu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sum = rs.wrapping_add(rt);
        self.set_reg_i64(instr.rd(), sum)?;
        Ok(())
    }
    fn do_ddiv(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let rt = self.get_reg_i64(instr.rt())?;
        if rt == 0 {
            return Ok(());
        };
        let q = rs.wrapping_div(rt);
        let r = rs.wrapping_rem(rt);
        self.set_lo_i64(q)?;
        self.set_hi_i64(r)?;

        Ok(())
    }
    fn do_ddivu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_u64(instr.rs())?;
        let rt = self.get_reg_u64(instr.rt())?;
        if rt == 0 {
            return Ok(());
        };
        let q = rs.wrapping_div(rt);
        let r = rs.wrapping_rem(rt);
        self.set_lo_u64(q)?;
        self.set_hi_u64(r)?;

        Ok(())
    }
    fn do_div(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i32(instr.rs())?;
        let rt = self.get_reg_i32(instr.rt())?;
        if rt == 0 {
            return Ok(());
        };
        let q = rs.wrapping_div(rt);
        let r = rs.wrapping_rem(rt);
        self.set_lo_i32(q)?;
        self.set_hi_i32(r)?;

        Ok(())
    }
    fn do_divu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_u32(instr.rs())?;
        let rt = self.get_reg_u32(instr.rt())?;
        if rt == 0 {
            return Ok(());
        };
        let q = rs.wrapping_div(rt);
        let r = rs.wrapping_rem(rt);
        self.set_lo_u32(q)?;
        self.set_hi_u32(r)?;

        Ok(())
    }
    fn do_dmfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_dmtcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_dmult(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())? as i128;
        let rt = self.get_reg_i64(instr.rt())? as i128;
        let p = rs.wrapping_mul(rt);
        let lo = p as i64;
        let hi = (p >> 64) as i64;
        self.set_lo_i64(lo)?;
        self.set_hi_i64(hi)?;

        Ok(())
    }
    fn do_dmultu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_u64(instr.rs())? as u128;
        let rt = self.get_reg_u64(instr.rt())? as u128;
        let p = rs.wrapping_mul(rt);
        let lo = p as u64;
        let hi = (p >> 64) as u64;
        self.set_lo_u64(lo)?;
        self.set_hi_u64(hi)?;

        Ok(())
    }
    fn do_dsll(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt << sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsllv(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x3F;
        let shifted = rt << sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsll32(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = instr.sa() as u32 + 32;
        let shifted = rt << sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsra(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt >> sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsrav(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x3F;
        let shifted = rt >> sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsra32(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sa = instr.sa() as u32 + 32;
        let shifted = rt >> sa;
        self.set_reg_i64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsrl(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_u64(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt >> sa;
        self.set_reg_u64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsrlv(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_u64(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x3F;
        let shifted = rt >> sa;
        self.set_reg_u64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsrl32(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rt = self.get_reg_u64(instr.rt())?;
        let sa = instr.sa() as u32 + 32;
        let shifted = rt >> sa;
        self.set_reg_u64(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_dsub(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let rt = self.get_reg_i64(instr.rt())?;
        let (sum, over) = rs.overflowing_sub(rt);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i64(instr.rd(), sum)?;
        Ok(())
    }
    fn do_dsubu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i64(instr.rs())?;
        let rt = self.get_reg_i64(instr.rt())?;
        let sum = rs.wrapping_sub(rt);
        self.set_reg_i64(instr.rd(), sum)?;
        Ok(())
    }
    fn do_j(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_jal(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_jarl(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_jr(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_lui(&mut self, instr: Instr) -> MipsResult<()> {
        let imm = instr.immi() as i32;
        self.set_reg_i32(instr.rt(), imm << 16)?;
        Ok(())
    }
    fn do_mfcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_mfhi(&mut self, instr: Instr) -> MipsResult<()> {
        let hi = self.get_hi_natural()?;
        self.set_reg_inatural(instr.rd(), hi)?;
        Ok(())
    }
    fn do_mflo(&mut self, instr: Instr) -> MipsResult<()> {
        let lo = self.get_lo_natural()?;
        self.set_reg_inatural(instr.rd(), lo)?;
        Ok(())
    }
    fn do_mtcz(&mut self, instr: Instr, cop: u8) -> MipsResult<()>;
    fn do_mthi(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        self.set_hi_natural(rs)?;
        Ok(())
    }
    fn do_mtlo(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        self.set_lo_natural(rs)?;
        Ok(())
    }
    fn do_mult(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i32(instr.rs())? as i64;
        let rt = self.get_reg_i32(instr.rt())? as i64;
        let p = rs.wrapping_mul(rt);
        let lo = p as i32;
        let hi = (p >> 32) as i32;
        self.set_lo_i32(lo)?;
        self.set_hi_i32(hi)?;

        Ok(())
    }
    fn do_multu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_u32(instr.rs())? as u64;
        let rt = self.get_reg_u32(instr.rt())? as u64;
        let p = rs.wrapping_mul(rt);
        let lo = p as u32;
        let hi = (p >> 32) as u32;
        self.set_lo_u32(lo)?;
        self.set_hi_u32(hi)?;

        Ok(())
    }
    fn do_nor(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        let nor = !(rs | rt);
        self.set_reg_inatural(instr.rd(), nor)?;

        Ok(())
    }
    fn do_or(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        let or = rs | rt;
        self.set_reg_inatural(instr.rd(), or)?;

        Ok(())
    }
    fn do_ori(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let imm = instr.immi() as i64;
        let or = rs | imm;
        self.set_reg_inatural(instr.rt(), or)?;

        Ok(())
    }
    fn do_sll(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_i32(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt << sa;
        self.set_reg_i32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_sllv(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_i32(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x1F;
        let shifted = rt << sa;
        self.set_reg_i32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_slt(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        let lt = rs < rt;
        let lt = if lt { 1 } else { 0 };
        self.set_reg_inatural(instr.rd(), lt)?;
        Ok(())
    }
    fn do_slti(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let imm = instr.immi() as i64;
        let lt = rs < imm;
        let lt = if lt { 1 } else { 0 };
        self.set_reg_inatural(instr.rd(), lt)?;
        Ok(())
    }
    fn do_sltiu(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_unatural(instr.rs())?;
        let imm = sext_16(instr.immu());
        let lt = rs < imm;
        let lt = if lt { 1 } else { 0 };
        self.set_reg_inatural(instr.rd(), lt)?;
        Ok(())
    }
    fn do_sltu(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_unatural(instr.rs())?;
        let rt = self.get_reg_unatural(instr.rt())?;
        let lt = rs < rt;
        let lt = if lt { 1 } else { 0 };
        self.set_reg_inatural(instr.rd(), lt)?;
        Ok(())
    }
    fn do_sra(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_i32(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt >> sa;
        self.set_reg_i32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_srav(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_i32(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x1F;
        let shifted = rt >> sa;
        self.set_reg_i32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_srl(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_u32(instr.rt())?;
        let sa = instr.sa() as u32;
        let shifted = rt >> sa;
        self.set_reg_u32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_srlv(&mut self, instr: Instr) -> MipsResult<()> {
        let rt = self.get_reg_u32(instr.rt())?;
        let sa = self.get_reg_u32(instr.rs())? & 0x3F;
        let shifted = rt >> sa;
        self.set_reg_u32(instr.rd(), shifted)?;

        Ok(())
    }
    fn do_sub(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i32(instr.rs())?;
        let rt = self.get_reg_i32(instr.rt())?;
        let (sum, over) = rs.overflowing_sub(rt);
        if over {
            self.integer_overflow(instr)?;
        }
        self.set_reg_i32(instr.rd(), sum)?;
        Ok(())
    }
    fn do_subu(&mut self, instr: Instr) -> MipsResult<()> {
        self.dword_operation(instr)?;
        let rs = self.get_reg_i32(instr.rs())?;
        let rt = self.get_reg_i32(instr.rt())?;
        let sum = rs.wrapping_sub(rt);
        self.set_reg_i32(instr.rd(), sum)?;
        Ok(())
    }
    fn do_sync(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_syscall(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_teq(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_teqi(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tge(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tgei(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tgeiu(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tgeu(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tlt(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tlti(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tltiu(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tltu(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tne(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_tnei(&mut self, instr: Instr) -> MipsResult<()>;
    fn do_xor(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let rt = self.get_reg_inatural(instr.rt())?;
        let or = rs ^ rt;
        self.set_reg_inatural(instr.rd(), or)?;

        Ok(())
    }
    fn do_xori(&mut self, instr: Instr) -> MipsResult<()> {
        let rs = self.get_reg_inatural(instr.rs())?;
        let imm = instr.immi() as i64;
        let or = rs ^ imm;
        self.set_reg_inatural(instr.rt(), or)?;

        Ok(())
    }

    fn do_branch_and_link(&mut self, instr: Instr, likely: bool) -> MipsResult<()>;
    fn do_branch(&mut self, instr: Instr, likely: bool) -> MipsResult<()>;

    fn get_reg_unatural(&self, reg: Reg) -> MipsResult<u64> {
        if self.is_64_bit_mode() {
            self.get_reg_u64(reg)
        } else {
            self.get_reg_u32(reg).map(Into::into)
        }
    }
    fn get_reg_u64(&self, reg: Reg) -> MipsResult<u64> {
        self.get_reg_i64(reg).map(|i| i as u64)
    }
    fn get_reg_u32(&self, reg: Reg) -> MipsResult<u32> {
        self.get_reg_i32(reg).map(|v| v as u32)
    }
    fn get_reg_inatural(&self, reg: Reg) -> MipsResult<i64> {
        if self.is_64_bit_mode() {
            self.get_reg_i64(reg)
        } else {
            self.get_reg_i32(reg).map(Into::into)
        }
    }
    fn get_reg_i64(&self, reg: Reg) -> MipsResult<i64>;
    fn get_reg_i32(&self, reg: Reg) -> MipsResult<i32> {
        self.get_reg_i64(reg).map(|v| v as i32)
    }

    fn set_reg_u64(&mut self, reg: Reg, to: u64) -> MipsResult<()> {
        self.set_reg_i64(reg, to as i64)
    }
    fn set_reg_u32(&mut self, reg: Reg, to: u32) -> MipsResult<()> {
        self.set_reg_i32(reg, to as i32)
    }
    fn set_reg_inatural(&mut self, reg: Reg, to: i64) -> MipsResult<()> {
        if self.is_64_bit_mode() {
            self.set_reg_i64(reg, to)
        } else {
            self.set_reg_i32(reg, to as i32)
        }
    }
    fn set_reg_i64(&mut self, reg: Reg, to: i64) -> MipsResult<()>;
    fn set_reg_i32(&mut self, reg: Reg, to: i32) -> MipsResult<()> {
        self.set_reg_i64(reg, to as i64)
    }

    fn set_lo_natural(&mut self, to: i64) -> MipsResult<()> {
        if self.is_64_bit_mode() {
            self.set_lo_i64(to)
        } else {
            self.set_lo_i32(to as i32)
        }
    }
    fn set_lo_u64(&mut self, to: u64) -> MipsResult<()> {
        self.set_hi_i64(to as i64)
    }
    fn set_lo_u32(&mut self, to: u32) -> MipsResult<()> {
        self.set_lo_i32(to as i32)
    }
    fn set_lo_i64(&mut self, to: i64) -> MipsResult<()>;
    fn set_lo_i32(&mut self, to: i32) -> MipsResult<()> {
        self.set_lo_i64(to as i64)
    }
    fn set_hi_natural(&mut self, to: i64) -> MipsResult<()> {
        if self.is_64_bit_mode() {
            self.set_hi_i64(to)
        } else {
            self.set_hi_i32(to as i32)
        }
    }
    fn set_hi_u64(&mut self, to: u64) -> MipsResult<()> {
        self.set_hi_i64(to as i64)
    }
    fn set_hi_u32(&mut self, to: u32) -> MipsResult<()> {
        self.set_hi_i32(to as i32)
    }
    fn set_hi_i64(&mut self, to: i64) -> MipsResult<()>;
    fn set_hi_i32(&mut self, to: i32) -> MipsResult<()> {
        self.set_hi_i64(to as i64)
    }

    fn get_lo_natural(&mut self) -> MipsResult<i64>;
    fn get_hi_natural(&mut self) -> MipsResult<i64>;

    fn is_64_bit_mode(&self) -> bool;

    fn reserved_instruction(&mut self, instr: Instr) -> MipsResult<()>;
    fn integer_overflow(&mut self, instr: Instr) -> MipsResult<()>;
    fn dword_operation(&mut self, instr: Instr) -> MipsResult<()>;
}

pub trait MipsCore<T>: RawCore {
    fn do_instruction(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()> {
        match instr.opcode() {
            OP_SPECIAL => self.do_special(instr)?,
            OP_REGIMM => self.do_regimm(instr)?,
            OP_J => self.do_j(instr)?,
            OP_JAL => self.do_jal(instr)?,
            OP_BEQ => self.do_beq(instr, false)?,
            OP_BNE => self.do_bne(instr, false)?,
            OP_BLEZ => self.do_blez(instr, false)?,
            OP_BGTZ => self.do_bgtz(instr, false)?,
            OP_ADDI => self.do_addi(instr)?,
            OP_ADDIU => self.do_addiu(instr)?,
            OP_SLTI => self.do_slti(instr)?,
            OP_SLTIU => self.do_sltiu(instr)?,
            OP_ANDI => self.do_andi(instr)?,
            OP_ORI => self.do_ori(instr)?,
            OP_XORI => self.do_xori(instr)?,
            OP_LUI => self.do_lui(instr)?,
            OP_COP0 => self.do_cop(instr, 0)?,
            OP_COP1 => self.do_cop(instr, 1)?,
            OP_COP2 => self.do_cop(instr, 2)?,
            OP_BEQL => self.do_beq(instr, true)?,
            OP_BNEL => self.do_bne(instr, true)?,
            OP_BLEZL => self.do_blez(instr, true)?,
            OP_BGTZL => self.do_bgtz(instr, true)?,
            OP_DADDI => self.do_daddi(instr)?,
            OP_DADDIU => self.do_daddiu(instr)?,
            OP_LDL => self.do_ldl(instr, bus)?,
            OP_LDR => self.do_ldr(instr, bus)?,
            OP_LB => self.do_lb(instr, bus)?,
            OP_LH => self.do_lh(instr, bus)?,
            OP_LWL => self.do_lwl(instr, bus)?,
            OP_LW => self.do_lw(instr, bus)?,
            OP_LBU => self.do_lbu(instr, bus)?,
            OP_LHU => self.do_lhu(instr, bus)?,
            OP_LWR => self.do_lwr(instr, bus)?,
            OP_LWU => self.do_lwu(instr, bus)?,
            OP_SB => self.do_sb(instr, bus)?,
            OP_SH => self.do_sh(instr, bus)?,
            OP_SWL => self.do_swl(instr, bus)?,
            OP_SW => self.do_sw(instr, bus)?,
            OP_SDL => self.do_sdl(instr, bus)?,
            OP_SDR => self.do_sdr(instr, bus)?,
            OP_SWR => self.do_swr(instr, bus)?,
            OP_LL => self.do_ll(instr, bus)?,
            OP_LWC1 => self.do_lwcz(instr, 1, bus)?,
            OP_LWC2 => self.do_lwcz(instr, 2, bus)?,
            OP_LLD => self.do_lld(instr, bus)?,
            OP_LDC1 => self.do_ldcz(instr, 1, bus)?,
            OP_LDC2 => self.do_ldcz(instr, 2, bus)?,
            OP_LD => self.do_ld(instr, bus)?,
            OP_SC => self.do_sc(instr, bus)?,
            OP_SWC1 => self.do_swcz(instr, 1, bus)?,
            OP_SWC2 => self.do_swcz(instr, 2, bus)?,
            OP_SCD => self.do_scd(instr, bus)?,
            OP_SDC1 => self.do_sdcz(instr, 1, bus)?,
            OP_SDC2 => self.do_sdcz(instr, 2, bus)?,
            OP_SD => self.do_sd(instr, bus)?,
            _ => self.reserved_instruction(instr)?,
        }

        Ok(())
    }

    fn do_lb(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lbu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_ld(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_ldcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()>;
    fn do_ldl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_ldr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lh(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lhu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_ll(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lld(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lw(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lwcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()>;
    fn do_lwl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lwr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_lwu(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sb(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sc(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_scd(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sd(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sdcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()>;
    fn do_sdl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sdr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sh(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_sw(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_swcz(&mut self, instr: Instr, cop: u8, bus: &mut T) -> MipsResult<()>;
    fn do_swl(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
    fn do_swr(&mut self, instr: Instr, bus: &mut T) -> MipsResult<()>;
}

/// The result of stepping a MipsCore based emulator forward.
/// A distinction needs to be made between emulation failure due to incomplete/incorrect implementation, and correctly emulated behaviour
/// that procludes returning the expected result, like the raising of a processor exception.
/// Returning an Ok instance of this type signifies execution along the regular happy path of the processor;
/// emulation was correct and successfull, and no exception or other early-terminating mechanism was encountered.
/// Returning an Err(None) instance of this type signifies that emulation was correct and successfull, but an exception or other early-terminating
/// mechanism was encountered; emulation can proceed as usual, but the currently executing instruction can not and should not be completed.
/// Returning an Err(Some) instance of this type signifies that emulation failed.
pub type MipsResult<R> = Result<R, Option<MipsErr>>;

#[derive(Debug)]
pub struct MipsErr {
    at: Backtrace,
    message: Box<str>,
}
impl MipsErr {
    pub fn new(message: impl Into<Box<str>>) -> Self {
        Self {
            at: Backtrace::capture(),
            message: message.into(),
        }
    }

    pub fn at(&self) -> &Backtrace {
        &self.at
    }
    pub fn message(&self) -> &str {
        &self.message
    }
}
impl Display for MipsErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for MipsErr {
    
}
