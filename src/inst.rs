// instructions taken from: https://en.wikipedia.org/wiki/X86_instruction_listings
// implementation reference: http://www.felixcloutier.com/x86/
    // This has highlighting: https://c9x.me/x86/ (but isn't as complete)
    // note this has more instructions
    // that may just be a side effect of the instructions being "out of alphabetical order"

use processor::*;
use std; 
use std::mem::transmute;
use std::ops::{ BitOr, Not };

// TODO: Implement call/ret instructions
// TODO: Implement movsx, movzx instructions
// TODO: Go through testing to ensure all instructions are correct
    // NOTE: After this I can start working on the interpreter abstractions

// TODO: Add comments to documentation instructions indicating their assembly forms
// TODO: Improve instructions to enforce sizing and other requirements


// TODO: Implement all instructions
// Considerations
/*
    How to perform sign extension of intermediate results?
    How to handle the instructions where small registers are needed
        and the ones where any registers are usable
    related: How to enforce size matching restrictions / behavior
        modification restrictions (ie. as with div)
    How to handle segment selector and all the different addressing modes
 */

fn msb8(num: u8) -> bool {
    num & (1 << 7) != 0
}
fn msb16(num: u16) -> bool {
    num & (1 << 15) != 0
}
fn msb32(num: u32) -> bool {
    num & (1 << 31) != 0
}
fn msb64(num: u64) -> bool {
    num & (1 << 63) != 0
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
    *al = (*al + (*ah * imm8)) & 0xff;
    *ah = 0;

    // Set appropriate flags
    flags.zero = *al == 0;
    flags.sign = msb8(*al);
    flags.parity = al.count_ones() % 2 != 0;
}
pub fn aad_10(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    aad(al, ah, 10, flags);
}
// Correct
pub fn aam(al: &mut u8, ah: &mut u8, imm8: u8, flags: &mut FlagRegister) {
    // Perform bcd adjustment
    let temp = *al;
    *ah = temp / imm8;
    *al = temp % imm8;

    // Set appropriate flags
    flags.zero = *al == 0;
    flags.sign = msb8(*al);
    flags.parity = al.count_ones() % 2 != 0;
}
pub fn aam_10(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
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
    add(&src, dst, flags);
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
    flags.zero = res == 0;
    flags.sign = msb32(res);
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
    flags.zero = res == 0;
    flags.sign = msb32(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// TODO: Figure out how to resolve the "near"/"far"/"protected"/etc.
pub fn call() {

}
// Correct
pub fn cbw(al: &u8, ax: &mut u16) {
    *ax = 0xff & (*al as u16);

    // Sign extend the value of al to fill ax
    if msb8(*al) {
        *ax |= 0xff00;
    }
}// Correct
pub fn cdq(eax: &u32, edx: &mut u32) {
    *edx = 0;

    if (*eax & (1 << 31)) != 0 {
        *edx = 0xffffffff;
    }
}
// Correct
pub fn cdqe(eax: &u32, rax: &mut u64) {
    *rax = 0xffffffff & (*eax as u64);

    // Sign extend the value of eax to fill rax
    if (*eax & (1 << 31)) != 0 {
        *rax |= 0xffffffff00000000;
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
pub fn cmova(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.carry && !flags.zero {
        mov(src, dst);
    }
}
// Correct
pub fn cmovae(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnc(src, dst, flags);
}
// Correct
pub fn cmovb(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
}
// Correct
pub fn cmovbe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
    cmovz(src, dst, flags);
}
// Correct
pub fn cmovc(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.carry {
        mov(src, dst);
    }
}
// Correct
pub fn cmove(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovz(src, dst, flags);
}
// Correct
pub fn cmovg(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.zero && flags.sign == flags.overflow {
        mov(src, dst);
    }
}
// Correct
pub fn cmovge(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign == flags.overflow {
        mov(src, dst);
    }
}
// Correct
pub fn cmovl(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign != flags.overflow {
        mov(src, dst);
    }
}
// Correct
pub fn cmovle(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovz(src, dst, flags);
    cmovl(src, dst, flags);
}
// Correct
pub fn cmovna(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovbe(src, dst, flags);
}
// Correct
pub fn cmovnae(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
}
// Correct
pub fn cmovnb(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnc(src, dst, flags);
}
// Correct
pub fn cmovnbe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmova(src, dst, flags);
}
// Correct
pub fn cmovnc(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.carry {
        mov(src, dst);
    }
}
// Correct
pub fn cmovne(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnz(src, dst, flags);
}
// Correct
pub fn cmovng(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovle(src, dst, flags);
}
// Correct
pub fn cmovnge(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovl(src, dst, flags);
}
// Correct
pub fn cmovnl(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovge(src, dst, flags);
}
// Correct
pub fn cmovnle(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovg(src, dst, flags);
}
// Correct
pub fn cmovno(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.overflow {
        mov(src, dst, flags);
    }
}
// Correct
pub fn cmovnp(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    // NOTE: My parity bit is opposite of felixcloutier
    if flags.parity {
        mov(src, dst, flags);
    }
}
// Correct
pub fn cmovns(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.sign {
        mov(src, dst, flags);
    }
}
// Correct
pub fn cmovnz(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        mov(src, dst);
    }
}
// Correct
pub fn cmovo(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.overflow {
        mov(src, dst);
    }
}
// Correct
pub fn cmovp(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.parity {
        mov(src, dst);
    }
}
// Correct
pub fn cmovpe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.parity == EVEN {
        mov(src, dst);
    }
}
// Correct
pub fn cmovpo(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.parity == ODD {
        mov(src, dst);
    }
}
// Correct
pub fn cmovs(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign {
        mov(src, dst);
    }
}
// Correct
pub fn cmovz(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.zero {
        mov(src, dst);
    }
}
// Correct
pub fn cmp(fst: &u32, snd: &u32, flags: &mut FlagRegister) {
    let mut tmp = *snd;
    sub(fst, &mut tmp, flags);
}
// Correct
// TODO: Implement cmpsb, cmpsw, cmpsd in terms of cmps
// pub fn cmps(ds: &u32, esi: &mut u32, es: &u32, edi: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
pub fn cmps(esi: &mut u32, edi: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    // Calculate source addresses
    let src1 = *esi as u64;
    // src1 |= (*ds as u64) << 32;
    let src2 = *edi as u64;
    // src2 |= (*es as u64) << 32;

    let src1 = src1 as usize;
    let src2 = src2 as usize;

    // Load the memory at the specified addresses
    let src1: &u32 = unsafe{ transmute(mem[src1..(src1 + 4)].as_ptr()) };
    let src2: &u32 = unsafe{ transmute(mem[src2..(src2 + 4)].as_ptr()) };
    cmp(src1, src2, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;

    // size 8: cmpsb
    // size 16: cmpsw
    // size 32: cmpsd
}
// Correct
pub fn cwd(ax: &u16, dx: &mut u16) {
    *dx = 0;

    // Sign extend ax into the dx register
    if msb16(*ax) {
        *dx = 0xffff;
    }
}
// Correct
pub fn cwde(ax: &u16, eax: &mut u32) {
    *eax = 0xffff & (*ax as u32);

    // Sign extend the value of ax to fill eax
    if (*ax & (1 << 15)) != 0 {
        *eax |= 0xffff0000;
    }
// TODO: Figure out if the documentation is accurate
pub fn daa(al: &mut u8, flags: &mut FlagRegister) {
    flags.adjust |= (*al & 0xf) > 9;
    flags.carry |= *al > 0x99;

    if flags.adjust {
        *al += 6;
    }
    if flags.carry {
        *al += 0x60;
    }
}
// TODO: Figure out if the documentation is accurate
pub fn das(al: &mut u8, flags: &mut FlagRegister) {
    flags.adjust |= (*al & 0xf) > 9;
    flags.carry |= *al > 0x99;

    if flags.adjust {
        *al -= 6;
    }

    if flags.carry {
        *al -= 0x60;
    }
}
// Correct
pub fn dec(dst: &mut u32, flags: &mut FlagRegister) {
    let carry = flags.carry;
    sub(&1, dst, flags);
    flags.carry = carry;
}
// Correct
pub fn div(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    let mut num = *eax as u64;
    num |= (*edx as u64) << 32;

    let res = num / (*src as u64);
    *edx = (num % (*src as u64)) as u32;
    *eax = res as u32;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.adjust = false;
    flags.zero = res == 0;
    flags.sign = msb64(res);
    flags.parity = (res & 255u64).count_ones() % 2 != 0;
}
// Not found on felixcloutier
// pub fn esc() {}
// TODO: Need to figure out how execution engine would work
pub fn hlt() {}
// Correct
pub fn idiv(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    // Convert operands to the correct values
    let mut num = *eax as u64;
    num |= (*edx as u64) << 32;

    let num: i64 = unsafe{ transmute(num) };
    let div: i32 = unsafe{ transmute(*src) };
    let div = div as i64;

    // Perform the operation
    let (res, over) = num.overflowing_div(div);
    *edx = unsafe{ transmute(num.wrapping_rem(div) as i32) };
    *eax = unsafe{ transmute(res as i32) };

    let res = *eax;

    // Set appropriate flags
    flags.carry = over;
    flags.overflow = over;
    flags.zero = res == 0;
    flags.sign = res > 0;
    flags.parity = (*eax & 255u32).count_ones() % 2 != 0;
}
// Correct
pub fn imul(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Convert to signed integers
    let isrc: i32 = unsafe{ transmute(*src) };
    let isrc = isrc as i64;
    let idst: i32 = unsafe{ transmute(*dst) };
    let idst = idst as i64;
    
    // Perform multiplication
    let res = isrc * idst;
    *dst = unsafe{ transmute(res as i32) };

    // Set appropriate flags
    flags.carry = res == ((res as i32) as i64);
    flags.overflow = flags.carry;
    flags.zero = res == 0;
}
// Correct
pub fn imul_trip(src1: &u32, src2: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Convert to signed integers
    let isrc1: i32 = unsafe{ transmute(*src1) };
    let isrc1 = isrc1 as i64;
    let isrc2: i32 = unsafe{ transmute(*src2) };
    let isrc2 = isrc2 as i64;
    
    // Perform multiplication
    let res = isrc1 * isrc2;
    *dst = unsafe{ transmute(res as i32) };

    // Set appropriate flags
    flags.carry = res == ((res as i32) as i64);
    flags.overflow = flags.carry;
    flags.zero = res == 0;
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
    cmova(&loc, rip, flags);
}
// Correct
pub fn jae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnc(&loc, rip, flags);
}
// Correct
pub fn jb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovb(&loc, rip, flags);
}
// Correct
pub fn jbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovbe(&loc, rip, flags);
}
// Correct
pub fn jc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovc(&loc, rip, flags);
}
// Correct
pub fn je(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmove(&loc, rip, flags);
}
// Correct
pub fn jg(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovg(&loc, rip, flags);
}
// Correct
pub fn jge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovge(&loc, rip, flags);
}
// Correct
pub fn jl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovl(&loc, rip, flags);
}
// Correct
pub fn jle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovle(&loc, rip, flags);
}
// Correct
pub fn jna(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovna(&loc, rip, flags);
}
// Correct
pub fn jnae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnae(&loc, rip, flags);
}
// Correct
pub fn jnb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnb(&loc, rip, flags);
}
// Correct
pub fn jnbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnbe(&loc, rip, flags);
}
// Correct
pub fn jnc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnc(&loc, rip, flags);
}
// Correct
pub fn jne(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovne(&loc, rip, flags);
}
// Correct
pub fn jng(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovng(&loc, rip, flags);
}
// Correct
pub fn jnge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnge(&loc, rip, flags);
}
// Correct
pub fn jnl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnl(&loc, rip, flags);
}
// Correct
pub fn jnle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnle(&loc, rip, flags);
}
// Correct
pub fn jno(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovno(&loc, rip, flags);
}
// Correct
pub fn jnp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnp(&loc, rip, flags);
}
// Correct
pub fn jns(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovns(&loc, rip, flags);
}
// Correct
pub fn jnz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnz(&loc, rip, flags);
}
// Correct
pub fn jo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovo(&loc, rip, flags);
}
// Correct
pub fn jp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovp(&loc, rip, flags);
}
// Correct
pub fn jpe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovpe(&loc, rip, flags);
}
// Correct
pub fn jpo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovpo(&loc, rip, flags);
}
// Correct
pub fn js(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovs(&loc, rip, flags);
}
// Correct
pub fn jz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovz(&loc, rip, flags);
}
// Correct
pub fn jcxz(loc: u32, ecx: &u32, rip: &mut u32) {
    if *ecx == 0 {
        jmp(loc, rip);
    }
}
// Correct
pub fn jmp(loc: u32, rip: &mut u32) {
    mov(&loc, rip);
}
// Correct
pub fn lahf(ah: &mut u8, flags: &FlagRegister) {
    let eflags: u32 = std::convert::From::from(flags);
    *ah = (eflags & 0xff) as u8;
    *ah |= 0x2;
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
// TODO: Implement
pub fn movsx() {}
// TODO: Implement
pub fn movzx() {}
// Correct
// TODO: Implement movsb, movsw, movsd
// pub fn movs(ds: &u32, esi: &mut u32, es: &u32, edi: &mut u32, mem: &mut [u8], flags: &mut FlagRegister) {
pub fn movs(esi: &mut u32, edi: &mut u32, mem: &mut [u8], flags: &mut FlagRegister) {
    // Calculate source addresses
    let src = *esi as u64;
    // src |= (*ds as u64) << 32;
    let dst = *edi as u64;
    // dst |= (*es as u64) << 32;

    let src = src as usize;
    let dst = dst as usize;

    // Load the memory at the specified addresses
    // TODO: Does this work properly?
    let src: &u32 = unsafe{ transmute(mem[src..(src + 4)].as_ptr()) };
    let dst: &u32 = unsafe{ transmute(mem[dst..(dst + 4)].as_mut_ptr()) };
    cmp(src, dst, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;

    // size 8: movsb
    // size 16: movsw
    // size 32: movsd
}
// Correct
pub fn mul(src: &u32, eax: &mut u32, edx: &mut u32, flags: &mut FlagRegister) {
    // Perform multiplication
    let res = (*src as u64) * (*eax as u64);
    *edx = ((res >> 32) & 0xffffffff) as u32;
    *eax = (res & 0xffffffff) as u32;

    // Set appropriate flags
    flags.overflow = *edx == 0;
    flags.carry = *edx == 0;
    flags.adjust = false;
    flags.zero = res == 0;
    flags.sign = msb32(*edx);
    flags.parity = (res & 255u64).count_ones() % 2 != 0;
}
// Correct
pub fn neg(dst: &mut u32, flags: &mut FlagRegister) {
    sub(&0, dst, flags);
    flags.carry = *dst == 0;
}
// todo: Not sure if this is correct or not
pub fn nop() {}
// Correct
pub fn not(dst: &mut u32, flags: &mut FlagRegister) {
    *dst = !*dst;
}
// Correct
pub fn or(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.bitor(*src);
    *dst = res;

    // Set appropriate flags
    flags.overflow = false;
    flags.carry = false;
    flags.adjust = false;
    flags.zero = res == 0;
    flags.sign = msb32(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// TODO: Figure out io
pub fn out() {}
// Correct
pub fn pop(dst: &mut u32, esp: &mut u32, mem: &[u8]) {
    let loc = *esp as usize;
    *esp += 4;
    let tmp: &u32 = unsafe{ transmute(mem[loc..(loc + 4)].as_ptr()) };
    *dst = *tmp;
}
// Correct
pub fn popf(esp: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    let mut eflags = 0;
    pop(&mut eflags, esp, mem);
    *flags = eflags.into();
}
// Correct
pub fn push(src: &u32, esp: &mut u32, mem: &mut [u8]) {
    *esp -= 4;
    let loc = *esp as usize;

    let mem: &mut u32 = unsafe{ transmute(mem[loc..(loc + 4)].as_mut_ptr()) };
    *mem = *src;
}
// Correct
pub fn pushf(esp: &mut u32, mem: &mut [u8], flags: &mut FlagRegister) {
    let eflags = flags.into();
    push(&eflags, esp, mem);
}
// Correct
pub fn rcl(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let mut count = *cnt & 0x1f;
    let mut dest = *dst;
    
    while count != 0 {
        let carry = msb32(dest);
        dest = (dest << 1) + (flags.carry as u32);
        flags.carry = carry;
        count -= 1;
    }

    if *cnt == 1 {
        flags.overflow = msb32(dest) ^ flags.carry;
    }
    
    *dst = dest;
}
// Correct
pub fn rcr(count: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let mut count = *count & 0x1f;
    let mut dest = *dst;

    if count == 1 {
        flags.overflow = msb32(dest) ^ flags.carry;
    }

    while count != 0 {
        let carry = (dest & 1) != 0;
        dest = (dest >> 1) + ((flags.carry as u32) << 31);
        flags.carry = carry;
        count -= 1;
    }

    *dst = dest;
}
// TODO: Figure out how to repeat instructions
// TODO: This requires a consistent instruction interface
pub fn rep() {} // movs/stos/cmps/lods/scas
pub fn repe() {}
pub fn repne() {}
pub fn repnz() {}
pub fn repz() {}
// TODO: Figure out calling semantics
pub fn ret() {}
pub fn retn() {}
pub fn retf() {}
// Correct
pub fn rol(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_left(*src % 32);
    *dst = res;

    // Set appropriate flags
    if *src == 1 {
        flags.overflow = flags.carry ^ msb32(res);
    }
}
// Correct
pub fn ror(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_right(*src);
    *dst = res;

    // Set appropriate flags
    if *src == 1 {
        flags.overflow = ((res & (1 << 30)) != 0) ^ msb32(res);
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
// Correct
pub fn sal(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    shl(cnt, dst, flags);
}
// Correct
pub fn sar(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    flags.carry = (*dst & 1) != 0;
    *dst >>= 1;
    *dst &= !(1 << 31);         // ensure the msb is a 0

    if *cnt > 1 {
        let cnt = *cnt - 1;
        sar(&cnt, dst, flags);
    } else {
        flags.overflow = msb32(*dst);
    }
}
// Correct
pub fn sbb(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let tmp = *src + (flags.carry as u32);
    sub(&tmp, dst, flags);
}
// Correct
// TODO: Implement scasb, scasw, scasd in terms of scas
pub fn scas(edi: &mut u32, eax: &u32, mem: &[u8], flags: &mut FlagRegister) {
    // Calculate source addresses
    let src = *edi as u64;
    // src1 |= (*es as u64) << 32;

    let src = src as usize;

    // Load the memory at the specified addresses
    let src: &u32 = unsafe{ transmute(mem[src..(src + 4)].as_ptr()) };
    let mut eax = *eax;
    cmp(src, &mut eax, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;

    // size 8: scasb
    // size 16: scasw
    // size 32: scasd
}
// Correct
pub fn seta(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmova(&1, dst, flags);
}
// Correct
pub fn setae(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovae(&1, dst, flags);
}
// Correct
pub fn setb(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovb(&1, dst, flags);
}
// Correct
pub fn setbe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovbe(&1, dst, flags);
}
// Correct
pub fn setc(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovc(&1, dst, flags);
}
// Correct
pub fn sete(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmove(&1, dst, flags);
}
// Correct
pub fn setg(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovg(&1, dst, flags);
}
// Correct
pub fn setge(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovge(&1, dst, flags);
}
// Correct
pub fn setl(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovl(&1, dst, flags);
}
// Correct
pub fn setle(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovle(&1, dst, flags);
}
// Correct
pub fn setna(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovna(&1, dst, flags);
}
// Correct
pub fn setnae(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnae(&1, dst, flags);
}
// Correct
pub fn setnb(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnb(&1, dst, flags);
}
// Correct
pub fn setnbe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnbe(&1, dst, flags);
}
// Correct
pub fn setnc(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnc(&1, dst, flags);
}
// Correct
pub fn setne(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovne(&1, dst, flags);
}
// Correct
pub fn setng(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovng(&1, dst, flags);
}
// Correct
pub fn setnge(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnge(&1, dst, flags);
}
// Correct
pub fn setnl(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnl(&1, dst, flags);
}
// Correct
pub fn setnle(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnle(&1, dst, flags);
}
// Correct
pub fn setno(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovno(&1, dst, flags);
}
// Correct
pub fn setnp(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnp(&1, dst, flags);
}
// Correct
pub fn setns(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovns(&1, dst, flags);
}
// Correct
pub fn setnz(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnz(&1, dst, flags);
}
// Correct
pub fn seto(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovo(&1, dst, flags);
}
// Correct
pub fn setp(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovp(&1, dst, flags);
}
// Correct
pub fn setpe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovpe(&1, dst, flags);
}
// Correct
pub fn setpo(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovpo(&1, dst, flags);
}
// Correct
pub fn sets(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovs(&1, dst, flags);
}
// Correct
pub fn setz(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovz(&1, dst, flags);
}
// Correct
pub fn shl(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    *dst <<= *cnt - 1;
    flags.carry = msb32(*dst);
    *dst <<= 1;

    flags.overflow = msb32(*dst) ^ flags.carry;
}
// Correct
pub fn shr(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    *dst >>= *cnt - 1;
    flags.carry = (*dst & 1) != 0;
    *dst >>= 1;

    flags.overflow = false;
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
// Correct
// TODO: Implement stosb, stosw, stosd in terms of stos
pub fn stos(edi: &mut u32, eax: &u32, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate source addresses
    let src = *edi as u64;
    // src1 |= (*es as u64) << 32;

    let src = src as usize;

    // Load the memory at the specified addresses
    let src: &mut u32 = unsafe{ transmute(mem[src..(src + 4)].as_mut_ptr()) };
    mov(eax, src);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;

    // size 8: stosb
    // size 16: stosw
    // size 32: stosd
}
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
    flags.zero = res == 0;
    flags.sign = (res & (1 << 31)) != 0;
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// Correct
pub fn test(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let mut tmp = *dst;
    and(src, &mut tmp, flags);
}
// Correct
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
    flags.zero = res == 0;
    flags.sign = (res & (1 << 31)) != 0;
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}

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


// NOTE: These integer instructions, I probably don't need to implement (I won't be using them)
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
pub fn outsd() {}
pub fn popad() {}
pub fn popfd() {}
pub fn pushad() {}
pub fn pushfd() {}
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