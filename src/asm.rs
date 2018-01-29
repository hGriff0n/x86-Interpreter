
use inst;
use processor::*;
use op::*;
use std::mem;

macro_rules! require {
    ($exp:expr) => (if !($exp) {
        panic!("'{}' failed at {}:{}", stringify!($exp), file!(), line!());
    });
}

macro_rules! require_if {
    ($cont:expr, $exp:expr) => (if $cont { require!($exp); });
}

pub fn aaa(al: &mut Operand, ah: &mut Operand, flags: &mut FlagRegister) {
    require!(al.len() == 1);
    require!(ah.len() == 1);

    inst::aaa(al.mref(), ah.mref(), flags);
}

pub fn aad(al: &mut Operand, ah: &mut Operand, imm8: u8, flags: &mut FlagRegister) {
    require!(al.len() == 1);
    require!(ah.len() == 1);

    inst::aad(al.mref(), ah.mref(), imm8, flags);
}

pub fn aad_10(al: &mut Operand, ah: &mut Operand, flags: &mut FlagRegister) {
    aad(al, ah, 10, flags);
}

pub fn aam(al: &mut Operand, ah: &mut Operand, imm8: u8, flags: &mut FlagRegister) {
    require!(al.len() == 1);
    require!(ah.len() == 1);

    inst::aam(al.mref(), ah.mref(), imm8, flags);
}

pub fn aam_10(al: &mut Operand, ah: &mut Operand, flags: &mut FlagRegister) {
    aam(al, ah, 10, flags);
}

pub fn aas(al: &mut Operand, ah: &mut Operand, flags: &mut FlagRegister) {
    require!(al.len() == 1);
    require!(ah.len() == 1);

    inst::aas(al.mref(), ah.mref(), flags);
}

// adc imm8 (, al)
// adc imm16 (, ax)
// adc imm32 (, eax)
// adc imm8, r/m8
// adc imm16, r/m16
// adc imm32, r/m32
// adc r8, r/m8
// adc r16, r/m16
// adc r32, r/m32
// adc r/m8, r8
// adc r/m16, r16
// adc r/m32, r32

pub fn adc(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {
    let mut flag = flags.carry as u8;
    let carry = Operand::from_value(&mut flag);

    add(&carry, dst, flags);
    add(src, dst, flags);
}

// TODO: How am I going to handle different sized operands?
pub fn a_dd(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {
    require_if!(src.origin == OpType::MEM, dst.origin == OpType::REG);
    require_if!(src.origin != OpType::IMM, src.len() == dst.len());

    // src.into() + dst.into();

    // Perform nibble addition for adjust flag setting
    let adjust = (src.getU8() & 15u8) + (dst.getU8() & 15u8) > 15;

    // TODO: Find a way to simplify the process of "sign/zero" extension
    // TODO: Find a way to remove the need to have the '$ext' parameter
    // Perform generic implementation (could also use function?)
    macro_rules! add_impl {
        ($type: ty, $ext: expr) => {{
            let src: $type = unsafe{ mem::transmute($ext) };
            let res: $type = *dst.iref();
            let (res, over) = res.overflowing_add(src);
            *dst.mref() = res;
            flags.overflow = over;
        }}
    }

    // Fill in the types as appropriate
    match dst.len() {
        4 => add_impl!(u32, src.getI32()),
        2 => add_impl!(u16, src.getI16()),
        1 => add_impl!(u8, src.getI8()),
        _ => panic!("Unreachable"),
    }

    // Set appropriate flags
    flags.carry = flags.overflow;
    flags.sign = dst.getI32() < 0;
    flags.zero = dst.getU32() == 0;
    flags.parity = (dst.getU32() & 255u32).count_ones() % 2 != 0;
    flags.adjust = adjust;
}

// Simplest implementation for add (possible?)
pub fn add(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {
    require_if!(src.origin == OpType::MEM, dst.origin == OpType::REG);
    require_if!(src.origin != OpType::IMM, src.len() == dst.len());
    require!(dst.origin != OpType::IMM);

    let adjust = (src.getU8() & 15u8) + (dst.getU8() & 15u8) > 15;

    flags.carry = false;
    for i in 0..dst.len() {
        // src.sxb(i) (sign extend if src smaller than i, returns u8)
            // also have a 'zxb' method to perform zero extension
        // dst.at(i) (get Operand view on the i'th byte of dst, returns &mut u8)
        let (res, over1) = dst.at(i).overflowing_add(src.sxb(i));
        let (res, over2) = res.overflowing_add(flags.carry as u8);
        *dst.at(i) = _res;

        flags.carry = over1 || over2;
    }

    flags.sign = dst.getI32() < 0;
    flags.zero = dst.getU32() == 0;
    flags.parity = dst.getU8().count_ones() % 2 != 0;
    flags.adjust = adjust;
}

// Possible problem instructions
    // mul/div, cmps* (these may be trivial actually)
    // rotate, shift (basically require the macro approach)

// cmpsb
pub fn cmpsb(esi: &mut Operand, edi: &mut Operand, mem: &[u8], flags: &mut FlagRegister) {

}

// imul src, dst
pub fn imul(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {

}

// div src
pub fn div(src: &Operand, eax: &mut Operand, edx: &mut Operand, flags: &mut FlagRegister) {
    require!(eax.len() == 4);
    require!(edx.len() == 4);
}

// sub src, dst
pub fn sub(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {
    require_if!(src.origin == OpType::MEM, dst.origin == OpType::REG);
    require_if!(src.origin != OpType::IMM, src.len() == dst.len());
    require!(dst.origin != OpType::IMM);

    let (_, adjust) = (dst.getU8() & 15u8).overflowing_sub(src.getU8() & 15u8);

    flags.carry = false;
    for i in 0..dst.len() {
        let (res, over1) = dst.at(i).overflowing_sub(flags.carry as u8);
        let (res, over2) = res.overflowing_sub(src.sxb(i));
        *dst.at(i) = res;

        flags.carry = over1 || over2;
    }

    flags.sign = dst.getI32() < 0;
    flags.zero = dst.getU32() == 0;
    flags.parity = dst.getU8().count_ones() % 2 != 0;
    flags.adjust = adjust;
}

// TODO: How am I going to handle different sized operands?
pub fn and(src: &Operand, dst: &mut Operand, flags: &mut FlagRegister) {

}
