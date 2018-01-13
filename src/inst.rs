// instructions taken from: https://en.wikipedia.org/wiki/X86_instruction_listings
// implementation reference: http://www.felixcloutier.com/x86/
    // This has highlighting: https://c9x.me/x86/ (but isn't as complete)
    // note this has more instructions
    // that may just be a side effect of the instructions being "out of alphabetical order"

use processor::*;
use std::io::*;

// TODO: Implement all instructions
// TODO: Come up with a better abstraction
    // Needs to enforce sizing, etc. demands
// Considerations
/*
    How to perform sign extension of intermediate results?
    How to handle the instructions where small registers are needed
        and the ones where any registers are usable
    related: How to enforce size matching restrictions / behavior
        modification restrictions (ie. as with div)
 */

fn msb(num: u8) -> bool {
    num & (1 << 7) != 0
}
fn msb(num: u16) -> bool {
    num & (1 << 15) != 0
}
fn msb(num: u32) -> bool {
    num & (1 << 32) != 0
}

// integer instructions
// Correct
pub fn aaa(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    // Set appropriate flags
    flags.adjust |= (*al & 0xf) > 9;
    flags.carry = flags.adjust;

    // Perform bcd adjustment
    if flags.adjust {
        *al += 6;
        *ah += 1;
    }

    *al &= 0xf;
}
// Correct
pub fn aad(al: &mut u8, ah: &mut u8, imm8: u8, flags: &mut FlagRegister) {
    // Perform bcd adjustment
    *al = (*al + (*ah * imm8)) & 0xff
    *ah = 0;

    // Set appropriate flags
    flags.zero = (*al == 0);
    flags.sign = msb(*al);
    flags.parity = al.count_ones() % 2 != 0;
}
pub fn aad(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    aad(al, ah, 10, flags);
}
// Correct
pub fn aam(al: &mut u8, ah: &mut u8, imm8: u8, flags: &mut FlagRegister) {
    // Perform bcd adjustment
    let temp = *al;
    *ah = temp / imm8;
    *al = temp % imm8;

    // Set appropriate flags
    flags.zero = (*al == 0);
    flags.sign = msb(*al);
    flags.parity = al.count_ones() % 2 != 0;
}
pub fn aam(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    aam(al, ah, 10, flags);
}
// Correct
pub fn aas(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    // Set appropriate flags
    flags.adjust |= (*al & 0xf) > 9;
    flags.carry = flags.adjust;

    // Perform bcd adjustment
    if flags.adjust {
        *al -= 6;
        *ah -= 1;
    }

    *al &= 0xf;
}

// Correct
pub fn adc(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let src = *src + (flags.carry as u32);
    add(&src, dst, &flags);
}
// Correct
pub fn add(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Perform nibble addition for adjust flag setting
    let adjust = (*dst & 15u32) + (*src & 15u32) > 15;

    // Perform actual addition operation
    let (res, over) = dst.overflowing_add(*src);
    *dst = res;

    // Set the appropriate flags
    flags.carry = over;
    flags.adjust = adjust;
    flags.overflow = over;
    flags.zero = (res == 0);
    flags.sign = msb(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// Correct
pub fn and(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Perform and operation
    let res = *dst & *src;
    *dst = res;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.adjust = false;
    flags.zero = (res == 0);
    flags.sign = msb(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// TODO: Implement
pub fn call() {}
// Correct
pub fn cbw(al: &u8, ax: &mut u16) {
    *ax = 0xff & (*al as u16);

    // Sign extend the value of al to fill ax
    if msb(*al) {
        *ax |= 0xff00;
    }
}
// Correct
pub fn clc(flags: &mut FlagRegister) {
    flags.carry = false;
}
// Correct
pub fn cld(flags: &mut FlagRegister) {
    flags.direction = false;
}
// Correct
pub fn cli(flags: &mut FlagRegister) {
    flags.interrupt = false;
}
// Correct
pub fn cmc(flags: &mut FlagRegister) {
    flags.carry ^= true;
}
// Correct
pub fn cmp(fst: &u32, snd: &u32, flags: &mut FlagRegister) {
    let mut tmp = *snd;
    sub(fst, &mut tmp, flags);
}
// TODO: Implement
pub fn cmps() {
    // These require loading from memory
}
// TODO: Implement
pub fn cmpsb() {}
// TODO: Implement
pub fn cmpsw() {}
// Correct
pub fn cwd(ax: &u16, dx: &mut u16) {
    *dx = 0;

    // Sign extend ax into the dx register
    if msb(*ax) != 0 {
        *dx = 0xffff;
    }
}
// TODO: Implement
pub fn daa(al: &mut u8, flags: &mut FlagRegister) {}
// TODO: Implement
pub fn das(al: &mut u8, flags: &mut FlagRegister) {}
// Correct
pub fn dec(dst: &mut u32, flags: &mut FlagRegister) {
    let carry = flags.carry;
    sub(&1, dst, flags);
    flags.carry = carry;
}
// Correct
pub fn div(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    let num: u64 = *eax;
    num |= (*edx as u64) << 32;

    let res = num / *src;
    *edx = num / *src;
    *eax = res;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.adjust = false;
    flags.zero = (res == 0);
    flags.sign = msb(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// Not found on felixcloutier
// pub fn esc() {}
// TODO: Implement
pub fn hlt() {}
// Correct
pub fn idiv(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    // Convert operands to the correct values
    let num: u64 = *eax;
    num |= (*edx as u64) << 32;
    let num: i64 = unsafe{ std::mem::transmute(num) };
    let div: i64 = unsafe{ std::mem::transmute(*src) };

    // Perform the operation
    let (res, over) = num.overflowing_div(div);
    *edx = unsafe { mem::transmute(num.wrapping_rem(div)) };
    *eax = unsafe { mem::transmute(res) };

    let res = *eax;

    // Set appropriate flags
    flags.carry = over;
    flags.adjust = adjust;
    flags.overflow = over;
    flags.zero = (res == 0);
    flags.sign = res > 0;
    flags.parity = (*eax & 255u32).count_ones() % 2 != 0;
}
// TODO: This is supposed to be signed multiplication
pub fn imul(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = (*dst as u64) * (*src as u64);
    *dst = res;

    // Set appropriate flags
    flags.carry = (res == *dst);
    flags.overflow = flags.carry;
    flags.zero = (res == 0);
}
// TODO: This is supposed to be signed multiplication
pub fn imul(src1: &u32, src2: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = (*src1 as u64) * (*src2 as u64);
    *dst = res;

    // Set appropriate flags
    flags.carry = (res == *dst);
    flags.overflow = flags.carry;
    flags.zero = (res == 0);
}
// TODO: Figure out i/o
pub fn _in_() {}
// Correct
pub fn inc(dst: &mut u32, flags: &mut FlagRegister) {
    let carry = flags.carry;
    add(&1, dst, flags);
    flags.carry = carry;
}
// TODO: Figure out interrupt handling
pub fn interrupt() {}
// TODO: Figure out interrupt handling
pub fn into() {}
// TODO: Figure out call procedure
pub fn iret() {}
// Correct
pub fn ja(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.carry && !flags.zero {
        jmp(loc, rip);
    }
}
// Correct
pub fn jae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jnc(loc, rip, flags);
}
// Correct
pub fn jb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jc(loc, rip, flags);
}
// Correct
pub fn jbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jc(loc, rip, flags);
    jz(loc, rip, flags);
}
// Correct
pub fn jc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.carry {
        jmp(loc, rip);
    }
}
// Correct
pub fn je(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jz(loc, rip, flags);
}
// Correct
pub fn jg(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.zero && flags.sign == flags.overflow {
        jmp(loc, rip);
    }
}
// Correct
pub fn jge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.sign == flags.overflow {
        jmp(loc, rip);
    }
}
// Correct
pub fn jl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.sign != flags.overflow {
        jmp(loc, rip);
    }
}
// Correct
pub fn jle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jz(loc, rip, flags);
    jl(loc, rip, flags);
}
// Correct
pub fn jna(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jbe(loc, rip, flags);
}
// Correct
pub fn jnae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jc(loc, rip, flags);
}
// Correct
pub fn jnb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jnc(loc, rip, flags);
}
// Correct
pub fn jnbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    ja(loc, rip, flags);
}
// Correct
pub fn jnc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.carry {
        jmp(loc, rip);
    }
}
// Correct
pub fn jne(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        jmp(loc, rip);
    }
}
// Correct
pub fn jng(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jle(loc, rip, flags);
}
// Correct
pub fn jnge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jl(loc, rip, flags);
}
// Correct
pub fn jnl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jge(loc, rip, flags);
}
// Correct
pub fn jnle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    jg(loc, rip, flags);
}
// Correct
pub fn jno(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.overflow {
        jmp(loc, rip);
    }
}
// Correct
pub fn jnp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    // NOTE: My parity bit is opposite of felixcloutier
    if flags.parity {
        jmp(loc, rip);
    }
}
// Correct
pub fn jns(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.sign {
        jmp(loc, rip);
    }
}
// Correct
pub fn jnz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        jmp(loc, rip);
    }
}
// Correct
pub fn jo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.overflow {
        jmp(loc, rip);
    }
}
// Correct
pub fn jp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    // NOTE: My parity bit is opposite of felixcloutier
    if !flags.parity {
        jmp(loc, rip);
    }
}
// Correct
pub fn jpe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.parity == EVEN {
        jmp(loc, rip);
    }
}
// Correct
pub fn jpo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.parity == ODD {
        jmp(loc, rip);
    }
}
// Correct
pub fn js(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.sign {
        jmp(loc, rip);
    }
}
// Correct
pub fn jz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.zero {
        jmp(loc, rip);
    }
}
// Correct
pub fn jcxz(loc: u32, ecx: &u32, rip: &mut u32) {
    if *ecx == 0 {
        jmp(loc, rip);
    }
}
// Correct
pub fn jmp(loc: u32, rip: &mut u32) {
    *rip = loc;
}
pub fn lahf(ah: &mut u8, flags: &FlagRegister) {
    *ah = 2;
    if flags.carry {
        *ah |= 1;
    }
    if flags.parity {
        *ah |= 4;
    }
    if flags.adjust {
        *ah |= 16;
    }
    if flags.zero {
        *ah |= 64;
    }
    if flags.sign {
        *ah |= 128;
    }
}
// TODO: Figure out memory addressing
pub fn lds() {}
// TODO: Figure out memory stuff
pub fn lea(src: &u32, dst: &mut u32) {}
// TODO: Figure out memory addressing
pub fn les() {}
// TODO: Figure out multithreading
pub fn lock() {}
// TODO: Figure out memory addressing
pub fn lodsb() {}
// TODO: Figure out memory addressing
pub fn lodsw() {}
// Correct
pub fn _loop_(loc: u32, ecx: &mut u32, rip: &mut u32) {
    *ecx -= 1;
    jcxz(loc, ecx, rip);
}
// Correct
pub fn loope(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    loopz(loc, ecx, rip, flags);
}
// Correct
pub fn loopne(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    loopnz(loc, ecx, rip, flags);
}
// Correct
pub fn loopnz(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        _loop_(loc, ecx, rip);
    }
}
// Correct
pub fn loopz(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.zero {
        _loop_(loc, ecx, rip);
    }
}
// Correct
pub fn mov(src: &u32, dst: &mut u32) {
    *dst = *src;
}
// TODO: Figure out how memory works
pub fn movsb() {}
// TODO: Figure out how memory works
pub fn movsw() {}
// TODO: Ensure this operates correctly
pub fn mul(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    let res = (*src as u64) * (*eax as u64);
    
    // Split into the two registers
    let low: u32 = res & 0xffffffff;
    let high: u32 = (res >> 32) & 0xffffffff;

    // Assign to registers
    *edx = high;
    *eax = low;

    // Set appropriate flags
    flags.overflow = (high == 0);
    flags.carry = (high == 0);
    flags.adjust = false;
    flags.zero = (res == 0);
    flags.sign = msb(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// Correct
pub fn neg(dst: &mut u32, flags: &mut FlagRegister) {
    sub(&0, dst, flags);
    flags.carry = (*dst == 0);
}
// TODO: Not sure if this is correct or not
pub fn nop() {}
// TODO: Not sure if `not` does what I expect
pub fn not(dst: &mut u32, flags: &mut FlagRegister) {
    *dst = dst.not();
}
// Correct
pub fn or(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.bitor(*src);
    *dst = res;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.adjust = false;
    flags.zero = (res == 0);
    flags.sign = msb(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// TODO: Figure out io
pub fn out() {}
// TODO: Figure out memory
pub fn pop(dst: &mut u32, esp: &mut u32, mem: &[u8]) {
    let loc = *esp;
    *dst = unsafe{ std::mem::transmute(mem[loc..(loc + 4)]) };
    *esp += 4;
}
// TODO: Set flags, figure out memory
pub fn popf(esp: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    let mut eflags = 0;
    pop(&mut eflags, esp, mem);

    // TODO: Set flags accordingly
}
// TODO: Figure out memory
pub fn push(src: &u32, esp: &mut u32, mem: &mut [u8]) {
    let loc = *esp;
    *esp += 4;
    let mem: &mut u32 = unsafe{ std::mem::transmute(mem[loc..(loc + 4)]) };
    *mem = *src;
}
// TODO: Set flags, Figure out memory
pub fn pushf(esp: &mut u32, mem: &mut [u8], flags: &mut FlagRegister) {
    let mut eflags = 0;
    // TODO: Set flags accordingly
    push(&eflags, esp, mem);
}
// Correct
pub fn rcl(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let count = *cnt & 0x1f;
    let dest = *dst;
    
    while count != 0 {
        let carry = msb(*dst);
        dest = (dest << 1) + (flags.carry as u32);
        flags.carry = carry;
        count -= 1;
    }

    if *cnt == 1 {
        flags.overflow = msb(dest) ^ carry;
    }
    
    *dst = dest;
}
// Correct
pub fn rcr(count: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let count = *count & 0x1f;
    let dest = *dst;

    if count == 1 {
        flags.overflow = msb(dest) ^ flags.carry;
    }

    while count != 0 {
        let carry = (dest & 1) != 0;
        dest = (dest >> 1) + ((flags.carry as u32) << 31);
        flags.carry = carry;
        count -= 1;
    }
}
// TODO: Figure out how to repeat instructions
pub fn rep() {} // movs/stos/cmps/lods/scas
pub fn repe() {}
pub fn repne() {}
pub fn repnz() {}
pub fn repz() {}
// TODO: Figure out calling semantics
pub fn ret() {}
pub fn retn() {}
pub fn retf() {}
// TODO: Figure out if rust's semantics are correct
pub fn rol(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_left(*src % 32);
    *dst = res;

    // Set appropriate flags
    if *src == 1 {
        flags.overflow = flags.carry ^ msb(res);
    }
}
// TODO: Figure out if rust's semantics are correct
pub fn ror(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_right(*src);
    *dst = res;

    // Set appropriate flags
    if *src == 1 {
        flags.overflow = ((res & (1 << 30)) != 0) ^ msb(res);
    }
}
// Correct
pub fn sahf(ah: &u8, flags: &mut FlagRegister) {
    flags.carry = (*ah & 1) != 0;
    flags.parity = (*ah & 4) != 0;
    flags.adjust = (*ah & 16) != 0;
    flags.zero = (*ah & 64) != 0;
    flags.sign = (*ah & 128) != 0;
}
// TODO: Implement shift instructions
pub fn sal() {}
// TODO: Implement shift instructions
pub fn sar() {}
// Correct
pub fn sbb(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let tmp = *src + (flags.carry as u32);
    sub(&tmp, &dst, &flags);
}
// TODO: Figure out memory
pub fn scasb() {}
// TODO: Figure out memory
pub fn scasw() {}
// TODO: Find out what rust's semantics are
pub fn shl(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = *dst << *src;

    
}
// TODO: Find out what rust's semantics are
pub fn shr(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = *dst >> *src;


}
// Correct
pub fn stc(flags: &mut FlagRegister) {
    flags.carry = true;
}
// Correct
pub fn std(flags: &mut FlagRegister) {
    flags.direction = true;
}
// Correct
pub fn sti(flags: &mut FlagRegister) {
    flags.interrupt = true;
}
// TODO: Figure out memory
pub fn stosb() {}
// TODO: Figure out memory
pub fn stosw() {}
// Correct
pub fn sub(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Perform nibble addition for adjust flag setting
    let (_, adjust) = (*dst & 15u32).overflowing_sub(*src & 15u32);

    // Perform actual addition operation
    let (res, over) = dst.overflowing_sub(*src);
    *dst = res;

    // Set the appropriate flags
    flags.carry = over;
    flags.adjust = adjust;
    flags.overflow = over;
    flags.zero = (res == 0);
    flags.sign = (res & (1 << 31)) != 0;
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// Correct
pub fn test(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let mut tmp = *dst;
    and(src, &mut tmp, flags);
}
// TODO: Implement correct argument forwarding
pub fn wait() {
    fwait();
}
// Correct
pub fn xchg(src: &mut u32, dst: &mut u32, flags: &FlagRegister) {
    let tmp = *src;
    *src = *dst;
    *dst = tmp;
}
// TODO: Figure out tables
pub fn xlat() {}
// Correct
pub fn xor(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = *dst ^ *src;
    *dst = res;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.zero = (res == 0);
    flags.sign = (res & (1 << 31)) != 0;
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}

// 80186/80188
pub fn bound() {}
pub fn enter() {}
pub fn ins() {}
pub fn leave() {}
pub fn outs() {}
pub fn popa() {}
pub fn pusha() {}

// 8028
pub fn arpl() {}
pub fn clts() {}
pub fn lar() {}
pub fn lgdt() {}
pub fn lidt() {}
pub fn lldt() {}
pub fn lmsw() {}
pub fn loadall() {}
pub fn lsl() {}
pub fn ltr() {}
pub fn sgdt() {}
pub fn sidt() {}
pub fn sldt() {}
pub fn smsw() {}
pub fn str() {}
pub fn verr() {}
pub fn verw() {}

// 80386
pub fn bsf() {}
pub fn bsr() {}
pub fn bt() {}
pub fn btc() {}
pub fn btr() {}
pub fn bts() {}
// Correct
pub fn cdq(eax: &u32, edx: &mut u32) {
    *edx = 0;

    if (*eax & (1 << 31)) != 0 {
        *edx = 0xffffffff;
    }
}
pub fn cmpsd() {}
// Correct
pub fn cwde(ax: &u16: eax: &mut u32) {
    *eax = 0xffff & (*ax as u32);

    // Sign extend the value of ax to fill eax
    if (*ax & (1 << 15)) != 0 {
        *eax |= 0xffff0000;
    }
}
pub fn insd() {}
pub fn iretd() {}
pub fn iretf() {}
pub fn jecxz() {}
pub fn lfs() {}
pub fn lgs() {}
pub fn lss() {}
pub fn lodsd() {}
pub fn loopw() {}
pub fn loopew() {}
pub fn loopnew() {}
pub fn loopnzw() {}
pub fn loopzw() {}
pub fn movsd() {}
pub fn movsx() {}
pub fn movzx() {}
pub fn outsd() {}
pub fn popad() {}
pub fn popfd() {}
pub fn pushad() {}
pub fn pushfd() {}
pub fn scasd() {}
pub fn seta() {}
pub fn setae() {}
pub fn setb() {}
pub fn setbe() {}
pub fn setc() {}
pub fn sete() {}
pub fn setg() {}
pub fn setge() {}
pub fn setl() {}
pub fn setle() {}
pub fn setna() {}
pub fn setnae() {}
pub fn setnb() {}
pub fn setnbe() {}
pub fn setnc() {}
pub fn setne() {}
pub fn setng() {}
pub fn setnge() {}
pub fn setnl() {}
pub fn setnle() {}
pub fn setno() {}
pub fn setnp() {}
pub fn setns() {}
pub fn setnz() {}
pub fn seto() {}
pub fn setp() {}
pub fn setpe() {}
pub fn setpo() {}
pub fn sets() {}
pub fn setz() {}
pub fn shld() {}
pub fn shrd() {}
pub fn stosd() {}

// 80486
pub fn bswap() {}
pub fn cmpxchg() {}
pub fn invd() {}
pub fn invlpg() {}
pub fn wbinvd() {}
pub fn xadd() {}

// pentium
pub fn cpuid() {}
pub fn cmpxchg8b() {}
pub fn rdmsr() {}
pub fn rdtsc() {}
pub fn wrmsr() {}
pub fn rsm() {}

// pentium mmx
pub fn rdpmc() {}

// amd k6 / pentium ii
pub fn syscall() {}
pub fn sysret() {}

// pentium pro
pub fn cmova() {}
pub fn cmovae() {}
pub fn cmovb() {}
pub fn cmovbe() {}
pub fn cmovc() {}
pub fn cmove() {}
pub fn cmovg() {}
pub fn cmovge() {}
pub fn cmovl() {}
pub fn cmovle() {}
pub fn cmovna() {}
pub fn cmovnae() {}
pub fn cmovnb() {}
pub fn cmovnbe() {}
pub fn cmovnc() {}
pub fn cmovne() {}
pub fn cmovng() {}
pub fn cmovnge() {}
pub fn cmovnl() {}
pub fn cmovnle() {}
pub fn cmovno() {}
pub fn cmovnp() {}
pub fn cmovns() {}
pub fn cmovnz() {}
pub fn cmovo() {}
pub fn cmovp() {}
pub fn cmovpe() {}
pub fn cmovpo() {}
pub fn cmovs() {}
pub fn cmovz() {}
pub fn ud2() {}

// sse
pub fn maskmovq() {}
pub fn movntps() {}
pub fn movntq() {}
pub fn prefetcht0() {}
pub fn prefetcht1() {}
pub fn prefetcht2() {}
pub fn prefetchnta() {}
pub fn sfence() {}

// sse2
pub fn clflush() {}
pub fn lfence() {}
pub fn mfence() {}
pub fn movnti() {}
pub fn pause() {}

// sse3
pub fn monitor() {}
pub fn mwait() {}

// sse4.2
pub fn crc32() {}

// x86-64
// Correct
pub fn cdqe(eax: &u32, rax: &mut u64) {
    *rax = 0xffffffff & (*eax as u64);

    // Sign extend the value of eax to fill rax
    if (*eax & (1 << 31)) != 0 {
        *rax |= 0xffffffff00000000;
    }
}
pub fn cqo() {}
pub fn cmpsq() {}
pub fn cmpxchg16b() {}
pub fn iretq() {}
pub fn jrcxz() {}
pub fn lodsq() {}
pub fn movsxd() {}
pub fn popfq() {}
pub fn pushfq() {}
pub fn rdtscp() {}
pub fn scasq() {}
pub fn stosq() {}
pub fn swapgs() {}

// amd-c
pub fn clgi() {}
pub fn invlpga() {}
// mov(CRn)
// mov(DRn)
pub fn skinit() {}
pub fn stgi() {}
pub fn vmload() {}
pub fn vmmcall() {}
pub fn vmrun() {}
pub fn vmsave() {}

// VT-x
pub fn vmptrld() {}
pub fn vmptrst() {}
pub fn vmclear() {}
pub fn vmread() {}
pub fn vmwrite() {}
pub fn vmcall() {}
pub fn vmlaunch() {}
pub fn vmresume() {}
pub fn vmxoff() {}
pub fn vmxon() {}

// abm
pub fn lzcnt() {}
pub fn popcnt() {}

// bmi1
pub fn andn() {}
pub fn bextr() {}
pub fn blsi() {}
pub fn blsmsk() {}
pub fn blsr() {}
pub fn tzcnt() {}

// bmi2
pub fn bzhi() {}
pub fn mulx() {}
pub fn pdep() {}
pub fn pext() {}
pub fn rorx() {}
pub fn sarx() {}
pub fn shrx() {}
pub fn shlx() {}

// tbm
pub fn blcfill() {}
pub fn blci() {}
pub fn blcic() {}
pub fn blcmask() {}
pub fn blcs() {}
pub fn blsfill() {}
pub fn blsic() {}
pub fn t1mskc() {}
pub fn tzmsk() {}

// floating point instructions
pub fn f2xm1() {}
pub fn fabs() {}
pub fn fadd() {}
pub fn faddp() {}
pub fn fbld() {}
pub fn fbstp() {}
pub fn fchs() {}
pub fn fclex() {}
pub fn fcom() {}
pub fn fcomp() {}
pub fn fcompp() {}
pub fn fdecstp() {}
pub fn fdisi() {}
pub fn fdiv() {}
pub fn fdivp() {}
pub fn fdivr() {}
pub fn fdivrp() {}
pub fn feni() {}
pub fn ffree() {}
pub fn fiadd() {}
pub fn ficom() {}
pub fn ficomp() {}
pub fn fidiv() {}
pub fn fidivr() {}
pub fn fild() {}
pub fn fimul() {}
pub fn fincstp() {}
pub fn finit() {}
pub fn fist() {}
pub fn fistp() {}
pub fn fisub() {}
pub fn fisubr() {}
pub fn fld() {}
pub fn fld1() {}
pub fn fldcw() {}
pub fn fldenv() {}
pub fn fldenvw() {}
pub fn fldl2e() {}
pub fn fldl2t() {}
pub fn fldlg2() {}
pub fn fldln2() {}
pub fn fldpi() {}
pub fn fldz() {}
pub fn fmul() {}
pub fn fmulp() {}
pub fn fnclex() {}
pub fn fndisi() {}
pub fn fneni() {}
pub fn fninit() {}
pub fn fnop() {}
pub fn fnsave() {}
pub fn fnsavew() {}
pub fn fnstcw() {}
pub fn fnstenv() {}
pub fn fnstenvw() {}
pub fn fnstsw() {}
pub fn fpatan() {}
pub fn fprem() {}
pub fn fptan() {}
pub fn frndint() {}
pub fn frstor() {}
pub fn frstorw() {}
pub fn fsave() {}
pub fn fsavew() {}
pub fn fscale() {}
pub fn fsqrt() {}
pub fn fst() {}
pub fn fstcw() {}
pub fn fstenv() {}
pub fn fstenvw() {}
pub fn fstp() {}
pub fn fstsw() {}
pub fn fsub() {}
pub fn fsubp() {}
pub fn fsubr() {}
pub fn fsubrp() {}
pub fn ftst() {}
pub fn fwait() {}
pub fn fxam() {}
pub fn fxch() {}
pub fn fxtract() {}
pub fn fyl2x() {}
pub fn fyl2xp1() {}

// 80287
pub fn fsetpm() {}

// 80387
pub fn fcos() {}
pub fn fldenvd() {}
pub fn fsaved() {}
pub fn fstenvd() {}
pub fn fprem1() {}
pub fn frstord() {}
pub fn fsin() {}
pub fn fsincos() {}
pub fn fucom() {}
pub fn fucomp() {}
pub fn fucompp() {}

// pentium pro
// fcmov ???
pub fn fcmovb() {}
pub fn fcmovbe() {}
pub fn fcmove() {}
pub fn fcmovnb() {}
pub fn fcmovnbe() {}
pub fn fcmovne() {}
pub fn fcmovnu() {}
pub fn fcmovu() {}
pub fn fcomi() {}
pub fn fcomip() {}
pub fn fucomi() {}
pub fn fucomip() {}

// sse, pentium ii
pub fn fxrstor() {}
pub fn fxsave() {}

// sse3
pub fn fisttp() {}

/*
// simd (note: some of these are duplicates for different sizes)
emm() {}
mov() {}
mov() {}
packssd() {}
packssw() {}
packusw() {}
padd() {}
padd() {}
padd() {}
padd() {}
padds() {}
padds() {}
paddus() {}
paddus() {}
pan() {}
pand() {}
po() {}
pxo() {}
pcmpeq() {}
pcmpeq() {}
pcmpeq() {}
pcmpgt() {}
pcmpgt() {}
pcmpgt() {}
pmaddw() {}
pmull() {}
psll() {}
psll() {}
psll() {}
psra() {}
psra() {}
psrl() {}
psrl() {}
psrl() {}
psub() {}
psub() {}
psub() {}
psubs() {}
psubs() {}
psubus() {}
psubus() {}
punpckhb() {}
punpckhw() {}
punpckhd() {}
punpcklb() {}
punpcklw() {}
punpckld() {}

// mmx+/ss() {}
pshuf() {}
pinsr() {}
pextr() {}
pmovmsk() {}
pminu() {}
pmaxu() {}
pavg() {}
pavg() {}
pmulhu() {}
pmins() {}
pmaxs() {}
psadb() {}

// sse() {}
psub() {}
pmulud() {}

// sse() {}
psign() {}
psign() {}
psign() {}
pshuf() {}
pmulhrs() {}
pmaddubs() {}
phsub() {}
phsubs() {}
phsub() {}
phadds() {}
phadd() {}
phadd() {}
pabs() {}
pabs() {}
pabs() {}

// 3dnow() {}
femm() {}
pavgus() {}
pf2i() {}
pfac() {}
pfad() {}
pfcmpe() {}
pfcmpg() {}
pfcmpg() {}
pfma() {}
pfmi() {}
pfmu() {}
pfrc() {}
pfrcpit() {}
pfrcpit() {}
pfrsqit() {}
pfrsqr() {}
pfsu() {}
pfsub() {}
pi2f() {}
pmulhr() {}
prefetc() {}
prefetch() {}

// athlon, k6-2() {}
pf2i() {}
pfnac() {}
pfpnac() {}
pi2f() {}
pswap() {}

// geode g() {}
pfrsqrt() {}
pfrcp() {}

// sse instruction() {}
andp() {}
andnp() {}
orp() {}
xorp() {}
movup() {}
movs() {}
movlp() {}
movhlp() {}
unpcklp() {}
unpckhp() {}
movhp() {}
movlhp() {}
movap() {}
movmskp() {}
cvtpi2p() {}
cvtsi2s() {}
cvttps2p() {}
cvttss2s() {}
cvtps2p() {}
cvtss2s() {}
ucomis() {}
comis() {}
sqrtp() {}
sqrts() {}
rsqrtp() {}
rsqrts() {}
rcpp() {}
rcps() {}
addp() {}
adds() {}
mulp() {}
muls() {}
subp() {}
subs() {}
minp() {}
mins() {}
divp() {}
divs() {}
maxp() {}
maxs() {}
ldmxcs() {}
stmxcs() {}
cmpp() {}
cmps() {}
shufp() {}

// pentium () {}
movap() {}
movntp() {}
movhp() {}
movlp() {}
movup() {}
movmskp() {}
movs() {}
addp() {}
adds() {}
divp() {}
divs() {}
maxp() {}
maxs() {}
minp() {}
mins() {}
mulp() {}
muls() {}
sqrtp() {}
sqrts() {}
subp() {}
subs() {}
andp() {}
andnp() {}
orp() {}
xorp() {}
cmpp() {}
comis() {}
ucmois() {}
shufp() {}
unpckhp() {}
unpcklp() {}
cvtdq2p() {}
cvtdq2p() {}
cvtpd2d() {}
cvtpd2p() {}
cvtpd2p() {}
cvtpi2p() {}
cvtps2d() {}
cvtps2p() {}
cvtsd2s() {}
cvtsd2s() {}
cvtsi2s() {}
cvtss2s() {}
cvttpd2d() {}
cvttpd2p() {}
cvttps2d() {}
cvttsd2si */

// TODO: SSE2 SIMD integer instructions