extern crate libc;
extern crate nom;
extern crate bit_vec;

mod x86;
mod ximpl;
mod parse;
mod emu;
mod inter;
mod view;

use std::fs::File;

// Pass on the `interpret_code` function to Rust programs
pub use x86::interpret_code;

// Add in a C FFI interface as well
// TODO: Look at returning success status
#[no_mangle]
pub extern "C" fn interpret_file(file: *const libc::c_char) {
    let mut file = File::open(&to_rust_string(file)).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    interpret_code(&contents);
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