use emu;
use inter;
use parse;
use nom::IResult;
use ximpl::{Code, Command};

pub fn interpret_code(code_str: &str) {
    interpret_iter(code_str.split("\n").map(|s| s.to_owned()));
}

pub fn interpret_iter<I: Iterator<Item=String>>(code_iter: I) {
    // Perform initial splitting of code
    let mut code = first_parse(code_iter);

    // Initialize registers and other assembly resources
    let mut emu = emu::Emulator::new();
    inter::collect_labels(&mut code, &mut emu);

    // Run the interpretation loop
    while emu.run() {
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

    if let Code::Parsed(line) = res {
        code[pc] = Code::Parsed(line);
    }

    code.get(pc)
}

// Perform initial organization of the input string
fn first_parse<I: Iterator<Item=String>>(code_iter: I) -> Vec<Code> {
    let mut ret = code_iter.map(|s| Code::Unread(s))
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
        _ => panic!(format!("Invalid x86 instruction string: {}", inst_str))
    }
}
