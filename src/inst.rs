// instructions taken from: https://en.wikipedia.org/wiki/X86_instruction_listings

// integer
pub fn aaa() {}
pub fn aad() {}
pub fn aam() {}
pub fn aas() {}
pub fn adc() {}
pub fn add(src: &u32, dst: &mut u32) {
    *dst += *src;
}
pub fn and() {}
pub fn call() {}
pub fn cbw() {}
pub fn clc() {}
pub fn cld() {}
pub fn cli() {}
pub fn cmc() {}
pub fn cmp() {}
pub fn cmpsb() {}
pub fn cmpsw() {}
pub fn cwd() {}
pub fn daa() {}
pub fn das() {}
pub fn dec() {}
pub fn div() {}
pub fn esc() {}
pub fn hlt() {}
pub fn idiv() {}
pub fn imul() {}
pub fn in() {}
pub fn inc() {}
pub fn interrupt() {}
pub fn into() {}
pub fn iret() {}
pub fn ja() {}
pub fn jae() {}
pub fn jb() {}
pub fn jbe() {}
pub fn jc() {}
pub fn je() {}
pub fn jg() {}
pub fn jge() {}
pub fn jl() {}
pub fn jle() {}
pub fn jna() {}
pub fn jnae() {}
pub fn jnb() {}
pub fn jnbe() {}
pub fn jnc() {}
pub fn jne() {}
pub fn jng() {}
pub fn jnge() {}
pub fn jnl() {}
pub fn jnle() {}
pub fn jno() {}
pub fn jnp() {}
pub fn jns() {}
pub fn jnz() {}
pub fn jo() {}
pub fn jp() {}
pub fn jpe() {}
pub fn jpo() {}
pub fn js() {}
pub fn jz() {}
pub fn jcxz() {}
pub fn jmp() {}
pub fn lahf() {}
pub fn lds() {}
pub fn lea() {}
pub fn les() {}
pub fn lock() {}
pub fn lodsb() {}
pub fn lodsw() {}
pub fn loop() {}
pub fn loope() {}
pub fn loopne() {}
pub fn loopnz() {}
pub fn loopz() {}
pub fn mov() {}
pub fn movsb() {}
pub fn movsw() {}
pub fn mul() {}
pub fn neg() {}
pub fn nop() {}
pub fn not() {}
pub fn or() {}
pub fn out() {}
pub fn pop() {}
pub fn popf() {}
pub fn push() {}
pub fn pushf() {}
pub fn rcl() {}
pub fn rcr() {}
pub fn rep() {} // movs/stos/cmps/lods/scas
pub fn repe() {}
pub fn repne() {}
pub fn repnz() {}
pub fn repz() {}
pub fn ret() {}
pub fn retn() {}
pub fn retf() {}
pub fn rol() {}
pub fn ror() {}
pub fn sahf() {}
pub fn sal() {}
pub fn sar() {}
pub fn sbb() {}
pub fn scasb() {}
pub fn scasw() {}
pub fn shl() {}
pub fn shr() {}
pub fn stc() {}
pub fn std() {}
pub fn sti() {}
pub fn stosb() {}
pub fn stosw() {}
pub fn sub() {}
pub fn test() {}
pub fn wait() {}
pub fn xchg() {}
pub fn xlat() {}
pub fn xor() {}

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
pub fn cdq() {}
pub fn cmpsd() {}
pub fn cwde() {}
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
pub fn cmova() {}
pub fn cmov() {}
pub fn cmovb() {}
pub fn cmov() {}
pub fn cmov() {}
pub fn cmov() {}
pub fn cmovg() {}
pub fn cmov() {}
pub fn cmovl() {}
pub fn cmovn() {}
pub fn cmovna() {}
pub fn cmovn() {}
pub fn cmovnb() {}
pub fn cmovn() {}
pub fn cmovn() {}
pub fn cmovn() {}
pub fn cmovng() {}
pub fn cmovn() {}
pub fn cmovnl() {}
pub fn cmovn() {}
pub fn cmovn() {}
pub fn cmovn() {}
pub fn cmovn() {}
pub fn cmov() {}
pub fn cmov() {}
pub fn cmovp() {}
pub fn cmovp() {}
pub fn cmov() {}
pub fn cmov() {}
pub fn ud() {}

// sse
pub fn maskmov() {}
pub fn movntp() {}
pub fn movnt() {}
pub fn prefetcht() {}
pub fn prefetcht() {}
pub fn prefetcht() {}
pub fn prefetchnt() {}
pub fn sfenc() {}

// sse2
pub fn clflus() {}
pub fn lfenc() {}
pub fn mfenc() {}
pub fn movnt() {}
pub fn paus() {}

// sse3
pub fn monito() {}
pub fn mwai() {}

// sse4.2
pub fn crc3() {}

// x86-64
pub fn cdq() {}
pub fn cq() {}
pub fn cmps() {}
pub fn cmpxchg16() {}
pub fn iret() {}
pub fn jrcx() {}
pub fn lods() {}
pub fn movsx() {}
pub fn popf() {}
pub fn pushf() {}
pub fn rdtsc() {}
pub fn scas() {}
pub fn stos() {}
pub fn swapg() {}

// amd-c
pub fn clg() {}
pub fn invlpg() {}
// mov(CRn)
// mov(DRn)
pub fn skini() {}
pub fn stg() {}
pub fn vmloa() {}
pub fn vmmcal() {}
pub fn vmru() {}
pub fn vmsav() {}

// VT-x
pub fn vmptrl() {}
pub fn vmptrs() {}
pub fn vmclea() {}
pub fn vmrea() {}
pub fn vmwrit() {}
pub fn vmcal() {}
pub fn vmlaunc() {}
pub fn vmresum() {}
pub fn vmxof() {}
pub fn vmxo() {}

// abm
pub fn lzcn() {}
pub fn popcn() {}

// bmi1
pub fn and() {}
pub fn bext() {}
pub fn bls() {}
pub fn blsms() {}
pub fn bls() {}
pub fn tzcn() {}

// bmi2
pub fn bzh() {}
pub fn mul() {}
pub fn pde() {}
pub fn pex() {}
pub fn ror() {}
pub fn sar() {}
pub fn shr() {}
pub fn shl() {}

// tbm
pub fn bext() {}
pub fn blcfil() {}
pub fn blc() {}
pub fn blci() {}
pub fn blcmas() {}
pub fn blc() {}
pub fn blsfil() {}
pub fn blsi() {}
pub fn t1msk() {}
pub fn tzms() {}

// floating point
pub fn f2xm() {}
pub fn fab() {}
pub fn fad() {}
pub fn fadd() {}
pub fn fbl() {}
pub fn fbst() {}
pub fn fch() {}
pub fn fcle() {}
pub fn fco() {}
pub fn fcom() {}
pub fn fcomp() {}
pub fn fdecst() {}
pub fn fdis() {}
pub fn fdi() {}
pub fn fdiv() {}
pub fn fdiv() {}
pub fn fdivr() {}
pub fn fen() {}
pub fn ffre() {}
pub fn fiad() {}
pub fn fico() {}
pub fn ficom() {}
pub fn fidi() {}
pub fn fidiv() {}
pub fn fil() {}
pub fn fimu() {}
pub fn fincst() {}
pub fn fini() {}
pub fn fis() {}
pub fn fist() {}
pub fn fisu() {}
pub fn fisub() {}
pub fn fl() {}
pub fn fld() {}
pub fn fldc() {}
pub fn flden() {}
pub fn fldenv() {}
pub fn fldl2() {}
pub fn fldl2() {}
pub fn fldlg() {}
pub fn fldln() {}
pub fn fldp() {}
pub fn fld() {}
pub fn fmu() {}
pub fn fmul() {}
pub fn fncle() {}
pub fn fndis() {}
pub fn fnen() {}
pub fn fnini() {}
pub fn fno() {}
pub fn fnsav() {}
pub fn fnsave() {}
pub fn fnstc() {}
pub fn fnsten() {}
pub fn fnstenv() {}
pub fn fnsts() {}
pub fn fpata() {}
pub fn fpre() {}
pub fn fpta() {}
pub fn frndin() {}
pub fn frsto() {}
pub fn frstor() {}
pub fn fsav() {}
pub fn fsave() {}
pub fn fscal() {}
pub fn fsqr() {}
pub fn fs() {}
pub fn fstc() {}
pub fn fsten() {}
pub fn fstenv() {}
pub fn fst() {}
pub fn fsts() {}
pub fn fsu() {}
pub fn fsub() {}
pub fn fsub() {}
pub fn fsubr() {}
pub fn fts() {}
pub fn fwai() {}
pub fn fxa() {}
pub fn fxc() {}
pub fn fxtrac() {}
pub fn fyl2() {}
pub fn fyl2xp() {}

// 80287
pub fn fsetp() {}

// 80387
pub fn fco() {}
pub fn fldenv() {}
pub fn fsave() {}
pub fn fstenv() {}
pub fn fprem() {}
pub fn frstor() {}
pub fn fsi() {}
pub fn fsinco() {}
pub fn fstenv() {}
pub fn fuco() {}
pub fn fucom() {}
pub fn fucomp() {}

// pentium pro
// fcmov ???
pub fn fcmov() {}
pub fn fcmovb() {}
pub fn fcmov() {}
pub fn fcmovn() {}
pub fn fcmovnb() {}
pub fn fcmovn() {}
pub fn fcmovn() {}
pub fn fcmov() {}
pub fn fcom() {}
pub fn fcomi() {}
pub fn fucom() {}
pub fn fucomi() {}

// sse, pentium ii
pub fn fxrsto() {}
pub fn fxsav() {}

// sse3
pub fn fistt() {}

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