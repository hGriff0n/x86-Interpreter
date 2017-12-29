extern crate libc;
extern crate nom;
extern crate bit_vec;

mod x86;
mod ximpl;
mod parse;
mod emu;
mod inter;
mod view;

use std::io::BufReader;
use std::fs::File;
use std::io::prelude::*;
use x86::interpret_iter;

// Pass on the `interpret_code` function for rust usage
pub use x86::interpret_code;

// Add in a C FFI interface as well
#[no_mangle]
pub extern "C" fn interpret_file(file: *const libc::c_char) {
    let file = BufReader::new(
        File::open(&to_rust_string(file)).expect("Unable to open file"));

    interpret_iter(file.lines().map(|l| l.unwrap()));
}

#[no_mangle]
pub extern "C" fn interpret_string(c_str: *const libc::c_char) {
    interpret_code(&to_rust_string(c_str));
}

fn to_rust_string(c_str: *const libc::c_char) -> String {
    unsafe {
         ::std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned()
    }
}