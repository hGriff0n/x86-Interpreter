
pub const EVEN: bool = false;
pub const ODD: bool = true;

use std::convert;

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

impl convert::From<u32> for FlagRegister {
    fn from(reg: u32) -> FlagRegister {
        Self {
            carry: (reg & 0x1) != 0,
            parity: (reg & 0x4) != 0,
            adjust: (reg & 0x10) != 0,
            zero: (reg & 0x40) != 0,
            sign: (reg & 0x80) != 0,
            trap: (reg & 0x100) != 0,
            interrupt: (reg & 0x200) != 0,
            direction: (reg & 0x400) != 0,
            overflow: (reg & 0x800) != 0,
            nested: (reg & 0x4000) != 0,
            resume: (reg & 0x10000) != 0,
            virt: (reg & 0x20000) != 0,
            align: (reg & 0x40000) != 0,
            vinterrupt: (reg & 0x80000) != 0,
            pending_int: (reg & 0x100000) != 0,
            cpuid: (reg & 0x200000) != 0
        }
    }
}

impl<'a> convert::From<&'a FlagRegister> for u32 {
    fn from(flags: &'a FlagRegister) -> u32 {
        let mut reg = 0x2 | (flags.carry as u32);
        reg |= (flags.parity as u32) << 2;
        reg |= (flags.adjust as u32) << 4;
        reg |= (flags.zero as u32) << 6;
        reg |= (flags.sign as u32) << 7;
        reg |= (flags.trap as u32) << 8;
        reg |= (flags.interrupt as u32) << 9;
        reg |= (flags.direction as u32) << 10;
        reg |= (flags.overflow as u32) << 11;
        reg |= (flags.nested as u32) << 14;
        reg |= (flags.resume as u32) << 16;
        reg |= (flags.virt as u32) << 17;
        reg |= (flags.align as u32) << 18;
        reg |= (flags.vinterrupt as u32) << 19;
        reg |= (flags.pending_int as u32) << 20;
        reg |= (flags.cpuid as u32) << 21;
        reg
    }
}

impl<'a> convert::From<&'a mut FlagRegister> for u32 {
    fn from(flags: &'a mut FlagRegister) -> u32 {
        From::from(&*flags)
    }
}
