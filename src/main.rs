extern crate nom;
extern crate bit_vec;

mod inst;

// GAS adds b/s/w/l/q/t suffixes to instructions (optional?)

fn main() {
    let mut a = 3 as u32;
    let mut b = 4 as u32;
    inst::add(&a, &mut b);

    println!("{}", b);
}