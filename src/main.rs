extern crate nom;
extern crate bit_vec;

mod parse;
mod ximpl;
mod emu;
mod inter;
mod view;
mod x86;

use std::io;
use std::vec::Vec;

// TODO: Add in "instruction" to immediately exit execution
// TODO: Abstract `interpret_code` to work off of Iterators
// TODO: Add in rust tests
// TODO: Implement x86 instruction set
// TODO: Improve the register interaction framework
    // TODO: Figure out how to work with unsigned types
    // TODO: Improve the error messages on sizing misalignment
// TODO: Work on displaying of register/memory dumps
    // ie. When do they get displayed/allow for control of display

fn main() {
    // TODO: Perform command line handling

    loop {
        match read_multiline() {
            Some(ref s) => {
                x86::interpret_code(s)
                println!("");   
            },
            None => break,
        }
    }
}

fn read_multiline() -> Option<String> {
    let input = io::stdin();
    let mut in_strs = Vec::new();
    let mut in_str = String::new();

    use std::result::Result;

    if let Ok(_) = input.read_line(&mut in_str) {
        if in_str.trim() != ":q" {
            in_strs.push(in_str.clone().trim().to_owned());

            loop {
                in_str = "".to_owned();
                match input.read_line(&mut in_str) {
                    Ok(_) => {
                        let in_str = in_str.clone().trim().to_owned();
                        if in_str.len() == 0 {
                            break
                        }

                        in_strs.push(in_str);
                    },
                    _ => break
                }
            }
        }
    }

    if in_strs.len() == 0 { None } else { Some(in_strs.join("\n")) }
}