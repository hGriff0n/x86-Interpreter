extern crate nom;
extern crate bit_vec;

mod processor;
mod inst;

// GAS adds b/s/w/l/q/t suffixes to instructions (optional?)

fn main() {
    let mut a = 14u32;
    let mut b = 13u32;
    let mut flags = processor::FlagRegister::new();
    inst::sub(&a, &mut b, &mut flags);

    println!("{}", b);
    println!("{:?}", flags);
}