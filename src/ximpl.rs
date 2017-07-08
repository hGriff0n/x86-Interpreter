use std::fmt;

// Types
// TODO: Merge `Code` and `Command` (for nicer debug printing)
//   ie. Parsed(Command) => `Command` | NOP => EndProgram
pub enum Code {
    Parsed(Command),
    Unread(String),
    EndProgram
}

#[derive(Debug)]
pub enum Command {
    Directive(String),
    Label(String),
    NoArg(String),
    OneArg(String, Argument),
    TwoArg(String, Argument, Argument),
    NOP
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Argument {
    Literal(i32),                           // value
    Reg(String),                            // register
    // NOTE: scale, mul properties are currently not parsed
    Mem(Box<Argument>, i32, i32, i32),      // idx, off, scale, mul
    Label(String),                          // label
}

// Enum for all CPU Flags
#[allow(dead_code)]
pub enum Flag {
    Carry,              // 0
    Parity,             // 2
    Adjust,             // 4
    Zero,               // 6
    Sign,               // 7
    Trap,               // 8
    Interrupt,          // 9
    Direction,          // 10
    Overflow,           // 11
    // IOPL,            // 12-13 (two-bits)
    Nested,             // 14
    Resume,             // 16
    Virtual,            // 17
    Alignment,          // 18
    VInterrupt,         // 19
    PendingInt,         // 20
    CPUID               // 21
}

pub fn mask_shift(f: Flag) -> usize {
    match f {
        Flag::Carry => 1,
        Flag::Parity => 2,
        Flag::Adjust => 4,
        Flag::Zero => 6,
        Flag::Sign => 7,
        Flag::Trap => 8,
        Flag::Interrupt => 9,
        Flag::Direction => 10,
        Flag::Overflow => 11,
        Flag::Nested => 14,
        Flag::Resume => 16,
        Flag::Virtual => 17,
        Flag::Alignment => 18,
        Flag::VInterrupt => 19,
        Flag::PendingInt => 20,
        Flag::CPUID => 21
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Code::Unread(ref s) => write!(f, "Unread({:?})", s),
            &Code::EndProgram => write!(f, "EndProgram"),
            &Code::Parsed(ref inst) => write!(f, "{:?}", inst)
        }
    }
}