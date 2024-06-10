use std::io::{self, Write};


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instr(pub u32);
impl Instr {
    pub fn opcode(self) -> u8 {
        (self.0 >> 26) as u8
    }
    pub fn rs(self) -> Reg {
        Reg((self.0 >> 21) as u8 & 0x1F)
    }
    pub fn base(self) -> Reg {
        self.rs()
    }
    pub fn rt(self) -> Reg {
        Reg((self.0 >> 16) as u8 & 0x1F)
    }
    pub fn rd(self) -> Reg {
        Reg((self.0 >> 11) as u8 & 0x1F)
    }
    pub fn sa(self) -> u8 {
        (self.0 >> 06) as u8 & 0x1F
    }
    pub fn funct(self) -> u8 {
        self.0 as u8 & 0x3F
    }
    pub fn immu(self) -> u16 {
        self.0 as u16
    }
    pub fn immi(self) -> i16 {
        self.0 as i16
    }
    pub fn branch_offset(self) -> i16 {
        self.immi()
    }
    pub fn jump_offset(self) -> u32 {
        self.0 & 0x3FFFFFF
    }
}

pub const OP_SPECIAL: u8 = 0o00;
pub const OP_REGIMM: u8 = 0o01;
pub const OP_J: u8 = 0o02;
pub const OP_JAL: u8 = 0o03;
pub const OP_BEQ: u8 = 0o04;
pub const OP_BNE: u8 = 0o05;
pub const OP_BLEZ: u8 = 0o06;
pub const OP_BGTZ: u8 = 0o07;
pub const OP_ADDI: u8 = 0o10;
pub const OP_ADDIU: u8 = 0o11;
pub const OP_SLTI: u8 = 0o12;
pub const OP_SLTIU: u8 = 0o13;
pub const OP_ANDI: u8 = 0o14;
pub const OP_ORI: u8 = 0o15;
pub const OP_XORI: u8 = 0o16;
pub const OP_LUI: u8 = 0o17;
pub const OP_COP0: u8 = 0o20;
pub const OP_COP1: u8 = 0o21;
pub const OP_COP2: u8 = 0o22;
pub const OP_BEQL: u8 = 0o24;
pub const OP_BNEL: u8 = 0o25;
pub const OP_BLEZL: u8 = 0o26;
pub const OP_BGTZL: u8 = 0o27;
pub const OP_DADDI: u8 = 0o30;
pub const OP_DADDIU: u8 = 0o31;
pub const OP_LDL: u8 = 0o32;
pub const OP_LDR: u8 = 0o33;
pub const OP_LB: u8 = 0o40;
pub const OP_LH: u8 = 0o41;
pub const OP_LWL: u8 = 0o42;
pub const OP_LW: u8 = 0o43;
pub const OP_LBU: u8 = 0o44;
pub const OP_LHU: u8 = 0o45;
pub const OP_LWR: u8 = 0o46;
pub const OP_LWU: u8 = 0o47;
pub const OP_SB: u8 = 0o50;
pub const OP_SH: u8 = 0o51;
pub const OP_SWL: u8 = 0o52;
pub const OP_SW: u8 = 0o53;
pub const OP_SDL: u8 = 0o54;
pub const OP_SDR: u8 = 0o55;
pub const OP_SWR: u8 = 0o56;
pub const OP_LL: u8 = 0o60;
pub const OP_LWC1: u8 = 0o61;
pub const OP_LWC2: u8 = 0o62;
pub const OP_LLD: u8 = 0o64;
pub const OP_LDC1: u8 = 0o65;
pub const OP_LDC2: u8 = 0o66;
pub const OP_LD: u8 = 0o67;
pub const OP_SC: u8 = 0o70;
pub const OP_SWC1: u8 = 0o71;
pub const OP_SWC2: u8 = 0o72;
pub const OP_SCD: u8 = 0o74;
pub const OP_SDC1: u8 = 0o75;
pub const OP_SDC2: u8 = 0o76;
pub const OP_SD: u8 = 0o77;

pub const OP_SP_SLL: u8 = 0o00;
pub const OP_SP_SRL: u8 = 0o02;
pub const OP_SP_SRA: u8 = 0o03;
pub const OP_SP_SLLV: u8 = 0o04;
pub const OP_SP_SRLV: u8 = 0o06;
pub const OP_SP_SRAV: u8 = 0o07;
pub const OP_SP_JR: u8 = 0o10;
pub const OP_SP_JALR: u8 = 0o11;
pub const OP_SP_SYSCALL: u8 = 0o14;
pub const OP_SP_BREAK: u8 = 0o15;
pub const OP_SP_SYNC: u8 = 0o17;
pub const OP_SP_MFHI: u8 = 0o20;
pub const OP_SP_MTHI: u8 = 0o21;
pub const OP_SP_MFLO: u8 = 0o22;
pub const OP_SP_MTLO: u8 = 0o23;
pub const OP_SP_DSLLV: u8 = 0o24;
pub const OP_SP_DSRLV: u8 = 0o26;
pub const OP_SP_DSRAV: u8 = 0o27;
pub const OP_SP_MULT: u8 = 0o30;
pub const OP_SP_MULTU: u8 = 0o31;
pub const OP_SP_DIV: u8 = 0o32;
pub const OP_SP_DIVU: u8 = 0o33;
pub const OP_SP_DMULT: u8 = 0o34;
pub const OP_SP_DMULTU: u8 = 0o35;
pub const OP_SP_DDIV: u8 = 0o36;
pub const OP_SP_DDIVU: u8 = 0o37;
pub const OP_SP_ADD: u8 = 0o40;
pub const OP_SP_ADDU: u8 = 0o41;
pub const OP_SP_SUB: u8 = 0o42;
pub const OP_SP_SUBU: u8 = 0o43;
pub const OP_SP_AND: u8 = 0o44;
pub const OP_SP_OR: u8 = 0o45;
pub const OP_SP_XOR: u8 = 0o46;
pub const OP_SP_NOR: u8 = 0o47;
pub const OP_SP_SLT: u8 = 0o52;
pub const OP_SP_SLTU: u8 = 0o53;
pub const OP_SP_DADD: u8 = 0o54;
pub const OP_SP_DADDU: u8 = 0o55;
pub const OP_SP_DSUB: u8 = 0o56;
pub const OP_SP_DSUBU: u8 = 0o57;
pub const OP_SP_TGE: u8 = 0o60;
pub const OP_SP_TGEU: u8 = 0o61;
pub const OP_SP_TLT: u8 = 0o62;
pub const OP_SP_TLTU: u8 = 0o63;
pub const OP_SP_TEQ: u8 = 0o64;
pub const OP_SP_TNE: u8 = 0o66;
pub const OP_SP_DSLL: u8 = 0o70;
pub const OP_SP_DSRL: u8 = 0o72;
pub const OP_SP_DSRA: u8 = 0o73;
pub const OP_SP_DSLL32: u8 = 0o74;
pub const OP_SP_DSRL32: u8 = 0o76;
pub const OP_SP_DSRA32: u8 = 0o77;

pub const OP_IR_BLTZ: u8 = 0o00;
pub const OP_IR_BGEZ: u8 = 0o01;
pub const OP_IR_BLTZL: u8 = 0o02;
pub const OP_IR_BGEZL: u8 = 0o03;
pub const OP_IR_TGEI: u8 = 0o10;
pub const OP_IR_TGEIU: u8 = 0o11;
pub const OP_IR_TLTI: u8 = 0o12;
pub const OP_IR_TLTIU: u8 = 0o13;
pub const OP_IR_TEQI: u8 = 0o14;
pub const OP_IR_TNEI: u8 = 0o16;
pub const OP_IR_BLTZAL: u8 = 0o20;
pub const OP_IR_BGEZAL: u8 = 0o21;
pub const OP_IR_BLTZALL: u8 = 0o22;
pub const OP_IR_BGEZALL: u8 = 0o23;

pub const OP_COP_MF: u8 = 0o00;
pub const OP_COP_DMF: u8 = 0o01;
pub const OP_COP_CF: u8 = 0o02;
pub const OP_COP_MT: u8 = 0o04;
pub const OP_COP_DMT: u8 = 0o05;
pub const OP_COP_CT: u8 = 0o06;
pub const OP_COP_BC: u8 = 0o10;

pub const OP_COP_BC_BCF: u8 = 0o00;
pub const OP_COP_BC_BCT: u8 = 0o01;
pub const OP_COP_BC_BCFL: u8 = 0o02;
pub const OP_COP_BC_BCTL: u8 = 0o03;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reg(pub u8);


pub fn print_instr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    match instr.opcode() {
        OP_SPECIAL => print_special_instr(o, instr)?,
        OP_REGIMM => print_regimm_instr(o, instr)?,
        OP_J => print_j(o, instr)?,
        OP_JAL => print_jal(o, instr)?,
        OP_BEQ => print_beq(o, instr)?,
        OP_BNE => print_bne(o, instr)?,
        OP_BLEZ => print_blez(o, instr)?,
        OP_BGTZ => print_bgtz(o, instr)?,
        OP_ADDI => print_addi(o, instr)?,
        OP_ADDIU => print_addiu(o, instr)?,
        OP_SLTI => print_slti(o, instr)?,
        OP_SLTIU => print_sltiu(o, instr)?,
        OP_ANDI => print_andi(o, instr)?,
        OP_ORI => print_ori(o, instr)?,
        OP_XORI => print_xori(o, instr)?,
        OP_LUI => print_lui(o, instr)?,
        OP_COP0 => print_cop_instr(o, instr, 0)?,
        OP_COP1 => print_cop_instr(o, instr, 1)?,
        OP_COP2 => print_cop_instr(o, instr, 2)?,
        OP_BEQL => print_beql(o, instr)?,
        OP_BNEL => print_bnel(o, instr)?,
        OP_BLEZL => print_blezl(o, instr)?,
        OP_BGTZL => print_bgtzl(o, instr)?,
        OP_DADDI => print_daddi(o, instr)?,
        OP_DADDIU => print_daddiu(o, instr)?,
        OP_LDL => print_ldl(o, instr)?,
        OP_LDR => print_ldr(o, instr)?,
        OP_LB => print_lb(o, instr)?,
        OP_LH => print_lh(o, instr)?,
        OP_LWL => print_lwl(o, instr)?,
        OP_LW => print_lw(o, instr)?,
        OP_LBU => print_lbu(o, instr)?,
        OP_LHU => print_lhu(o, instr)?,
        OP_LWR => print_lwr(o, instr)?,
        OP_LWU => print_lwu(o, instr)?,
        OP_SB => print_sb(o, instr)?,
        OP_SH => print_lh(o, instr)?,
        OP_SWL => print_swl(o, instr)?,
        OP_SW => print_sw(o, instr)?,
        OP_SDL => print_sdl(o, instr)?,
        OP_SDR => print_sdr(o, instr)?,
        OP_SWR => print_swr(o, instr)?,
        OP_LL => print_ll(o, instr)?,
        OP_LWC1 => print_lwcz(o, instr, 1)?,
        OP_LWC2 => print_lwcz(o, instr, 2)?,
        OP_LLD => print_lld(o, instr)?,
        OP_LDC1 => print_ldcz(o, instr, 1)?,
        OP_LDC2 => print_ldcz(o, instr, 2)?,
        OP_LD => print_ld(o, instr)?,
        OP_SC => print_sc(o, instr)?,
        OP_SWC1 => print_swcz(o, instr, 1)?,
        OP_SWC2 => print_swcz(o, instr, 2)?,
        OP_SCD => print_scd(o, instr)?,
        OP_SDC1 => print_sdcz(o, instr, 1)?,
        OP_SDC2 => print_sdcz(o, instr, 2)?,
        OP_SD => print_sd(o, instr)?,
        or => write!(o, "unprintable opcode {or:o}")?,
    }

    Ok(())
}
fn print_special_instr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    match instr.funct() {
        OP_SP_SLL => print_sll(o, instr)?,
        OP_SP_SRL => print_srl(o, instr)?,
        OP_SP_SRA => print_sra(o, instr)?,
        OP_SP_SLLV => print_sllv(o, instr)?,
        OP_SP_SRLV => print_srlv(o, instr)?,
        OP_SP_SRAV => print_srav(o, instr)?,
        OP_SP_JR => print_jr(o, instr)?,
        OP_SP_JALR => print_jalr(o, instr)?,
        OP_SP_SYSCALL => print_syscall(o, instr)?,
        OP_SP_BREAK => print_break(o, instr)?,
        OP_SP_SYNC => print_sync(o, instr)?,
        OP_SP_MFHI => print_mfhi(o, instr)?,
        OP_SP_MTHI => print_mthi(o, instr)?,
        OP_SP_MFLO => print_mflo(o, instr)?,
        OP_SP_MTLO => print_mtlo(o, instr)?,
        OP_SP_DSLLV => print_dsllv(o, instr)?,
        OP_SP_DSRLV => print_dsrlv(o, instr)?,
        OP_SP_DSRAV => print_dsrav(o, instr)?,
        OP_SP_MULT => print_mult(o, instr)?,
        OP_SP_MULTU => print_multu(o, instr)?,
        OP_SP_DIV => print_div(o, instr)?,
        OP_SP_DIVU => print_divu(o, instr)?,
        OP_SP_DMULT => print_dmult(o, instr)?,
        OP_SP_DMULTU => print_dmultu(o, instr)?,
        OP_SP_DDIV => print_ddiv(o, instr)?,
        OP_SP_DDIVU => print_ddivu(o, instr)?,
        OP_SP_ADD => print_add(o, instr)?,
        OP_SP_ADDU => print_addu(o, instr)?,
        OP_SP_SUB => print_sub(o, instr)?,
        OP_SP_SUBU => print_subu(o, instr)?,
        OP_SP_AND => print_and(o, instr)?,
        OP_SP_OR => print_or(o, instr)?,
        OP_SP_XOR => print_xor(o, instr)?,
        OP_SP_NOR => print_nor(o, instr)?,
        OP_SP_SLT => print_slt(o, instr)?,
        OP_SP_SLTU => print_sltu(o, instr)?,
        OP_SP_DADD => print_dadd(o, instr)?,
        OP_SP_DADDU => print_daddu(o, instr)?,
        OP_SP_DSUB => print_dsub(o, instr)?,
        OP_SP_DSUBU => print_dsubu(o, instr)?,
        OP_SP_TGE => print_tge(o, instr)?,
        OP_SP_TGEU => print_tgeu(o, instr)?,
        OP_SP_TLT => print_tlt(o, instr)?,
        OP_SP_TLTU => print_tltu(o, instr)?,
        OP_SP_TEQ => print_teq(o, instr)?,
        OP_SP_TNE => print_tne(o, instr)?,
        OP_SP_DSLL => print_dsll(o, instr)?,
        OP_SP_DSRL => print_dsrl(o, instr)?,
        OP_SP_DSRA => print_dsra(o, instr)?,
        OP_SP_DSLL32 => print_dsll32(o, instr)?,
        OP_SP_DSRL32 => print_dsrl32(o, instr)?,
        OP_SP_DSRA32 => print_dsra32(o, instr)?,
        or => write!(o, "unprintable SPECIAL {or:o}")?,
    }
    Ok(())
}
fn print_regimm_instr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    match instr.rt().0 {
        or => write!(o, "unprintable REGIMM {or:o}")?,
    }
    Ok(())
}
fn print_cop_instr(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    match instr.rs().0 {
        OP_COP_MF => print_mfcz(o, instr, cop)?,
        OP_COP_DMF => print_dmfcz(o, instr, cop)?,
        OP_COP_CF => print_cfcz(o, instr, cop)?,
        OP_COP_MT => print_mtcz(o, instr, cop)?,
        OP_COP_DMT => print_dmtcz(o, instr, cop)?,
        OP_COP_CT => print_mtcz(o, instr, cop)?,
        OP_COP_BC => print_cop_bc_instr(o, instr, cop)?,
        0o20..=0o37 => print_copz(o, instr, cop)?,
        or => write!(o, "unprintable COPz {or:o}")?,
    }
    Ok(())
}
fn print_cop_bc_instr(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    match instr.rt().0 {
        OP_COP_BC_BCF => print_bczf(o, instr, cop)?,
        OP_COP_BC_BCT => print_bczt(o, instr, cop)?,
        OP_COP_BC_BCFL => print_bczfl(o, instr, cop)?,
        OP_COP_BC_BCTL => print_bcztl(o, instr, cop)?,
        or => write!(o, "unprintable COPz BC {or:o}")?,
    }
    Ok(())
}

pub fn print_gp_reg(o: &mut impl Write, r: Reg) -> io::Result<()> {
    let r = r.0;
    let (p, n) = match r {
        0 => return write!(o, "zr"),
        1 => return write!(o, "at"),
        2 | 3 => ('v', r - 2),
        4..=7 => ('a', r - 4),
        8..=15 => ('t', r - 8),
        16..=23 => ('s', r - 16),
        24 | 25 => ('t', r - 16),
        26 | 27 => ('k', r - 26),
        28 => return write!(o, "gp"),
        29 => return write!(o, "sp"),
        30 => return write!(o, "fp"),
        31 => return write!(o, "ra"),
        _ => unreachable!(),
    };

    write!(o, "{p}{n}")?;
    Ok(())
}
fn print_binary_instr(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rt())?;
    Ok(())
}
fn print_three_addr_instr(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rd())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rt())?;

    Ok(())
}
fn print_reg_imm_instr(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_binary_branch(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_unary_branch(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_cop_mov_instr(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_shift(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rd())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", {}", instr.sa())?;
    Ok(())
}
fn print_vshift(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rd())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rs())?;
    Ok(())
}
fn print_memory_instr(o: &mut impl Write, mnemonic: &str, instr: Instr) -> io::Result<()> {
    write!(o, "{mnemonic} ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", 0x{:x}(", instr.immi())?;
    print_gp_reg(o, instr.base())?;
    write!(o, ")")?;
    Ok(())
}


fn print_add(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "ADD", instr)?;
    Ok(())
}
fn print_addi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "ADDI", instr)?;
    Ok(())
}
fn print_addiu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "ADDIU", instr)?;
    Ok(())
}
fn print_addu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "ADDU", instr)?;
    Ok(())
}
fn print_and(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "AND", instr)?;
    Ok(())
}
fn print_andi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "ANDI", instr)?;
    Ok(())
}
fn print_bczf(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "BC{cop}F 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_bczfl(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "BC{cop}FL 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_bczt(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "BC{cop}T 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_bcztl(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "BC{cop}TL 0x{:x}", instr.branch_offset())?;
    Ok(())
}
fn print_beq(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_branch(o, "BEQ", instr)?;
    Ok(())
}
fn print_beql(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_branch(o, "BEQL", instr)?;
    Ok(())
}
fn print_bgez(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGEZ", instr)?;
    Ok(())
}
fn print_bgezal(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGEZAL", instr)?;
    Ok(())
}
fn print_bgezall(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGEZALL", instr)?;
    Ok(())
}
fn print_bgezl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGEZL", instr)?;
    Ok(())
}
fn print_bgtz(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGTZ", instr)?;
    Ok(())
}
fn print_bgtzl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BGTZL", instr)?;
    Ok(())
}
fn print_blez(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLEZ", instr)?;
    Ok(())
}
fn print_blezl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLEZL", instr)?;
    Ok(())
}
fn print_bltz(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLTZ", instr)?;
    Ok(())
}
fn print_bltzal(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLTZAL", instr)?;
    Ok(())
}
fn print_bltzall(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLTZALL", instr)?;
    Ok(())
}
fn print_bltzl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_unary_branch(o, "BLTZL", instr)?;
    Ok(())
}
fn print_bne(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_branch(o, "BNE", instr)?;
    Ok(())
}
fn print_bnel(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_branch(o, "BNEL", instr)?;
    Ok(())
}
fn print_break(o: &mut impl Write, _instr: Instr) -> io::Result<()> {
    write!(o, "BREAK")?;
    Ok(())
}
fn print_cfcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("CFC{cop}"), instr)?;
    Ok(())
}
fn print_copz(o: &mut impl Write, _instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "COP{cop}")?;
    Ok(())
}
fn print_ctcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("CTC{cop}"), instr)?;
    Ok(())
}
fn print_dadd(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "DADD", instr)?;
    Ok(())
}
fn print_daddi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "DADDI", instr)?;
    Ok(())
}
fn print_daddiu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "DADDIU", instr)?;
    Ok(())
}
fn print_daddu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "DADDU", instr)?;
    Ok(())
}
fn print_ddiv(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DDIV", instr)?;
    Ok(())
}
fn print_ddivu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DDIVU", instr)?;
    Ok(())
}
fn print_div(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DIV", instr)?;
    Ok(())
}
fn print_divu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DIVU", instr)?;
    Ok(())
}
fn print_dmfcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("DMFC{cop}"), instr)?;
    Ok(())
}
fn print_dmtcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("DMTC{cop}"), instr)?;
    Ok(())
}
fn print_dmult(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DMULT", instr)?;
    Ok(())
}
fn print_dmultu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "DMULTU", instr)?;
    Ok(())
}
fn print_dsll(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSLL", instr)?;
    Ok(())
}
fn print_dsllv(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "DSLLV", instr)?;
    Ok(())
}
fn print_dsll32(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSLL32", instr)?;
    Ok(())
}
fn print_dsra(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSRA", instr)?;
    Ok(())
}
fn print_dsrav(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "DSRAV", instr)?;
    Ok(())
}
fn print_dsra32(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSRA32", instr)?;
    Ok(())
}
fn print_dsrl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSRL", instr)?;
    Ok(())
}
fn print_dsrlv(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "DSRLV", instr)?;
    Ok(())
}
fn print_dsrl32(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "DSRL32", instr)?;
    Ok(())
}
fn print_dsub(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "DSUB", instr)?;
    Ok(())
}
fn print_dsubu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "DSUBU", instr)?;
    Ok(())
}
fn print_j(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "J 0x{:x}", instr.jump_offset())?;
    Ok(())
}
fn print_jal(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "JAL 0x{:x}", instr.jump_offset())?;
    Ok(())
}
fn print_jalr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "JALR ")?;
    print_gp_reg(o, instr.rd())?;
    write!(o, ", ")?;
    print_gp_reg(o, instr.rs())?;
    Ok(())
}
fn print_jr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "JR ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_lb(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LB", instr)?;
    Ok(())
}
fn print_lbu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LBU", instr)?;
    Ok(())
}
fn print_ld(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LD", instr)?;
    Ok(())
}
fn print_ldcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "LDC{cop} r{}, 0x{:x}(", instr.rt().0, instr.immi())?;
    print_gp_reg(o, instr.base())?;
    write!(o, ")")?;
    Ok(())
}
fn print_ldl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LDL", instr)?;
    Ok(())
}
fn print_ldr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LDR", instr)?;
    Ok(())
}
fn print_lh(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LH", instr)?;
    Ok(())
}
fn print_lhu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LHU", instr)?;
    Ok(())
}
fn print_ll(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LL", instr)?;
    Ok(())
}
fn print_lld(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LLD", instr)?;
    Ok(())
}
fn print_lui(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "LUI ")?;
    print_gp_reg(o, instr.rt())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_lw(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LW", instr)?;
    Ok(())
}
fn print_lwcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "LWC{cop} r{}, 0x{:x}(", instr.rt().0, instr.immi())?;
    print_gp_reg(o, instr.base())?;
    write!(o, ")")?;
    Ok(())
}
fn print_lwl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LWL", instr)?;
    Ok(())
}
fn print_lwr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LWR", instr)?;
    Ok(())
}
fn print_lwu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "LWU", instr)?;
    Ok(())
}
fn print_mfcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("MFC{cop}"), instr)?;
    Ok(())
}
fn print_mfhi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "MFHI ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_mflo(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "MFLO ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_mtcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    print_cop_mov_instr(o, &format!("MTC{cop}"), instr)?;
    Ok(())
}
fn print_mthi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "MTHI ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_mtlo(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "MTLO ")?;
    print_gp_reg(o, instr.rd())?;
    Ok(())
}
fn print_mult(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "MULT", instr)?;
    Ok(())
}
fn print_multu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "MULTU", instr)?;
    Ok(())
}
fn print_nor(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "NOR", instr)?;
    Ok(())
}
fn print_or(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "OR", instr)?;
    Ok(())
}
fn print_ori(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "ORI", instr)?;
    Ok(())
}
fn print_sb(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SB", instr)?;
    Ok(())
}
fn print_sc(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SC", instr)?;
    Ok(())
}
fn print_scd(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SCD", instr)?;
    Ok(())
}
fn print_sd(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SD", instr)?;
    Ok(())
}
fn print_sdcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "SDC{cop} r{}, 0x{:x}(", instr.rt().0, instr.immi())?;
    print_gp_reg(o, instr.base())?;
    write!(o, ")")?;
    Ok(())
}
fn print_sdl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SDL", instr)?;
    Ok(())
}
fn print_sdr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SDR", instr)?;
    Ok(())
}
fn print_sh(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "Sh", instr)?;
    Ok(())
}
fn print_sll(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "SLL", instr)?;
    Ok(())
}
fn print_sllv(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "SLLV", instr)?;
    Ok(())
}
fn print_slt(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "SLT", instr)?;
    Ok(())
}
fn print_slti(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "SLTI", instr)?;
    Ok(())
}
fn print_sltiu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "SLTIU", instr)?;
    Ok(())
}
fn print_sltu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "SLTU", instr)?;
    Ok(())
}
fn print_sra(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "SRA", instr)?;
    Ok(())
}
fn print_srav(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "SRAV", instr)?;
    Ok(())
}
fn print_srl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_shift(o, "SRL", instr)?;
    Ok(())
}
fn print_srlv(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_vshift(o, "SRLV", instr)?;
    Ok(())
}
fn print_sub(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "SUB", instr)?;
    Ok(())
}
fn print_subu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "SUBU", instr)?;
    Ok(())
}
fn print_sw(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SW", instr)?;
    Ok(())
}
fn print_swcz(o: &mut impl Write, instr: Instr, cop: u8) -> io::Result<()> {
    write!(o, "SWC{cop} r{}, 0x{:x}(", instr.rt().0, instr.immi())?;
    print_gp_reg(o, instr.base())?;
    write!(o, ")")?;
    Ok(())
}
fn print_swl(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SWL", instr)?;
    Ok(())
}
fn print_swr(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_memory_instr(o, "SWR", instr)?;
    Ok(())
}
fn print_sync(o: &mut impl Write, _instr: Instr) -> io::Result<()> {
    write!(o, "SYNC")?;
    Ok(())
}
fn print_syscall(o: &mut impl Write, _instr: Instr) -> io::Result<()> {
    write!(o, "SYSCALL")?;
    Ok(())
}
fn print_teq(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TEQ", instr)?;
    Ok(())
}
fn print_teqi(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TEQI ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_tge(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TGE", instr)?;
    Ok(())
}
fn print_tgei(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TGEI ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_tgeiu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TGEIU ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_tgeu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TGEU", instr)?;
    Ok(())
}
fn print_tlt(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TLT", instr)?;
    Ok(())
}
fn print_tlti(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TLTI ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_tltiu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TLTIU ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_tltu(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TLTU", instr)?;
    Ok(())
}
fn print_tne(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_binary_instr(o, "TNE", instr)?;
    Ok(())
}
fn print_tnei(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    write!(o, "TNEI ")?;
    print_gp_reg(o, instr.rs())?;
    write!(o, ", 0x{:x}", instr.immi())?;
    Ok(())
}
fn print_xor(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_three_addr_instr(o, "XOR", instr)?;
    Ok(())
}
fn print_xori(o: &mut impl Write, instr: Instr) -> io::Result<()> {
    print_reg_imm_instr(o, "XORI", instr)?;
    Ok(())
}
