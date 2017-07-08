#![feature(box_patterns)]

extern crate nom;
extern crate bit_vec;

mod parse;
mod ximpl;
mod emu;
mod inter;
mod view;

use ximpl::{Code, Command};
use nom::IResult;

// TODO: Modify main to make it usable outside of testing
// TODO: Add in rust tests
// TODO: Add in ffi interface
// TODO: Implement x86 instruction set
// TODO: Improve the register interaction framework
    // TODO: Figure out how to work with unsigned types
    // TODO: Improve the error messages on sizing misalignment

fn main() {
    // interpret_code("start:\nadd $4, %eax\njmp end\ndec %eax\nend: inc %eax");
    interpret_code("mov $4, 0(%esp)\nmov 0(%esp), %eax");
}

fn interpret_code(code_str: &str) {
    // Perform initial splitting of code
    let mut code = first_parse(code_str.to_string());

    // Initialize registers and other assembly resources
    let mut emu = emu::Emulator::new();
    inter::collect_labels(&mut code, &mut emu);

    // Run the interpretation loop
    loop {
        match fetch(&mut code, emu.getPC()) {
            Some(&Code::Parsed(ref inst)) => inter::dispatch(inst, &mut emu),
            _ => break
        };
    }

    emu.dumpRegisters();
    emu.dumpLabels();

    println!("\n   ::: x86 Emulator Instruction Dump :::");
    println!("{:?}", code);
}

// Grab and decode the next instruction
fn fetch<'a>(code: &'a mut Vec<Code>, pc: usize) -> Option<&'a Code> {
    let res = match code.get(pc) {
        Some(&Code::Unread(ref line)) => Code::Parsed(second_parse(line)),
        _ => Code::EndProgram
    };

    match res {
        Code::Parsed(line) => code[pc] = Code::Parsed(line),
        _ => ()
    }

    code.get(pc)
}

// Perform initial splitting/organization of the input string
fn first_parse(code_str: String) -> Vec<Code> {
    let mut ret = code_str.split("\n")
                          .map(|s| Code::Unread(s.to_string()))
                          .collect::<Vec<_>>();

    // Add an instruction at the end to avoid indexing issues
    // Not necessary, but removing this will break `collect_labels`
    // And there is a poetic sense in keeping it around
    ret.push(Code::EndProgram);
    ret
}

// TODO: Maybe look at abstracting this further (ie. break down the construction a bit more)
// TODO: Figure out what I mean by ^
fn second_parse(inst_str: &str) -> Command {
    match parse::x86_instruction(inst_str) {
        IResult::Done(_, res) => res,
        _ => panic!("Invalid x86 instruction string")
    }
}