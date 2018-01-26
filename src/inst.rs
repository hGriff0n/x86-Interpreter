// instructions taken from: https://en.wikipedia.org/wiki/X86_instruction_listings
// implementation reference: http://www.felixcloutier.com/x86/
    // This has highlighting: https://c9x.me/x86/ (but isn't as complete)
    // note this has more instructions
    // that may just be a side effect of the instructions being "out of alphabetical order"

use processor::*;
use std;
use std::mem::transmute;
use std::ops::{ BitOr };

// TODO: Add comments to documentation instructions indicating their assembly forms
// TODO: Improve instructions to enforce sizing and other requirements
// TODO: Implement all instructions

// Considerations
/*
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
// aaa
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
// aad imm8
pub fn aad(al: &mut u8, ah: &mut u8, imm8: u8, flags: &mut FlagRegister) {
    // Perform bcd adjustment
    *al = (*al + (*ah * imm8)) & 0xff;
    *ah = 0;

    // Set appropriate flags
    flags.zero = *al == 0;
    flags.sign = msb8(*al);
    flags.parity = al.count_ones() % 2 != 0;
}
// aad
// FIXME: delete?
pub fn aad_10(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    aad(al, ah, 10, flags);
}
// aam imm8
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
// aam
pub fn aam_10(al: &mut u8, ah: &mut u8, flags: &mut FlagRegister) {
    aam(al, ah, 10, flags);
}
// aas
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
// adc src, dst
pub fn adc(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    // Add the carry flag
    let mut src = *src;
    add(&(flags.carry as u32), dst, flags);

    // Store the result of overflow in case the increment set it
    let over = flags.overflow;
    add(&src, dst, flags);

    // Set appropriate flags
    flags.overflow |= over;
    flags.carry |= over;
}
// add src, dst
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
// and src, dst
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
// call loc
// TODO: Handle near/far/protected/etc. distinctions
pub fn call(loc: &u32, rip: &mut u32, esp: &mut u32, mem: &mut [u8]) {
    push(rip, esp, mem);
    *rip = *loc;
}
// cbw
pub fn cbw(al: &u8, ax: &mut u16) {
    *ax = 0xff & (*al as u16);

    // Sign extend the value of al to fill ax
    if msb8(*al) {
        *ax |= 0xff00;
    }
}
// cdq
pub fn cdq(eax: &u32, edx: &mut u32) {
    *edx = 0;

    if (*eax & (1 << 31)) != 0 {
        *edx = 0xffffffff;
    }
}
// cdqe
pub fn cdqe(eax: &u32, rax: &mut u64) {
    *rax = 0xffffffff & (*eax as u64);

    // Sign extend the value of eax to fill rax
    if (*eax & (1 << 31)) != 0 {
        *rax |= 0xffffffff00000000;
    }
}
// clc
pub fn clc(flags: &mut FlagRegister) {
    flags.carry = false;
}
// cld
pub fn cld(flags: &mut FlagRegister) {
    flags.direction = false;
}
// cli
pub fn cli(flags: &mut FlagRegister) {
    flags.interrupt = false;
}
// cmc
pub fn cmc(flags: &mut FlagRegister) {
    flags.carry ^= true;
}
// cmovCC src, dst
pub fn cmova(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.carry && !flags.zero {
        mov(src, dst);
    }
}
pub fn cmovae(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnc(src, dst, flags);
}
pub fn cmovb(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
}
pub fn cmovbe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
    cmovz(src, dst, flags);
}
pub fn cmovc(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.carry {
        mov(src, dst);
    }
}
pub fn cmove(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovz(src, dst, flags);
}
pub fn cmovg(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.zero && flags.sign == flags.overflow {
        mov(src, dst);
    }
}
pub fn cmovge(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign == flags.overflow {
        mov(src, dst);
    }
}
pub fn cmovl(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign != flags.overflow {
        mov(src, dst);
    }
}
pub fn cmovle(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovz(src, dst, flags);
    cmovl(src, dst, flags);
}
pub fn cmovna(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovbe(src, dst, flags);
}
pub fn cmovnae(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovc(src, dst, flags);
}
pub fn cmovnb(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnc(src, dst, flags);
}
pub fn cmovnbe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmova(src, dst, flags);
}
pub fn cmovnc(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.carry {
        mov(src, dst);
    }
}
pub fn cmovne(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovnz(src, dst, flags);
}
pub fn cmovng(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovle(src, dst, flags);
}
pub fn cmovnge(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovl(src, dst, flags);
}
pub fn cmovnl(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovge(src, dst, flags);
}
pub fn cmovnle(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    cmovg(src, dst, flags);
}
pub fn cmovno(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.overflow {
        mov(src, dst);
    }
}
pub fn cmovnp(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    // NOTE: My parity bit is opposite of felixcloutier
    if flags.parity {
        mov(src, dst);
    }
}
pub fn cmovns(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.sign {
        mov(src, dst);
    }
}
pub fn cmovnz(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        mov(src, dst);
    }
}
pub fn cmovo(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.overflow {
        mov(src, dst);
    }
}
pub fn cmovp(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if !flags.parity {
        mov(src, dst);
    }
}
pub fn cmovpe(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.parity == EVEN {
        mov(src, dst);
    }
}
pub fn cmovpo(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.parity == ODD {
        mov(src, dst);
    }
}
pub fn cmovs(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.sign {
        mov(src, dst);
    }
}
pub fn cmovz(src: &u32, dst: &mut u32, flags: &FlagRegister) {
    if flags.zero {
        mov(src, dst);
    }
}
// cmp fst, snd
pub fn cmp(fst: &u32, snd: &u32, flags: &mut FlagRegister) {
    let mut tmp = *snd;
    sub32(fst, &mut tmp, flags);
}
// cmpsb
pub fn cmpsb(esi: &mut u32, edi: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    // Calculate addresses
    let src1 = *esi as usize;
    let src2 = *edi as usize;

    // Load the values
    let src1: &u8 = unsafe{ transmute(mem[src1..(src1 + 1)].as_ptr()) };
    let src2: &u8 = unsafe{ transmute(mem[src2..(src2 + 1)].as_ptr()) };
    let mut src2 = *src2;
    sub8(src1, &mut src2, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// cmpsw
pub fn cmpsw(esi: &mut u32, edi: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    // Calculate addresses
    let src1 = *esi as usize;
    let src2 = *edi as usize;

    // Load the values
    let src1: &u16 = unsafe{ transmute(mem[src1..(src1 + 2)].as_ptr()) };
    let src2: &u16 = unsafe{ transmute(mem[src2..(src2 + 2)].as_ptr()) };
    let mut src2 = *src2;
    sub16(src1, &mut src2, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// cmpsd
pub fn cmpsd(esi: &mut u32, edi: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    // Calculate addresses
    let src1 = *esi as usize;
    let src2 = *edi as usize;

    // Load the values
    let src1: &u32 = unsafe{ transmute(mem[src1..(src1 + 4)].as_ptr()) };
    let src2: &u32 = unsafe{ transmute(mem[src2..(src2 + 4)].as_ptr()) };
    let mut src2 = *src2;
    cmp(src1, &mut src2, flags);

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// cwd
pub fn cwd(ax: &u16, dx: &mut u16) {
    *dx = 0;

    // Sign extend ax into the dx register
    if msb16(*ax) {
        *dx = 0xffff;
    }
}
// cwde
pub fn cwde(ax: &u16, eax: &mut u32) {
    *eax = 0xffff & (*ax as u32);

    // Sign extend the value of ax to fill eax
    if (*ax & (1 << 15)) != 0 {
        *eax |= 0xffff0000;
    }
}
// daa
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
// das
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
// dec dst
pub fn dec(dst: &mut u32, flags: &mut FlagRegister) {
    let carry = flags.carry;
    sub32(&1, dst, flags);
    flags.carry = carry;
}
// div src
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
// enter size, nesting
pub fn enter(size: u32, nesting: u32, ebp: &mut u32, esp: &mut u32, mem: &mut [u8]) {
    let nesting = nesting % 32;
    push(ebp, esp, mem);
    let temp = *esp;

    if nesting > 0 {
        for _ in 1..nesting {
            *ebp -= 4;
            push(ebp, esp, mem);
        }

        push(&temp, esp, mem);
    }

    *ebp = temp;
    *esp = temp - size;
}
// TODO: Need to figure out how execution engine would work
pub fn hlt() {}
// idiv src
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
// imul src, dst
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
// imul src1, src2, dst
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
// inc dst
pub fn inc(dst: &mut u32, flags: &mut FlagRegister) {
    let carry = flags.carry;
    add(&1, dst, flags);
    flags.carry = carry;
}
// jCC loc
pub fn ja(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmova(&loc, rip, flags);
}
pub fn jae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnc(&loc, rip, flags);
}
pub fn jb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovb(&loc, rip, flags);
}
pub fn jbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovbe(&loc, rip, flags);
}
pub fn jc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovc(&loc, rip, flags);
}
pub fn je(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmove(&loc, rip, flags);
}
pub fn jg(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovg(&loc, rip, flags);
}
pub fn jge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovge(&loc, rip, flags);
}
pub fn jl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovl(&loc, rip, flags);
}
pub fn jle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovle(&loc, rip, flags);
}
pub fn jna(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovna(&loc, rip, flags);
}
pub fn jnae(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnae(&loc, rip, flags);
}
pub fn jnb(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnb(&loc, rip, flags);
}
pub fn jnbe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnbe(&loc, rip, flags);
}
pub fn jnc(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnc(&loc, rip, flags);
}
pub fn jne(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovne(&loc, rip, flags);
}
pub fn jng(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovng(&loc, rip, flags);
}
pub fn jnge(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnge(&loc, rip, flags);
}
pub fn jnl(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnl(&loc, rip, flags);
}
pub fn jnle(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnle(&loc, rip, flags);
}
pub fn jno(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovno(&loc, rip, flags);
}
pub fn jnp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnp(&loc, rip, flags);
}
pub fn jns(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovns(&loc, rip, flags);
}
pub fn jnz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovnz(&loc, rip, flags);
}
pub fn jo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovo(&loc, rip, flags);
}
pub fn jp(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovp(&loc, rip, flags);
}
pub fn jpe(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovpe(&loc, rip, flags);
}
pub fn jpo(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovpo(&loc, rip, flags);
}
pub fn js(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovs(&loc, rip, flags);
}
pub fn jz(loc: u32, rip: &mut u32, flags: &FlagRegister) {
    cmovz(&loc, rip, flags);
}
pub fn jcxz(loc: u32, ecx: &u32, rip: &mut u32) {
    if *ecx == 0 {
        jmp(loc, rip);
    }
}
// jmp
pub fn jmp(loc: u32, rip: &mut u32) {
    mov(&loc, rip);
}
// lahf
pub fn lahf(ah: &mut u8, flags: &FlagRegister) {
    let eflags: u32 = std::convert::From::from(flags);
    *ah = (eflags & 0xff) as u8;
    *ah |= 0x2;
}
// leave
pub fn leave(esp: &mut u32, ebp: &mut u32, mem: &[u8]) {
    *esp = *ebp;
    pop(ebp, esp, mem);
}
// lodsb
pub fn lodsb(esi: &mut u32, al: &mut u8, mem: &[u8], flags: &FlagRegister) {
    // Calculate source addresses
    let src = *esi as usize;

    // Store the byte at the specified addresses
    let src: &u8 = unsafe{ transmute(mem[src..(src + 1)].as_ptr()) };
    *al = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
}
// lodsw
pub fn lodsw(esi: &mut u32, ax: &mut u16, mem: &[u8], flags: &FlagRegister) {
    // Calculate source addresses
    let src = *esi as usize;

    // Store the byte at the specified addresses
    let src: &u16 = unsafe{ transmute(mem[src..(src + 2)].as_ptr()) };
    *ax = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
}
// lodsd
pub fn lodsd(esi: &mut u32, eax: &mut u32, mem: &[u8], flags: &FlagRegister) {
    // Calculate source addresses
    let src = *esi as usize;

    // Store the byte at the specified addresses
    let src: &u32 = unsafe{ transmute(mem[src..(src + 4)].as_ptr()) };
    *eax = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
}
// loop loc
pub fn _loop_(loc: u32, ecx: &mut u32, rip: &mut u32) {
    *ecx -= 1;
    jcxz(loc, ecx, rip);
}
// loopCC loc
pub fn loope(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    loopz(loc, ecx, rip, flags);
}
pub fn loopne(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    loopnz(loc, ecx, rip, flags);
}
pub fn loopnz(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    if !flags.zero {
        _loop_(loc, ecx, rip);
    }
}
pub fn loopz(loc: u32, ecx: &mut u32, rip: &mut u32, flags: &FlagRegister) {
    if flags.zero {
        _loop_(loc, ecx, rip);
    }
}
// mov src, dst
pub fn mov(src: &u32, dst: &mut u32) {
    *dst = *src;
}
// movsx src, dst
pub fn movsx(src: &u16, dst: &mut u32) {
    let src: i16 = unsafe{ transmute(*src) };
    *dst = unsafe{ transmute(src as i32) };
}
// movzx src, dst
pub fn movzx(src: &u16, dst: &mut u32) {
    *dst = *src as u32;
}
// movsb
pub fn movsb(esi: &mut u32, edi: &mut u32, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate addresses
    let src = *esi as usize;
    let dst = *edi as usize;

    // Load and move the address values
    let src: &u8 = unsafe{ transmute(mem[src..(src + 1)].as_ptr()) };
    let dst: &mut u8 = unsafe{ transmute(mem[dst..(dst + 1)].as_mut_ptr()) };
    *dst = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// movsw
pub fn movsw(esi: &mut u32, edi: &mut u32, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate addresses
    let src = *esi as usize;
    let dst = *edi as usize;

    // Load and move the address values
    let src: &u16 = unsafe{ transmute(mem[src..(src + 2)].as_ptr()) };
    let dst: &mut u16 = unsafe{ transmute(mem[dst..(dst + 2)].as_mut_ptr()) };
    *dst = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// movsd
pub fn movsd(esi: &mut u32, edi: &mut u32, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate addresses
    let src = *esi as usize;
    let dst = *edi as usize;

    // Load and move the address values
    let src: &u32 = unsafe{ transmute(mem[src..(src + 4)].as_ptr()) };
    let dst: &mut u32 = unsafe{ transmute(mem[dst..(dst + 4)].as_mut_ptr()) };
    *dst = *src;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *esi += change;
    *edi += change;
}
// mul src
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
// neg dst
pub fn neg(dst: &mut u32, flags: &mut FlagRegister) {
    sub32(&0, dst, flags);
    flags.carry = *dst == 0;
}
// todo: Not sure if this is correct or not
// nop
pub fn nop() {}
// not dst
pub fn not(dst: &mut u32) {
    *dst = !*dst;
}
// or src, dst
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
// pop dst
pub fn pop(dst: &mut u32, esp: &mut u32, mem: &[u8]) {
    let loc = *esp as usize;
    *esp += 4;
    let tmp: &u32 = unsafe{ transmute(mem[loc..(loc + 4)].as_ptr()) };
    *dst = *tmp;
}
// popf
pub fn popf(esp: &mut u32, mem: &[u8], flags: &mut FlagRegister) {
    let mut eflags = 0;
    pop(&mut eflags, esp, mem);
    *flags = eflags.into();
}
// push src
pub fn push(src: &u32, esp: &mut u32, mem: &mut [u8]) {
    *esp -= 4;
    let loc = *esp as usize;

    let mem: &mut u32 = unsafe{ transmute(mem[loc..(loc + 4)].as_mut_ptr()) };
    *mem = *src;
}
// pushf
pub fn pushf(esp: &mut u32, mem: &mut [u8], flags: &mut FlagRegister) {
    let eflags = flags.into();
    push(&eflags, esp, mem);
}
// rcl cnt, dst
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
// rcr cnt, dst
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
// ret
// ret size
pub fn ret(size: u32, rip: &mut u32, esp: &mut u32, mem: &[u8]) {
    pop(rip, esp, mem);
    *rip &= 0xffff;
    *esp += size;
}
// rol src, dst
pub fn rol(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_left(*cnt % 32);
    *dst = res;

    // Set appropriate flags
    if *cnt == 1 {
        flags.overflow = flags.carry ^ msb32(res);
    }
}
// ror cnt, dst
pub fn ror(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let res = dst.rotate_right(*cnt);
    *dst = res;

    // Set appropriate flags
    if *cnt == 1 {
        flags.overflow = ((res & (1 << 30)) != 0) ^ msb32(res);
    }
}
// sahf
pub fn sahf(ah: &u8, flags: &mut FlagRegister) {
    flags.carry = (*ah & 1) != 0;
    flags.parity = (*ah & 4) != 0;
    flags.adjust = (*ah & 16) != 0;
    flags.zero = (*ah & 64) != 0;
    flags.sign = (*ah & 128) != 0;
}
// sal cnt, dst
pub fn sal(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    shl(cnt, dst, flags);
}
// sar cnt, dst
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
// sbb src, dst
// TODO: This probably has the same flags issue as 'adc'
pub fn sbb(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    let tmp = *src + (flags.carry as u32);
    sub32(&tmp, dst, flags);
}
// scasb
pub fn scasb(edi: &mut u32, al: &u8, mem: &[u8], flags: &mut FlagRegister) {
    let src = *edi as usize;
    let src: &u8 = unsafe{ transmute(mem[src..(src + 1)].as_ptr()) };

    let src = *src as u8;
    let mut al = *al;
    sub8(&src, &mut al, flags);

    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// scasw
pub fn scasw(edi: &mut u32, ax: &u16, mem: &[u8], flags: &mut FlagRegister) {
    let src = *edi as usize;
    let src: &u16 = unsafe{ transmute(mem[src..(src + 2)].as_ptr()) };

    let src = *src as u16;
    let mut ax = *ax;
    sub16(&src, &mut ax, flags);

    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// scasd
pub fn scasd(edi: &mut u32, eax: &u32, mem: &[u8], flags: &mut FlagRegister) {
    let src = *edi as usize;
    let src: &u32 = unsafe{ transmute(mem[src..(src + 4)].as_ptr()) };

    let mut eax = *eax;
    cmp(src, &mut eax, flags);

    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// setCC dst
pub fn seta(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmova(&1, dst, flags);
}
pub fn setae(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovae(&1, dst, flags);
}
pub fn setb(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovb(&1, dst, flags);
}
pub fn setbe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovbe(&1, dst, flags);
}
pub fn setc(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovc(&1, dst, flags);
}
pub fn sete(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmove(&1, dst, flags);
}
pub fn setg(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovg(&1, dst, flags);
}
pub fn setge(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovge(&1, dst, flags);
}
pub fn setl(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovl(&1, dst, flags);
}
pub fn setle(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovle(&1, dst, flags);
}
pub fn setna(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovna(&1, dst, flags);
}
pub fn setnae(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnae(&1, dst, flags);
}
pub fn setnb(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnb(&1, dst, flags);
}
pub fn setnbe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnbe(&1, dst, flags);
}
pub fn setnc(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnc(&1, dst, flags);
}
pub fn setne(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovne(&1, dst, flags);
}
pub fn setng(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovng(&1, dst, flags);
}
pub fn setnge(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnge(&1, dst, flags);
}
pub fn setnl(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnl(&1, dst, flags);
}
pub fn setnle(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnle(&1, dst, flags);
}
pub fn setno(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovno(&1, dst, flags);
}
pub fn setnp(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnp(&1, dst, flags);
}
pub fn setns(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovns(&1, dst, flags);
}
pub fn setnz(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovnz(&1, dst, flags);
}
pub fn seto(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovo(&1, dst, flags);
}
pub fn setp(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovp(&1, dst, flags);
}
pub fn setpe(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovpe(&1, dst, flags);
}
pub fn setpo(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovpo(&1, dst, flags);
}
pub fn sets(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovs(&1, dst, flags);
}
pub fn setz(dst: &mut u32, flags: &FlagRegister) {
    *dst = 0;
    cmovz(&1, dst, flags);
}
// shl cnt, dst
pub fn shl(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    *dst <<= *cnt - 1;
    flags.carry = msb32(*dst);
    *dst <<= 1;

    flags.overflow = msb32(*dst) ^ flags.carry;
}
// shr cnt, dst
pub fn shr(cnt: &u32, dst: &mut u32, flags: &mut FlagRegister) {
    *dst >>= *cnt - 1;
    flags.carry = (*dst & 1) != 0;
    *dst >>= 1;

    flags.overflow = false;
}
// stc
pub fn stc(flags: &mut FlagRegister) {
    flags.carry = true;
}
// std
pub fn std(flags: &mut FlagRegister) {
    flags.direction = true;
}
// sti
pub fn sti(flags: &mut FlagRegister) {
    flags.interrupt = true;
}
// stosb
pub fn stosb(edi: &mut u32, al: &u8, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate source addresses
    let dst = *edi as usize;

    // Store the byte at the specified addresses
    let dst: &mut u8 = unsafe{ transmute(mem[dst..(dst + 1)].as_mut_ptr()) };
    *dst = *al;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// stosw
pub fn stosw(edi: &mut u32, eax: &u16, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate source addresses
    let dst = *edi as usize;

    // Store the word at the specified addresses
    let dst: &mut u16 = unsafe{ transmute(mem[dst..(dst + 2)].as_mut_ptr()) };
    *dst = *eax;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// stosd
pub fn stosd(edi: &mut u32, eax: &u32, mem: &mut [u8], flags: &FlagRegister) {
    // Calculate source addresses
    let dst = *edi as usize;

    // Store the dword at the specified addresses
    let dst: &mut u32 = unsafe{ transmute(mem[dst..(dst + 4)].as_mut_ptr()) };
    *dst = *eax;

    // Automatically increment the registers
    let change = (flags.direction as u32) * 2 - 1;
    *edi += change;
}
// sub src, dst
pub fn sub8(src: &u8, dst: &mut u8, flags: &mut FlagRegister) {
    let (_, adjust) = (*dst & 15u8).overflowing_sub(*src & 15u8);
    let (res, over) = dst.overflowing_sub(*src);
    *dst = res;

    // Set the appropriate flags
    flags.carry = over;
    flags.adjust = adjust;
    flags.overflow = over;
    flags.zero = res == 0;
    flags.sign = msb8(res);
    flags.parity = res.count_ones() % 2 != 0;
}
pub fn sub16(src: &u16, dst: &mut u16, flags: &mut FlagRegister) {
    let (_, adjust) = (*dst & 15u16).overflowing_sub(*src & 15u16);
    let (res, over) = dst.overflowing_sub(*src);
    *dst = res;

    // Set the appropriate flags
    flags.carry = over;
    flags.adjust = adjust;
    flags.overflow = over;
    flags.zero = res == 0;
    flags.sign = msb16(res);
    flags.parity = (res & 255u16).count_ones() % 2 != 0;
}
pub fn sub32(src: &u32, dst: &mut u32, flags: &mut FlagRegister) {
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
    flags.sign = msb32(res);
    flags.parity = (res & 255u32).count_ones() % 2 != 0;
}
// test src1, src2
pub fn test(src: &u32, src2: &u32, flags: &mut FlagRegister) {
    let mut tmp = *src2;
    and(src, &mut tmp, flags);
}
// wait
pub fn wait() {
    fwait();
}
// xchg src, dst
pub fn xchg(src: &mut u32, dst: &mut u32) {
    let tmp = *src;
    *src = *dst;
    *dst = tmp;
}
// xor, src, dst
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

// TODO: What does this even do
// pub fn lea() {}

// TODO: Figure out how to repeat/multithread instructions
// TODO: This requires a consistent instruction interface
// pub fn rep() {} // movs/stos/cmps/lods/scas
// pub fn repe() {}
// pub fn repne() {}
// pub fn repnz() {}
// pub fn repz() {}
// pub fn lock() {}

// TODO: Figure out far/near distinction
// pub fn lds() {}
// pub fn les() {}

// TODO: Figure out interrupt handling
// pub fn interrupt() {}
// pub fn into() {}
// pub fn iret() {}

// TODO: Figure out io
// pub fn out() {}
// pub fn _in_() {}


// floating point instructions
// pub fn f2xm1() {}
// pub fn fabs() {}
// pub fn fadd() {}
// pub fn faddp() {}
// pub fn fbld() {}
// pub fn fbstp() {}
// pub fn fchs() {}
// pub fn fclex() {}
// pub fn fcom() {}
// pub fn fcomp() {}
// pub fn fcompp() {}
// pub fn fdecstp() {}
// pub fn fdisi() {}
// pub fn fdiv() {}
// pub fn fdivp() {}
// pub fn fdivr() {}
// pub fn fdivrp() {}
// pub fn feni() {}
// pub fn ffree() {}
// pub fn fiadd() {}
// pub fn ficom() {}
// pub fn ficomp() {}
// pub fn fidiv() {}
// pub fn fidivr() {}
// pub fn fild() {}
// pub fn fimul() {}
// pub fn fincstp() {}
// pub fn finit() {}
// pub fn fist() {}
// pub fn fistp() {}
// pub fn fisub() {}
// pub fn fisubr() {}
// pub fn fld() {}
// pub fn fld1() {}
// pub fn fldcw() {}
// pub fn fldenv() {}
// pub fn fldenvw() {}
// pub fn fldl2e() {}
// pub fn fldl2t() {}
// pub fn fldlg2() {}
// pub fn fldln2() {}
// pub fn fldpi() {}
// pub fn fldz() {}
// pub fn fmul() {}
// pub fn fmulp() {}
// pub fn fnclex() {}
// pub fn fndisi() {}
// pub fn fneni() {}
// pub fn fninit() {}
// pub fn fnop() {}
// pub fn fnsave() {}
// pub fn fnsavew() {}
// pub fn fnstcw() {}
// pub fn fnstenv() {}
// pub fn fnstenvw() {}
// pub fn fnstsw() {}
// pub fn fpatan() {}
// pub fn fprem() {}
// pub fn fptan() {}
// pub fn frndint() {}
// pub fn frstor() {}
// pub fn frstorw() {}
// pub fn fsave() {}
// pub fn fsavew() {}
// pub fn fscale() {}
// pub fn fsqrt() {}
// pub fn fst() {}
// pub fn fstcw() {}
// pub fn fstenv() {}
// pub fn fstenvw() {}
// pub fn fstp() {}
// pub fn fstsw() {}
// pub fn fsub() {}
// pub fn fsubp() {}
// pub fn fsubr() {}
// pub fn fsubrp() {}
// pub fn ftst() {}
pub fn fwait() {}
// pub fn fxam() {}
// pub fn fxch() {}
// pub fn fxtract() {}
// pub fn fyl2x() {}
// pub fn fyl2xp1() {}

// 80287
// pub fn fsetpm() {}

// 80387
// pub fn fcos() {}
// pub fn fldenvd() {}
// pub fn fsaved() {}
// pub fn fstenvd() {}
// pub fn fprem1() {}
// pub fn frstord() {}
// pub fn fsin() {}
// pub fn fsincos() {}
// pub fn fucom() {}
// pub fn fucomp() {}
// pub fn fucompp() {}

// pentium pro
// fcmov ???
// pub fn fcmovb() {}
// pub fn fcmovbe() {}
// pub fn fcmove() {}
// pub fn fcmovnb() {}
// pub fn fcmovnbe() {}
// pub fn fcmovne() {}
// pub fn fcmovnu() {}
// pub fn fcmovu() {}
// pub fn fcomi() {}
// pub fn fcomip() {}
// pub fn fucomi() {}
// pub fn fucomip() {}

// sse, pentium ii
// pub fn fxrstor() {}
// pub fn fxsave() {}

// sse3
// pub fn fisttp() {}


// NOTE: These integer instructions, I probably don't need to implement (I won't be using them)
// pub fn xlat() {}

// 80186/80188
// pub fn bound() {}
// pub fn ins() {}
// pub fn outs() {}
// pub fn popa() {}
// pub fn pusha() {}

// 8028
// pub fn arpl() {}
// pub fn clts() {}
// pub fn lar() {}
// pub fn lgdt() {}
// pub fn lidt() {}
// pub fn lldt() {}
// pub fn lmsw() {}
// pub fn loadall() {}
// pub fn lsl() {}
// pub fn ltr() {}
// pub fn sgdt() {}
// pub fn sidt() {}
// pub fn sldt() {}
// pub fn smsw() {}
// pub fn str() {}
// pub fn verr() {}
// pub fn verw() {}

// 80386
// pub fn bsf() {}
// pub fn bsr() {}
// pub fn bt() {}
// pub fn btc() {}
// pub fn btr() {}
// pub fn bts() {}
// pub fn insd() {}
// pub fn iretd() {}
// pub fn iretf() {}
// pub fn jecxz() {}
// pub fn lfs() {}
// pub fn lgs() {}
// pub fn lss() {}
// pub fn loopw() {}
// pub fn loopew() {}
// pub fn loopnew() {}
// pub fn loopnzw() {}
// pub fn loopzw() {}
// pub fn outsd() {}
// pub fn popad() {}
// pub fn popfd() {}
// pub fn pushad() {}
// pub fn pushfd() {}
// pub fn shld() {}
// pub fn shrd() {}

// 80486
// pub fn bswap() {}
// pub fn cmpxchg() {}
// pub fn invd() {}
// pub fn invlpg() {}
// pub fn wbinvd() {}
// pub fn xadd() {}

// pentium
// pub fn cpuid() {}
// pub fn cmpxchg8b() {}
// pub fn rdmsr() {}
// pub fn rdtsc() {}
// pub fn wrmsr() {}
// pub fn rsm() {}

// pentium mmx
// pub fn rdpmc() {}

// amd k6 / pentium ii
// pub fn syscall() {}
// pub fn sysret() {}

// pentium pro
// pub fn ud2() {}

// sse
// pub fn maskmovq() {}
// pub fn movntps() {}
// pub fn movntq() {}
// pub fn prefetcht0() {}
// pub fn prefetcht1() {}
// pub fn prefetcht2() {}
// pub fn prefetchnta() {}
// pub fn sfence() {}

// sse2
// pub fn clflush() {}
// pub fn lfence() {}
// pub fn mfence() {}
// pub fn movnti() {}
// pub fn pause() {}

// sse3
// pub fn monitor() {}
// pub fn mwait() {}

// sse4.2
// pub fn crc32() {}

// x86-64
// pub fn cqo() {}
// pub fn cmpsq() {}
// pub fn cmpxchg16b() {}
// pub fn iretq() {}
// pub fn jrcxz() {}
// pub fn lodsq() {}
// pub fn movsxd() {}
// pub fn popfq() {}
// pub fn pushfq() {}
// pub fn rdtscp() {}
// pub fn scasq() {}
// pub fn stosq() {}
// pub fn swapgs() {}

// amd-c
// pub fn clgi() {}
// pub fn invlpga() {}
// // mov(CRn)
// // mov(DRn)
// pub fn skinit() {}
// pub fn stgi() {}
// pub fn vmload() {}
// pub fn vmmcall() {}
// pub fn vmrun() {}
// pub fn vmsave() {}

// VT-x
// pub fn vmptrld() {}
// pub fn vmptrst() {}
// pub fn vmclear() {}
// pub fn vmread() {}
// pub fn vmwrite() {}
// pub fn vmcall() {}
// pub fn vmlaunch() {}
// pub fn vmresume() {}
// pub fn vmxoff() {}
// pub fn vmxon() {}

// abm
// pub fn lzcnt() {}
// pub fn popcnt() {}

// bmi1
// pub fn andn() {}
// pub fn bextr() {}
// pub fn blsi() {}
// pub fn blsmsk() {}
// pub fn blsr() {}
// pub fn tzcnt() {}

// bmi2
// pub fn bzhi() {}
// pub fn mulx() {}
// pub fn pdep() {}
// pub fn pext() {}
// pub fn rorx() {}
// pub fn sarx() {}
// pub fn shrx() {}
// pub fn shlx() {}

// tbm
// pub fn blcfill() {}
// pub fn blci() {}
// pub fn blcic() {}
// pub fn blcmask() {}
// pub fn blcs() {}
// pub fn blsfill() {}
// pub fn blsic() {}
// pub fn t1mskc() {}
// pub fn tzmsk() {}

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
