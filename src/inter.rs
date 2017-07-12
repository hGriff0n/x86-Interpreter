
use ximpl::{Command, Code, Argument, Flag};
use emu::*;
use parse;
use nom::IResult;
use std::str;

// x86 cheatsheet
// https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf

pub fn dispatch<'a>(inst: &'a Command, emu: &'a mut Emulator) {
    println!("{:?}", inst);

    match inst {
        &Command::NoArg(ref mne) => run_no_arg(mne, emu),
        &Command::OneArg(ref mne, ref arg) => run_one_arg(mne, arg, emu),
        &Command::TwoArg(ref mne, ref a1, ref a2) => run_two_arg(mne, a1, a2, emu),
        _ => emu.updatePC()
    }
}

// Collect and remove all labels within the program
pub fn collect_labels(code: &mut Vec<Code>, emu: &mut Emulator) {
    let mut pc = 0;

    loop {
        // Collect the label and any extra code (if possible)
        let res = match code[pc] {
            Code::Unread(ref s) => match parse::label(s) {
                IResult::Done(left, res) => {
                    let s = unsafe {
                        str::from_utf8_unchecked(left).to_string()
                    };
                    (s, res)
                },
                _ => ("".to_string(), Command::NOP),
            },
            Code::EndProgram => break,
            _ => continue
        };

        // Remember the label location and remove it from the code vector
        match res {
            (ref s, Command::Label(ref lbl)) if s == "" => {
                emu.addLabel(lbl, pc);
                code.remove(pc);
            },
            // If there's some extra code, insert it at the label's spot
            (ref s, Command::Label(ref lbl)) => {
                emu.addLabel(lbl, pc);
                code[pc] = Code::Unread(s.to_string());
                pc += 1;
            },
            _ => pc += 1,
        }
    }
}

#[allow(unreachable_code)]
#[allow(unused_variables)]
fn run_no_arg(mne: &str, emu: &mut Emulator) {
    match mne {
        "cwtl" => panic!("Unimplemented instruction: `cwtl`"),
        "cltq" => panic!("Unimplemented instruction: `cltq`"),
        "cqto" => panic!("Unimplemented instruction: `cqto`"),
        "leave" => panic!("Unimplemented instruction: `leave`"),
        "ret" => panic!("Unimplemented instruction: `ret`"),
        "exit" => emu.exit(),
        "dump" => emu.dump_all(),
        _ => panic!("Unknown instruction")
    }

    emu.updatePC();
}

fn run_one_arg(mne: &str, arg: &Argument, emu: &mut Emulator) {
    use self::Argument::*;

    match mne {
        "push" => {
            let esp = emu.getReg("esp").get();
            let val = get_value(emu, arg);
            emu.getMemorySized(esp, -4).set(val);

            let mut esp = emu.getReg("esp");
            esp -= 4;
        },
        "pop" => {
            let esp = {
                let mut esp = emu.getReg("esp");
                esp += 4;
                esp.get()
            };

            let val = emu.getMemorySized(esp, -4).get();

            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r);
                    reg.set(val);
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m.set(val);
                },
                _ => panic!("Unsupported operation")
            }
        },
        "inc" => {
            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r);
                    reg += 1;
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m += 1;
                },
                _ => panic!("Unsupported operation")
            }
        },
        "dec" => {
            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r);
                    reg -= 1;
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m -= 1;
                },
                _ => panic!("Unsupported operation")
            }
        },
        "neg" => {
            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r);
                    reg *= -1;
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m *= -1;
                },
                _ => panic!("Unsupported operation")
            }
        },
        "not" => panic!("Unimplemented instruction: `not`"),
        "imulq" => panic!("Unimplemented instruction: `imulq`"),
        "mulq" => panic!("Unimplemented instruction: `mulq`"),
        "idivq" => panic!("Unimplemented instruction: `idivq`"),
        "divq" => panic!("Unimplemented instruction: `divq`"),
        "sete" => {
            let val = if emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setz" => {
            let val = if emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setne" => {
            let val = if emu.getFlag(Flag::Zero) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnz" => {
            let val = if emu.getFlag(Flag::Zero) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "sets" => {
            let val = if emu.getFlag(Flag::Sign) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setns" => {
            let val = if emu.getFlag(Flag::Sign) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setg" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnle" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setge" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow));
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnl" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow));
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setl" => {
            let val = if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnge" => {
            let val = if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setle" => {
            let val = (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setng" => {
            let val = (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "seta" => {
            let val = if !emu.getFlag(Flag::Carry) && !emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnbe" => {
            let val = if !emu.getFlag(Flag::Carry) && !emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setae" => {
            let val = if emu.getFlag(Flag::Carry) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnb" => {
            let val = if emu.getFlag(Flag::Carry) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setb" => {
            let val = if emu.getFlag(Flag::Carry) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setnae" => {
            let val = if emu.getFlag(Flag::Carry) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setbe" => {
            let val = if emu.getFlag(Flag::Carry) || emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "setna" => {
            let val = if emu.getFlag(Flag::Carry) || emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r).set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    emu.getMemory(base + off).set(val);
                },
                _ => panic!("Invalid operand")
            }
        },
        "jmp" => do_jump(emu, arg),
        "je" =>
            if emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jz" =>
            if emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jnz" =>
            if !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jne" =>
            if !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "js" =>
            if emu.getFlag(Flag::Sign) {
                do_jump(emu, arg)
            },
        "jns" =>
            if !emu.getFlag(Flag::Sign) {
                do_jump(emu, arg)
            },
        "jg" =>
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jnle" =>
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jge" =>
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) {
                do_jump(emu, arg)
            },
        "jnl" =>
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) {
                do_jump(emu, arg)
            },
        "jl" =>
            if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) {
                do_jump(emu, arg)
            },
        "jnge" =>
            if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) {
                do_jump(emu, arg)
            },
        "jle" =>
            if (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "jng" =>
            if (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            },
        "ja" =>
            if !emu.getFlag(Flag::Zero) && !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jnbe" =>
            if !emu.getFlag(Flag::Zero) && !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jae" =>
            if !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jnb" =>
            if !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jb" =>
            if emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jnae" =>
            if emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jbe" =>
            if emu.getFlag(Flag::Zero) || emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "jna" =>
            if emu.getFlag(Flag::Zero) || emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            },
        "call" => panic!("Unimplemented instruction: `call`"),
        _ => panic!("Unknown instruction")
    }

    emu.updatePC();
}

fn run_two_arg(mne: &str, src: &Argument, dest: &Argument, emu: &mut Emulator) {
    use self::Argument::*;

    match mne {
        "movs" => panic!("Unimplemented instruction: `movs`"),
        "movz" => panic!("Unimplemented instruction: `movz`"),
        "mov" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r.set(val)
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m.set(val)
                },
                _ => panic!("Invalid operand type to `mov`")
            }
        },
        "leaq" => panic!("Unimplemented instruction: `leaq`"),
        "add" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r += val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m += val;
                },
                _ => panic!("Invalid operand type to `add`")
            }
        },
        "sub" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r -= val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m -= val;
                },
                _ => panic!("Invalid operand type to `sub`")
            }
        },
        "imul" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r *= val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m *= val;
                },
                _ => panic!("Invalid operand type to `imul`")
            }
        },
        "xor" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r ^= val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m ^= val;
                },
                _ => panic!("Invalid operand type to `xor`")
            }
        },
        "or" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r |= val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m |= val;
                },
                _ => panic!("Invalid operand type to `or`")
            }
        },
        "and" => {
            let val = get_value(emu, src);

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r);
                    r &= val
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base + off);
                    m &= val;
                },
                _ => panic!("Invalid operand type to `and`")
            }
        },
        "sal" => panic!("Unimplemented instruction: `sal`"),
        "shl" => panic!("Unimplemented instruction: `shl`"),
        "sar" => panic!("Unimplemented instruction: `sar`"),
        "shr" => panic!("Unimplemented instruction: `shr`"),
        "cmp" => panic!("Unimplemented instruction: `cmp`"),
        "test" => panic!("Unimplemented instruction: `test`"),
        "cmove" => panic!("Unimplemented instruction: `cmove`"),
        "cmovz" => panic!("Unimplemented instruction: `cmovz`"),
        "cmovne" => panic!("Unimplemented instruction: `cmovne`"),
        "cmovnz" => panic!("Unimplemented instruction: `cmovnz`"),
        "cmovs" => panic!("Unimplemented instruction: `cmovs`"),
        "cmovns" => panic!("Unimplemented instruction: `cmovns`"),
        "cmovg" => panic!("Unimplemented instruction: `cmovg`"),
        "cmovnle" => panic!("Unimplemented instruction: `cmovnle`"),
        "cmovge" => panic!("Unimplemented instruction: `cmovge`"),
        "cmovnl" => panic!("Unimplemented instruction: `cmovnl`"),
        "cmovl" => panic!("Unimplemented instruction: `cmovl`"),
        "cmovnge" => panic!("Unimplemented instruction: `cmovnge`"),
        "cmovle" => panic!("Unimplemented instruction: `cmovle`"),
        "cmovng" => panic!("Unimplemented instruction: `cmovng`"),
        "cmova" => panic!("Unimplemented instruction: `cmova`"),
        "cmovnbe" => panic!("Unimplemented instruction: `cmovnbe`"),
        "cmovae" => panic!("Unimplemented instruction: `cmovae`"),
        "cmovnb" => panic!("Unimplemented instruction: `cmovnb`"),
        "cmovb" => panic!("Unimplemented instruction: `cmovb`"),
        "cmovnae" => panic!("Unimplemented instruction: `cmovnae`"),
        "cmovbe" => panic!("Unimplemented instruction: `cmovbe`"),
        "cmovna" => panic!("Unimplemented instruction: `cmovna`"),
        _ => panic!("Unknown instruction")
    }

    emu.updatePC();
}

fn get_value(emu: &mut Emulator, arg: &Argument) -> i32 {
    match arg {
        &Argument::Reg(ref r) => emu.getReg(r).get(),
        &Argument::Literal(i) => i,
        &Argument::Mem(ref base, off, _, _) => {
            let base = get_value(emu, base);
            emu.getMemory(base + off).get()
        },
        _ => panic!("Attempt to call `get_value` with a non-value type")
    }
}

// Handle the common code for all jump commands
fn do_jump(emu: &mut Emulator, arg: &Argument) {
    match arg {
        &Argument::Label(ref s) => {
            emu.gotoLabel(s);
            // TODO: Remove once I no longer have the 'updatePC' at the bottom
            let pc = emu.getPC();
            emu.setPC(pc - 1);
        },
        // TODO: Be sure to divide the result by 4 (or whatever)
        &Argument::Reg(_) => panic!("TODO: Add in register jump addressing"),
        &Argument::Mem(_, _, _, _) => panic!("TODO: Add in memory jump addressing"),
        _ => println!("Found something else")
    }
}