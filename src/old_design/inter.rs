
use ximpl::{Command, Code, Argument, Flag};
use emu::*;
use parse;
use nom::IResult;
use std::{str, mem};

// x86 cheatsheet
// https://cs.brown.edu/courses/cs033/docs/guides/x64_cheatsheet.pdf

// Fuller Instruction Listing
// http://www.felixcloutier.com/x86/

pub fn dispatch<'a>(inst: &'a Command, emu: &'a mut Emulator) -> Result<(), String> {
    println!("{:?}", inst);

    match inst {
        &Command::NoArg(ref mne) => run_no_arg(mne, emu),
        &Command::OneArg(ref mne, ref arg) => run_one_arg(mne, arg, emu),
        &Command::TwoArg(ref mne, ref a1, ref a2) => run_two_arg(mne, a1, a2, emu),
        _ => {
            emu.updatePC();
            Ok(())
        }
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
fn run_no_arg(mne: &str, emu: &mut Emulator) -> Result<(), String> {
    // Note: This may result in some problems in the future (for now it's fine)
    emu.updatePC();

    match mne {
        "cwtl" => Err("Unimplemented instruction: `cwtl`".to_owned()),
        "cltq" => Err("Unimplemented instruction: `cltq`".to_owned()),
        "cqto" => Err("Unimplemented instruction: `cqto`".to_owned()),
        "leave" => Err("Unimplemented instruction: `leave`".to_owned()),
        "ret" => Err("Unimplemented instruction: `ret`".to_owned()),
        "exit" => {
            emu.exit();
            Ok(())
        },
        "dump" => {
            emu.dump_all();
            Ok(())
        },
        _ => Err("Unknown instruction".to_owned())
    }
}

fn set_flags(emu: &mut Emulator, old_val: i32, new_val: i32, perf_add: bool) {
    if new_val == 0 {
        emu.setFlag(Flag::Zero, true);
    }

    if new_val < 0 {
        emu.setFlag(Flag::Sign, true);
    }

    match perf_add {
        true if old_val > new_val => emu.setFlag(Flag::Overflow, true),
        false if old_val < new_val => emu.setFlag(Flag::Overflow, true),
        _ => ()
    }

    let val = emu.getFlag(Flag::Overflow);
    emu.setFlag(Flag::Carry, val);

    let test: i8 = unsafe { mem::transmute::<i32, [i8;4]>(new_val)[0] };
    if test.count_zeros() == test.count_ones() {
        emu.setFlag(Flag::Parity, true);
    }

    // TODO: Set 'Auxiliary Carry Flag'
}

fn run_one_arg(mne: &str, arg: &Argument, emu: &mut Emulator) -> Result<(), String> {
    use self::Argument::*;

    let ret = match mne {
        "push" => {
            emu.clearFlags();

            let esp = emu.getReg("esp")?.get();
            let val = get_value(emu, arg)?;
            emu.getMemorySized(esp, -4).set(val)?;

            let mut esp = emu.getReg("esp")?;
            esp -= 4;
            Ok(())
        },
        "pop" => {
            emu.clearFlags();
            
            let esp = {
                let mut esp = emu.getReg("esp")?;
                esp += 4;
                esp.get()
            };

            let val = emu.getMemorySized(esp, -4).get();

            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r)?;
                    reg.set(val)
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    // let mut m = emu.getMemory(base + off);
                    // m.set(val)
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Unsupported operation".to_owned())
            }
        },
        "inc" => {
            emu.clearFlags();
            
            match arg {
                &Reg(ref r) => {
                    let old_val: i32;
                    let new_val = {
                        let mut reg = emu.getReg(r)?;
                        old_val = reg.get();
                        reg += 1;
                        reg.get()
                    };

                    set_flags(emu, old_val, new_val, true);
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let old_val: i32;
                    let new_val = {
                        let base = get_value(emu, &base)?;
                        let mut mem = emu.getMemory(base + off);
                        old_val = mem.get();
                        mem += 1;
                        mem.get()
                    };
                    
                    set_flags(emu, old_val, new_val, true);
                    Ok(())
                },
                _ => Err("Unsupported operation".to_owned())
            }
        },
        "dec" => {
            emu.clearFlags();
            
            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r)?;
                    reg -= 1;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m -= 1;
                    Ok(())
                },
                _ => Err("Unsupported operation".to_owned())
            }

            // TODO: Set OF, SF, ZF, AF, PF flags
        },
        "neg" => {
            emu.clearFlags();
            
            match arg {
                &Reg(ref r) => {
                    let mut reg = emu.getReg(r)?;
                    reg *= -1;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m *= -1;
                    Ok(())
                },
                _ => Err("Unsupported operation".to_owned())
            }

            // TODO: Set OF, SF, ZF, AF, CF, PF flags
        },
        "not" => Err("Unimplemented instruction: `not`".to_owned()),
        "imulq" => Err("Unimplemented instruction: `imulq`".to_owned()),
        "mulq" => Err("Unimplemented instruction: `mulq`".to_owned()),
        "idivq" => Err("Unimplemented instruction: `idivq`".to_owned()),
        "divq" => Err("Unimplemented instruction: `divq`".to_owned()),
        "sete" => {
            let val = if emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setz" => {
            let val = if emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setne" => {
            let val = if emu.getFlag(Flag::Zero) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnz" => {
            let val = if emu.getFlag(Flag::Zero) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "sets" => {
            let val = if emu.getFlag(Flag::Sign) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setns" => {
            let val = if emu.getFlag(Flag::Sign) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setg" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnle" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setge" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow));
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnl" => {
            let val = !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow));
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setl" => {
            let val = if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnge" => {
            let val = if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setle" => {
            let val = (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setng" => {
            let val = (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero);
            let val = if val { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "seta" => {
            let val = if !emu.getFlag(Flag::Carry) && !emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnbe" => {
            let val = if !emu.getFlag(Flag::Carry) && !emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setae" => {
            let val = if emu.getFlag(Flag::Carry) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnb" => {
            let val = if emu.getFlag(Flag::Carry) { 0 } else { 1 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setb" => {
            let val = if emu.getFlag(Flag::Carry) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setnae" => {
            let val = if emu.getFlag(Flag::Carry) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setbe" => {
            let val = if emu.getFlag(Flag::Carry) || emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "setna" => {
            let val = if emu.getFlag(Flag::Carry) || emu.getFlag(Flag::Zero) { 1 } else { 0 };
            
            match arg {
                &Reg(ref r) => emu.getReg(r)?.set(val),
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    emu.getMemory(base + off).set(val)
                },
                _ => Err("Invalid operand".to_owned())
            }
        },
        "jmp" => {
            emu.clearFlags();
            do_jump(emu, arg)
        },
        "je" => {
            if emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jz" => {
            if emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnz" => {
            if !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jne" => {
            if !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "js" => {
            if emu.getFlag(Flag::Sign) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jns" => {
            if !emu.getFlag(Flag::Sign) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jg" => {
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnle" => {
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) && !emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jge" => {
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnl" => {
            if !(emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jl" => {
            if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnge" => {
            if emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jle" => {
            if (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jng" => {
            if (emu.getFlag(Flag::Sign) ^ emu.getFlag(Flag::Overflow)) || emu.getFlag(Flag::Zero) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "ja" => {
            if !emu.getFlag(Flag::Zero) && !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnbe" => {
            if !emu.getFlag(Flag::Zero) && !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jae" => {
            if !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnb" => {
            if !emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jb" => {
            if emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jnae" => {
            if emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jbe" => {
            if emu.getFlag(Flag::Zero) || emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "jna" => {
            if emu.getFlag(Flag::Zero) || emu.getFlag(Flag::Carry) {
                do_jump(emu, arg)
            } else {
                Ok(())
            }
        },
        "call" => Err("Unimplemented instruction: `call`".to_owned()),
        _ => Err("Unknown instruction".to_owned())
    };

    emu.updatePC();
    ret
}

fn run_two_arg(mne: &str, src: &Argument, dest: &Argument, emu: &mut Emulator) -> Result<(), String> {
    use self::Argument::*;

    let ret = match mne {
        "movs" => Err("Unimplemented instruction: `movs`".to_owned()),
        "movz" => Err("Unimplemented instruction: `movz`".to_owned()),
        "mov" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r.set(val)
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m.set(val)
                },
                _ => Err("Invalid operand type to `mov`".to_owned())
            }
        },
        "leaq" => Err("Unimplemented instruction: `leaq`".to_owned()),
        "add" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r += val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m += val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `add`".to_owned())
            }

            // TODO: Set OF, SF, ZF, AF, CF, PF flags
        },
        "sub" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r -= val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m -= val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `sub`".to_owned())
            }

            // TODO: Set OF, SF, ZF, AF, CF, PF flags
        },
        "imul" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r *= val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m *= val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `imul`".to_owned())
            }

            // TODO: Set SF, CF, OF flags
        },
        "xor" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r ^= val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m ^= val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `xor`".to_owned())
            }

            // TODO: Set OF, CF, SF, ZF, PF flags
        },
        "or" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r |= val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base);
                    let mut m = emu.getMemory(base? + off);
                    m |= val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `or`".to_owned())
            }

            // TODO: Set OF, CF, SF, ZF, PF flags
        },
        "and" => {
            emu.clearFlags();
            
            let val = get_value(emu, src)?;

            match dest {
                &Reg(ref r) => {
                    let mut r = emu.getReg(r)?;
                    r &= val;
                    Ok(())
                },
                &Mem(ref base, off, _, _) => {
                    let base = get_value(emu, &base)?;
                    let mut m = emu.getMemory(base + off);
                    m &= val;
                    Ok(())
                },
                _ => Err("Invalid operand type to `and`".to_owned())
            }

            // TODO: Set OF, CF, SF, ZF, PF flags
        },
        "sal" => Err("Unimplemented instruction: `sal`".to_owned()),
        "shl" => Err("Unimplemented instruction: `shl`".to_owned()),
        "sar" => Err("Unimplemented instruction: `sar`".to_owned()),
        "shr" => Err("Unimplemented instruction: `shr`".to_owned()),
        "cmp" => Err("Unimplemented instruction: `cmp`".to_owned()),
        "test" => Err("Unimplemented instruction: `test`".to_owned()),
        "cmove" => Err("Unimplemented instruction: `cmove`".to_owned()),
        "cmovz" => Err("Unimplemented instruction: `cmovz`".to_owned()),
        "cmovne" => Err("Unimplemented instruction: `cmovne`".to_owned()),
        "cmovnz" => Err("Unimplemented instruction: `cmovnz`".to_owned()),
        "cmovs" => Err("Unimplemented instruction: `cmovs`".to_owned()),
        "cmovns" => Err("Unimplemented instruction: `cmovns`".to_owned()),
        "cmovg" => Err("Unimplemented instruction: `cmovg`".to_owned()),
        "cmovnle" => Err("Unimplemented instruction: `cmovnle`".to_owned()),
        "cmovge" => Err("Unimplemented instruction: `cmovge`".to_owned()),
        "cmovnl" => Err("Unimplemented instruction: `cmovnl`".to_owned()),
        "cmovl" => Err("Unimplemented instruction: `cmovl`".to_owned()),
        "cmovnge" => Err("Unimplemented instruction: `cmovnge`".to_owned()),
        "cmovle" => Err("Unimplemented instruction: `cmovle`".to_owned()),
        "cmovng" => Err("Unimplemented instruction: `cmovng`".to_owned()),
        "cmova" => Err("Unimplemented instruction: `cmova`".to_owned()),
        "cmovnbe" => Err("Unimplemented instruction: `cmovnbe`".to_owned()),
        "cmovae" => Err("Unimplemented instruction: `cmovae`".to_owned()),
        "cmovnb" => Err("Unimplemented instruction: `cmovnb`".to_owned()),
        "cmovb" => Err("Unimplemented instruction: `cmovb`".to_owned()),
        "cmovnae" => Err("Unimplemented instruction: `cmovnae`".to_owned()),
        "cmovbe" => Err("Unimplemented instruction: `cmovbe`".to_owned()),
        "cmovna" => Err("Unimplemented instruction: `cmovna`".to_owned()),
        _ => Err("Unknown instruction".to_owned())
    };

    emu.updatePC();
    ret
}

fn get_value(emu: &mut Emulator, arg: &Argument) -> Result<i32, String> {
    match arg {
        &Argument::Reg(ref r) => Ok(emu.getReg(r)?.get()),
        &Argument::Literal(i) => Ok(i),
        &Argument::Mem(ref base, off, _, _) => {
            let base = get_value(emu, base)?;
            Ok(emu.getMemory(base + off).get())
        },
        _ => Err("Attempt to call `get_value` with a non-value type".to_owned())
    }
}

// Handle the common code for all jump commands
fn do_jump(emu: &mut Emulator, arg: &Argument) -> Result<(), String> {
    match arg {
        &Argument::Label(ref s) => {
            emu.gotoLabel(s);
            // TODO: Remove once I no longer have the 'updatePC' at the bottom
            let pc = emu.getPC();
            emu.setPC(pc - 1);
            Ok(())
        },
        // TODO: Be sure to divide the result by 4 (or whatever)
        &Argument::Reg(_) => Err("TODO: Add in register jump addressing".to_owned()),
        &Argument::Mem(_, _, _, _) => Err("TODO: Add in memory jump addressing".to_owned()),
        _ => Err("Encountered invalid jump argument type".to_owned())
    }
}