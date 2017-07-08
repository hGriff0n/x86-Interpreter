extern crate nom;
extern crate bit_vec;

mod parse;
mod ximpl;
mod emu;
mod inter;
mod view;
mod x86;

// TODO: Modify main to make it usable outside of testing
// TODO: Abstract `interpret_code` to work off of Iterators
// TODO: Add in rust tests
// TODO: Add in ffi interface
// TODO: Implement x86 instruction set
// TODO: Improve the register interaction framework
    // TODO: Figure out how to work with unsigned types
    // TODO: Improve the error messages on sizing misalignment

fn main() {
    
    // interpret_code("start:\nadd $4, %eax\njmp end\ndec %eax\nend: inc %eax");
    x86::interpret_code("mov $4, 0(%esp)\nmov 0(%esp), %eax");
}

fn read_multiline() {

}