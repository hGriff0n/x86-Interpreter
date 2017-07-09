
use nom::*;
use std::str;
use ximpl::{Command, Argument};

// Nom resources
// http://hermanradtke.com/2016/08/08/introduction-to-nom-rust-parsing-combinator-framework.html
// http://stevedonovan.github.io/rust-gentle-intro/nom-intro.html
// http://siciarz.net/24-days-rust-nom-part-1/
// http://spw15.langsec.org/papers/couprie-nom.pdf
// https://maikklein.github.io/post/nom/

// Entry-point function
pub fn x86_instruction(input: &str) -> IResult<&[u8], Command> {
    _x86_instruction(input.as_bytes())
}
pub fn label(input: &str) -> IResult<&[u8], Command> {
    _label(input.as_bytes())
}


// Parser implementation
named!(_x86_instruction<Command>, ws!(alt!(
    directive | no_arg_inst | one_arg_inst | two_arg_inst | _label
)));
named!(directive<Command>, ws!(
    do_parse!(
        tag!(".") >>
        val: map_res!(alpha, str::from_utf8) >>
        (Command::Directive(val.to_string()))
    )
));
named!(_label<Command>, ws!(
    do_parse!(
        val: map_res!(alphanumeric, str::from_utf8) >>
        tag!(":") >>
        (Command::Label(val.to_string()))
    )
));
named!(one_arg_inst<Command>, ws!(
    do_parse!(
        mne: ws!(one_arg_mnemonic) >>
        arg: operand >>
        (Command::OneArg(mne.to_string(), arg))
    )
));
named!(two_arg_inst<Command>, ws!(
    do_parse!(
        mne: ws!(two_arg_mnemonic) >>
        arg1: operand >>
        tag!(",") >>
        arg2: operand >>
        (Command::TwoArg(mne.to_string(), arg1, arg2))
    )
));
named!(no_arg_inst<Command>, ws!(
    do_parse!(
        mne: no_arg_mnemonic >>
        (Command::NoArg(mne.to_string()))
    )
));
named!(operand<Argument>, ws!(
    alt!(literal | register | mem_access | label_use)
));
named!(literal<Argument>, do_parse!(
    tag!("$") >>
    val: int >>

    (Argument::Literal(val))
));
// TODO: Augment with scaled indexing capabilities
named!(mem_access<Argument>, do_parse!(
    off: opt!(int) >>
    tag!("(") >>
    idx: operand >>
    tag!(")") >>
    (Argument::Mem(Box::new(idx), off.unwrap_or(0), 1, 0))
));
named!(label_use<Argument>, do_parse!(
    val: map_res!(alphanumeric, str::from_utf8) >>
    (Argument::Label(val.to_string()))
));

// Helper functions
named!(digits<&str>, map_res!(digit, str::from_utf8));
named!(int<i32>, do_parse!(
    n: opt!(tag!("-")) >>
    val: map_res!(digits, str::FromStr::from_str) >>
    (match n {
        Some(_) => -1 * val,
        None => val
    })
));


// Instruction/Register Mnemonic Setss
named!(one_arg_mnemonic<&str>,
    map_res!(alt!(
        tag!("push") | tag!("pop") |
        tag!("inc") | tag!("dec") |
        tag!("neg") | tag!("not") |
        tag!("imulq") | tag!("mulq") |
        tag!("idivq") | tag!("divq") |
        tag!("sete") | tag!("setz") |
        tag!("setne") | tag!("setnz") |
        tag!("sets") | tag!("setns") |
        tag!("setg") | tag!("setnle") |
        tag!("setge") | tag!("setnl") |
        tag!("setl") | tag!("setnge") |
        tag!("setle") | tag!("setng") |
        tag!("seta") | tag!("setnbe") |
        tag!("setae") | tag!("setnb") |
        tag!("setb") | tag!("setnae") |
        tag!("setbe") | tag!("setna") |
        tag!("jmp") | tag!("je") |
        tag!("jz") | tag!("jne") |
        tag!("jnz") | tag!("js") |
        tag!("jns") | tag!("jg") |
        tag!("jnle") | tag!("jge") |
        tag!("jnl") | tag!("jl") |
        tag!("jnge") | tag!("jle") |
        tag!("jng") | tag!("ja") |
        tag!("jnbe") | tag!("jae") |
        tag!("jnb") | tag!("jb") |
        tag!("jnae") | tag!("jbe") |
        tag!("jna") | tag!("call")
    ), str::from_utf8)
);
named!(two_arg_mnemonic<&str>, map_res!(
    alt!(
        tag!("movs") | tag!("movz") |
        tag!("mov") | tag!("leaq") |
        tag!("add") | tag!("sub") |
        tag!("imul") | tag!("xor") |
        tag!("or") | tag!("and") |
        tag!("sal") | tag!("shl") |
        tag!("sar") | tag!("shr") |
        tag!("cmp") | tag!("test") |
        tag!("cmove") | tag!("cmovz") |
        tag!("cmovne") | tag!("cmovnz") |
        tag!("cmovs") | tag!("cmovns") |
        tag!("cmovg") | tag!("cmovnle") |
        tag!("cmovge") | tag!("cmovnl") |
        tag!("cmovl") | tag!("cmovnge") |
        tag!("cmova") | tag!("cmovnbe") |
        tag!("cmovae") | tag!("cmovnb") |
        tag!("cmovb") | tag!("cmovnae") |
        tag!("cmobe") | tag!("cmovna")
    ), str::from_utf8
));
named!(no_arg_mnemonic<&str>, map_res!(
    alt!(
        tag!("cwtl") | tag!("cltq") |
        tag!("cqto") | tag!("leave") |
        tag!("ret") | tag!("exit") |
        tag!("dump")
    ), str::from_utf8
));
named!(register<Argument>, do_parse!(
    tag!("%") >>
    reg: map_res!(
        alt!(
            tag!("ah") | tag!("al") |
            tag!("ch") | tag!("cl") |
            tag!("dh") | tag!("dl") |
            tag!("bh") | tag!("bl") |
            tag!("ax") | tag!("cx") |
            tag!("dx") | tag!("bx") |
            tag!("sp") | tag!("bp") |
            tag!("si") | tag!("di") |
            tag!("eax") | tag!("ecx") |
            tag!("edx") | tag!("ebx") |
            tag!("esp") | tag!("ebp") |
            tag!("esi") | tag!("edi") |
            tag!("rax") | tag!("rcx") |
            tag!("rdx") | tag!("rbx") |
            tag!("rsp") | tag!("rbp") |
            tag!("rsi") | tag!("rdi")
        ), str::from_utf8) >>
    (Argument::Reg(reg.to_string()))
));