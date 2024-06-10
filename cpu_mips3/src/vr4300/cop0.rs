use bitfield_struct::bitfield;

#[derive(Copy, Clone, Debug, Default)]
pub struct Cop0 {
    pub index: Index,
    pub random: Random,
    pub entry_lo0: EntryLo,
    pub entry_lo1: EntryLo,
    pub context: Context,
    pub page_mask: PageMask,
    pub wired: Wired,
    pub bad_v_addr: BadVAddr,
    pub count: Count,
    pub entry_hi: EntryHi,
    pub compare: Compare,
    pub status: Status,
    pub cause: Cause,
    pub epc: EPC,
    pub config: Config,
    pub ll_addr: LLAddr,
    pub watch_lo: WatchLo,
    pub watch_hi: WatchHi,
    pub x_context: XContext,
    pub p_err: PErr,
    pub tag_lo: TagLo,
    pub error_epc: ErrorEPC,
}
impl Cop0 {
    pub fn init() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn mode(&self) -> Mode {
        self.status.mode()
    }
    pub fn is_64_bit_mode(&self) -> bool {
        self.status.is_64_bit_mode()
    }
    pub fn is_big_endian(&self) -> bool {
        self.config.be()
    }
    pub fn is_ksg0_cached(&self) -> bool {
        self.config.is_kseg0_cached()
    }
}

#[bitfield(u32)]
pub struct Index {
    #[bits(5)]
    index: u8,
    bit: bool,
    #[bits(25)]
    _rfu: usize,
    p: bool,
}

#[bitfield(u32)]
pub struct Random {
    #[bits(5, default = 31)]
    random: u8,
    bit: bool,
    #[bits(26)]
    _rfu: usize,
}

#[bitfield(u32)]
pub struct EntryLo {
    g: bool,
    v: bool,
    d: bool,
    #[bits(3)]
    c: u8,
    #[bits(20)]
    pfn: u32,
    #[bits(6)]
    _rfu: usize,
}

#[bitfield(u64)]
pub struct Context {
    #[bits(4)]
    _rfu: usize,
    #[bits(19)]
    bad_vpn2: u32,
    #[bits(41)]
    pte_base: u64,
}

#[bitfield(u32)]
pub struct PageMask {
    #[bits(13)]
    _rfu: usize,
    #[bits(12)]
    mask: u16,
    #[bits(7)]
    _rfu: usize,
}

#[bitfield(u32)]
pub struct Wired {
    #[bits(6)]
    wired: u8,
    #[bits(26)]
    _rfu: usize,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct BadVAddr(u64);

#[derive(Copy, Clone, Debug, Default)]
pub struct Count(u32);

#[bitfield(u64)]
pub struct EntryHi {
    asid: u8,
    #[bits(5)]
    _rfu: usize,
    #[bits(27)]
    vpn2: u32,
    #[bits(22)]
    _fill: usize,
    #[bits(2)]
    r: u8,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Compare(u32);

#[bitfield(u32)]
pub struct Status {
    ie: bool,
    exl: bool,
    #[bits(default = true)]
    erl: bool,
    #[bits(2)]
    ksu: u8,
    ux: bool,
    sx: bool,
    kx: bool,
    im: u8,
    de: bool,
    ce: bool,
    ch: bool,
    _rfu: bool,
    sr: bool,
    ts: bool,
    #[bits(default = true)]
    bev: bool,
    _rfu: bool,
    its: bool,
    re: bool,
    fr: bool,
    rp: bool,
    cu0: bool,
    cu1: bool,
    cu2: bool,
    cu3: bool,
}
impl Status {
    pub fn mode(self) -> Mode {
        let ksu = self.ksu();
        let exl = self.exl();
        let erl = self.erl();
        if exl || erl || ksu == 0 {
            Mode::Kernel
        } else if ksu == 1 {
            Mode::Supervisor
        } else {
            Mode::User
        }
    }

    pub fn is_64_bit_mode(self) -> bool {
        match self.mode() {
            Mode::Kernel => self.kx(),
            Mode::Supervisor => self.sx(),
            Mode::User => self.ux(),
        }
    }
}

#[bitfield(u32)]
pub struct Cause {
    #[bits(2)]
    _rfu: usize,
    #[bits(5)]
    exec_code: u8,
    _rfu: bool,
    ip: u8,
    #[bits(12)]
    _rfu: u16,
    #[bits(2)]
    ce: u8,
    _rfu: bool,
    bd: bool,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct EPC(u64);

#[bitfield(u32)]
pub struct Config {
    #[bits(3)]
    k0: u8,
    cu: bool,
    #[bits(11, default = 0b1100_1000_110)]
    _constant0: u16,
    #[bits(default = true)]
    be: bool,
    #[bits(8, default = 0b0000_0110)]
    _constant1: u8,
    #[bits(4)]
    ep: u8,
    #[bits(3)]
    ec: u8,
    _rfu: bool,
}
impl Config {
    pub fn is_kseg0_cached(self) -> bool {
        self.k0() != 0b010
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct LLAddr(u32);

#[bitfield(u32)]
pub struct WatchLo {
    w: bool,
    r: bool,
    _rfu: bool,
    #[bits(29)]
    paddr0: u32,
}

#[bitfield(u32)]
pub struct WatchHi {
    #[bits(4)]
    paddr1: u8,
    #[bits(28)]
    _rfu: usize,
}

#[bitfield(u64)]
pub struct XContext {
    #[bits(4)]
    _rfu: usize,
    #[bits(27)]
    bad_vpn2: u32,
    #[bits(2)]
    r: u8,
    #[bits(31)]
    pte_base: u32,
}

#[bitfield(u32)]
pub struct PErr {
    diagnostic: u8,
    #[bits(24)]
    _rfu: usize,
}

#[bitfield(u32)]
pub struct TagLo {
    #[bits(6)]
    _rfu: usize,
    #[bits(2)]
    pstate: u8,
    #[bits(20)]
    ptag_lo: u32,
    #[bits(4)]
    _rfu: usize,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ErrorEPC(u64);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Kernel,
    Supervisor,
    User,
}
