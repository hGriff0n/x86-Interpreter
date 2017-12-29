extern crate nom;
extern crate bit_vec;

mod processor;
mod inst;

// GAS adds b/s/w/l/q/t suffixes to instructions (optional?)

fn main() {
    let mut a = 15u32;
    let mut b = 15u32;
    let mut flags = processor::FlagRegister::new();
    inst::add(&a, &mut b, &mut flags);

    println!("{}", b);
    println!("{:?}", flags);
}