
pub const EVEN: bool = false;
pub const ODD: bool = true;

#[derive(Debug)]
pub struct FlagRegister {
    pub carry: bool,
    pub parity: bool,           // true iff num ones is odd
    pub adjust: bool,
    pub zero: bool,
    pub sign: bool,
    pub trap: bool,
    pub interrupt: bool,
    pub direction: bool,
    pub overflow: bool,
    pub nested: bool,
    pub resume: bool,
    pub virt: bool,
    pub align: bool,
    pub vinterrupt: bool,
    pub pending_int: bool,
    pub cpuid: bool
}

impl FlagRegister {
    pub fn new() -> FlagRegister {
        Self {
            carry: false,
            parity: EVEN,
            adjust: false,
            zero: false,
            sign: false,
            trap: false,
            interrupt: false,
            direction: false,
            overflow: false,
            nested: false,
            resume: false,
            virt: false,
            align: false,
            vinterrupt: false,
            pending_int: false,
            cpuid: false
        }
    }

    pub fn copy(o: &FlagRegister) -> FlagRegister {
        Self {
            carry: o.carry,
            parity: o.parity,
            adjust: o.adjust,
            zero: o.zero,
            sign: o.sign,
            trap: o.trap,
            interrupt: o.interrupt,
            direction: o.direction,
            overflow: o.overflow,
            nested: o.nested,
            resume: o.resume,
            virt: o.virt,
            align: o.align,
            vinterrupt: o.vinterrupt,
            pending_int: o.pending_int,
            cpuid: o.cpuid
        }
    }
}