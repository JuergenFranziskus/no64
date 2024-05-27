use std::fmt::Display;

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
    pub fn immediate(self) -> u16 {
        self.0 as u16
    }
    pub fn address(self) -> u32 {
        self.0 & 0x3FFFFFF
    }

    pub const SPECIAL: u8 = 0o00;
    pub const BEQ: u8 = 0o04;
    pub const BNE: u8 = 0o05;
    pub const ADDIU: u8 = 0o11;
    pub const ANDI: u8 = 0o14;
    pub const ORI: u8 = 0o15;
    pub const LUI: u8 = 0o17;
    pub const COP0: u8 = 0o20;
    pub const BEQL: u8 = 0o24;
    pub const BNEL: u8 = 0o25;
    pub const LW: u8 = 0o43;
    pub const SW: u8 = 0o53;

    pub const SPECIAL_SRL: u8 = 0o02;
    pub const SPECIAL_JR: u8 = 0o10;
    pub const SPECIAL_OR: u8 = 0o45;

    pub const COPZ_MT: u8 = 0o04;
}
impl Instr {
    pub fn is_load_store(self) -> bool {
        matches!(self.opcode(), Self::LW | Self::SW)
    }
    fn print_special(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.funct() {
            Self::SPECIAL_SRL => write!(f, "SRL {}, {}, {}", self.rd(), self.rt(), self.sa()),
            Self::SPECIAL_JR => write!(f, "JR {}", self.rs()),
            Self::SPECIAL_OR => write!(f, "OR {}, {}, {}", self.rd(), self.rs(), self.rt()),
            or => write!(
                f,
                "SPECIAL? 0o{or:o}; rs = {}, rt = {}, rd = {} sa = {}",
                self.rs(),
                self.rt(),
                self.rd(),
                self.sa()
            ),
        }
    }
    fn print_copz(&self, cop: u8, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let func = self.rs().0;
        match func {
            Self::COPZ_MT => write!(f, "MTC{cop} r{}, {}", self.rd().0, self.rt())?,
            or => write!(f, "COP{cop}? 0o{or:o}")?,
        }

        Ok(())
    }
}
impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode() {
            Self::SPECIAL => self.print_special(f),
            Self::BEQ => write!(
                f,
                "BEQ {}, {}, 0x{:x}",
                self.rs(),
                self.rt(),
                self.immediate()
            ),
            Self::BNE => write!(
                f,
                "BNE {}, {}, 0x{:x}",
                self.rs(),
                self.rt(),
                self.immediate()
            ),
            Self::ADDIU => write!(
                f,
                "ADDIU {}, {}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::ANDI => write!(
                f,
                "ANDI {}, {}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::ORI => write!(
                f,
                "ORI {}, {}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::LUI => write!(f, "LUI, {}, 0x{:x}", self.rt(), self.immediate()),
            Self::COP0 => self.print_copz(0, f),
            Self::BEQL => write!(
                f,
                "BEQL {}, {}, 0x{:x}",
                self.rs(),
                self.rt(),
                self.immediate()
            ),
            Self::BNEL => write!(
                f,
                "BNEL {}, {}, 0x{:x}",
                self.rs(),
                self.rt(),
                self.immediate()
            ),
            Self::LW => write!(
                f,
                "LW {}, 0x{:x}({})",
                self.rt(),
                self.immediate(),
                self.base()
            ),
            Self::SW => write!(
                f,
                "SW {}, 0x{:x}({})",
                self.rt(),
                self.immediate(),
                self.base()
            ),
            or => write!(f, "? 0o{or:o}"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Reg(pub u8);
impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let subend = match self.0 {
            0 => return write!(f, "zr"),
            1 => return write!(f, "at"),
            2 | 3 => {
                write!(f, "v")?;
                2
            }
            4..=7 => {
                write!(f, "a")?;
                4
            }
            8..=15 => {
                write!(f, "t")?;
                8
            }
            16..=23 => {
                write!(f, "s")?;
                16
            }
            24 | 25 => {
                write!(f, "t")?;
                16
            }
            26 | 27 => {
                write!(f, "k")?;
                26
            }
            28 => return write!(f, "gp"),
            29 => return write!(f, "sp"),
            30 => return write!(f, "fp"),
            31 => return write!(f, "ra"),
            _ => unreachable!(),
        };

        write!(f, "{}", self.0 - subend)?;
        Ok(())
    }
}
