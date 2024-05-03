use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instr(pub u32);
impl Instr {
    pub fn opcode(self) -> u8 {
        (self.0 >> 26) as u8
    }
    pub fn rs(self) -> u8 {
        (self.0 >> 21) as u8 & 0x1F
    }
    pub fn base(self) -> u8 {
        self.rs()
    }
    pub fn rt(self) -> u8 {
        (self.0 >> 16) as u8 & 0x1F
    }
    pub fn rd(self) -> u8 {
        (self.0 >> 11) as u8 & 0x1F
    }
    pub fn sa(self) -> u8 {
        (self.0 >> 06) as u8 & 0x1F
    }
    pub fn funct(self) -> u8 {
        self.0 as u8 & 0x1F
    }
    pub fn immediate(self) -> u16 {
        self.0 as u16
    }
    pub fn address(self) -> u32 {
        self.0 & 0x3FFFFFF
    }

    pub const ADDIU: u8 = 0o11;
    pub const ANDI: u8 = 0o14;
    pub const ORI: u8 = 0o15;
    pub const LUI: u8 = 0o17;
    pub const COP0: u8 = 0o20;
    pub const BEQL: u8 = 0o24;
    pub const LW: u8 = 0o43;

    pub const COPZ_MT: u8 = 0o04;
}
impl Display for Instr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.opcode() {
            Self::ADDIU => write!(
                f,
                "ADDIU r{}, r{}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::ANDI => write!(
                f,
                "ANDI r{}, r{}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::ORI => write!(
                f,
                "ORI r{}, r{}, 0x{:x}",
                self.rt(),
                self.rs(),
                self.immediate()
            ),
            Self::LUI => write!(f, "LUI, r{}, 0x{:x}", self.rt(), self.immediate()),
            Self::COP0 => write!(f, "COP0"),
            Self::BEQL => write!(
                f,
                "BEQL r{}, r{}, 0x{:x}",
                self.rs(),
                self.rt(),
                self.immediate()
            ),
            Self::LW => write!(
                f,
                "LW r{}, 0x{:x}(r{})",
                self.rt(),
                self.immediate(),
                self.base()
            ),
            or => write!(f, "undisplayable opcode 0o{or:o}"),
        }
    }
}
