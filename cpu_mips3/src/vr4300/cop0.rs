use util::{bitmask_32, get_field_32, get_flag_32, set_flag_32, sign_bit_32};

pub struct Cop0 {
    pub index: Index,
    pub random: Random,
    pub entry_lo0: EntryLo,
    pub entry_lo1: EntryLo,
    pub context: Context,
    pub page_mask: PageMask,
    pub wired: Wired,
    pub bad_vaddr: u64,
    pub count: u32,
    pub compare: u32,
    pub status: Status,
    pub cause: Cause,
    pub epc: u64,
    pub config: Config,
    pub ll_addr: u32,
    pub watch_lo: WatchLo,
    pub watch_hi: WatchHi,
    pub xcontext: XContext,
    pub perr: PErr,
    pub tag_lo: TagLo,
    pub err_epc: u64,
}
impl Cop0 {
    pub fn init() -> Self {
        Self {
            index: Index::init(),
            random: Random::init(),
            entry_lo0: EntryLo::init(),
            entry_lo1: EntryLo::init(),
            context: Context::init(),
            page_mask: PageMask::init(),
            wired: Wired::init(),
            bad_vaddr: 0,
            count: 0,
            compare: 0,
            status: Status::init(),
            cause: Cause::init(),
            epc: 0,
            config: Config::init(),
            ll_addr: 0,
            watch_lo: WatchLo::init(),
            watch_hi: WatchHi::init(),
            xcontext: XContext::init(),
            perr: PErr::init(),
            tag_lo: TagLo::init(),
            err_epc: 0,
        }
    }

    pub fn is_big_endian(&self) -> bool {
        let flip = self.status.re() && self.status.is_user_mode();
        self.config.be() ^ flip
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Index(u8);
impl Index {
    pub fn init() -> Self {
        Self(0)
    }
    pub fn write(&mut self, word: u32) {
        let index = word as u8 & 0x3F;
        let p = if sign_bit_32(word) { 0x80 } else { 0 };
        self.0 = index | p
    }
    pub fn read(self) -> u32 {
        let index = self.index() as u32;
        let p = if self.p() { 0x8000_0000 } else { 0 };
        index | p
    }
    pub fn index(self) -> u8 {
        self.0 & 0x3F
    }
    pub fn p(self) -> bool {
        self.0 & 0x80 != 0
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Random(u8);
impl Random {
    pub fn init() -> Self {
        Self(31)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntryLo(u64);
impl EntryLo {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Context(u64);
impl Context {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PageMask(u32);
impl PageMask {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Wired(u8);
impl Wired {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Status(u32);
impl Status {
    pub fn init() -> Self {
        let mut data = 0;
        set_flag_32(&mut data, Self::ERL, true);
        set_flag_32(&mut data, Self::DS_BEV, true);
        Self(data)
    }

    pub fn write(&mut self, val: u32) {
        self.0 = val & !Self::RFU_MASK;
    }
    pub fn read(self) -> u32 {
        self.0 & !Self::RFU_MASK
    }

    pub fn cop0_enabled(self) -> bool {
        get_flag_32(self.0, Self::CU)
    }

    pub fn is_64_bit_mode(self) -> bool {
        if self.is_user_mode() {
            self.ux()
        } else if self.is_supervisor_mode() {
            self.sx()
        } else if self.is_kernel_mode() {
            self.kx()
        } else {
            unreachable!()
        }
    }
    pub fn is_user_mode(self) -> bool {
        !self.exl() && !self.erl() && self.ksu() == 2
    }
    pub fn is_supervisor_mode(self) -> bool {
        !self.exl() && !self.erl() && self.ksu() == 1
    }
    pub fn is_kernel_mode(self) -> bool {
        self.exl() || self.erl() || self.ksu() == 0
    }

    fn exl(self) -> bool {
        get_flag_32(self.0, Self::EXL)
    }
    fn erl(self) -> bool {
        get_flag_32(self.0, Self::ERL)
    }
    fn ksu(self) -> u8 {
        get_field_32(self.0, Self::KSU, Self::KSU_WIDTH) as u8
    }
    fn ux(self) -> bool {
        get_flag_32(self.0, Self::UX)
    }
    fn sx(self) -> bool {
        get_flag_32(self.0, Self::SX)
    }
    fn kx(self) -> bool {
        get_flag_32(self.0, Self::KX)
    }
    fn re(self) -> bool {
        get_flag_32(self.0, Self::RE)
    }

    const IE: u32 = 0;
    const EXL: u32 = 1;
    const ERL: u32 = 2;
    const KSU: u32 = 3;
    const UX: u32 = 5;
    const SX: u32 = 6;
    const KX: u32 = 7;
    const IM: u32 = 8;
    const DS_DE: u32 = 16;
    const DS_CE: u32 = 17;
    const DS_CH: u32 = 18;
    const DS_SR: u32 = 20;
    const DS_TS: u32 = 21;
    const DS_BEV: u32 = 22;
    const DS_ITS: u32 = 24;
    const RE: u32 = 25;
    const FR: u32 = 26;
    const RP: u32 = 27;
    const CU: u32 = 28;

    const KSU_WIDTH: u32 = 2;
    const IM_WIDTH: u32 = 8;
    const CU_WIDTH: u32 = 4;

    const RFU_MASK: u32 = 1 << 23;
}

#[derive(Copy, Clone, Debug)]
pub struct Cause(u32);
impl Cause {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Config(u32);
impl Config {
    pub fn init() -> Self {
        let mut data = 0;
        set_flag_32(&mut data, Self::BE, true);

        Self(data)
    }

    pub fn write(&mut self, data: u32) {
        self.0 = data & !Self::CONST_MASK;
    }
    pub fn read(self) -> u32 {
        self.0 & !Self::CONST_MASK | Self::CONST_VAL
    }

    pub fn be(self) -> bool {
        get_flag_32(self.0, Self::BE)
    }

    const K0: u32 = 0;
    const CU: u32 = 1;
    const BE: u32 = 15;
    const EP: u32 = 24;
    const EC: u32 = 28;

    const K0_WIDTH: u32 = 3;
    const EP_WIDTH: u32 = 4;
    const EC_WIDTH: u32 = 3;

    const CONST_MASK: u32 = bitmask_32(4, 11) | bitmask_32(16, 8) | bitmask_32(30, 1);
    const CONST_VAL: u32 = 0b110_0_11001000110_0_000;
}

#[derive(Copy, Clone, Debug)]
pub struct WatchLo(u32);
impl WatchLo {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct WatchHi(u8);
impl WatchHi {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct XContext(u64);
impl XContext {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PErr(u8);
impl PErr {
    pub fn init() -> Self {
        Self(0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TagLo(u32);
impl TagLo {
    pub fn init() -> Self {
        Self(0)
    }
}
