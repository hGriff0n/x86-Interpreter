extern crate nom;
extern crate bit_vec;
extern crate core;

mod processor;
mod inst;
mod op;
mod asm;

// The Operand approach improves signed/unsigned up-/down-conversions
    // Also provides an easy interface for accessing size information
    // However, the interface for accessing the data is necessarily a bit complicated
        // Instructions are also predicated on a 32-bit register sizing
    // If I take this approach, I'll need to:
        // Spend more time on Operand interfaces
        // Rework instructon interface to take in Operand members
        // Rework instruction interface to incorporate the sizing/etc. control flow
// Instruction approach
    // Duplicate the inst.rs instructions to cover all the possibilities
        // Also have to insert the unsigned/etc. checks and coverage
    // Simplest approach, but possibly more complex and fragile
        // Avoids narrowlingly tying instructions to abstraction

// TODO: Start creating abstractions for inst.rs instructions
    // Rep/etc. needs to take a closure that executes another function
    // Provide better facilities for casting bits between various types
    // Represent intermediate/memory/registers
        // Size restrictions, unsigned/signed conversions
        // Could also just add this functionality directly into the instructions
// TODO: Come up with cpu/emulator abstraction
    // Come up with instruction storage abstractions (ie. OneArg/etc.)
// TODO: Implement all instructions for that emulator abstraction level
// TODO: Develop the parser assembler
    // GAS adds b/s/w/l/q/t suffixes to instructions (optional?)
// TODO: Complete the evaluation loop
// TODO: Develop the ffi interface (complete with file save/load)

// Current Plan: Go through and implement instructions for "Operand" interface

fn main() {
    let mut tmp = 5u8;
    let mut al = op::Operand::from_value(&mut tmp);
    let mut _ah = 0u16;
    {
        let mut ah = op::Operand::from_value(&mut _ah);
        let mut flags = processor::FlagRegister::new();

        // inst::aaa(al.mref(), &mut ah, &mut flags);
        // println!("{}{}", ah, al.getU8());

        // *al.mref() = 11u8;
        // {
        //     let mut ah = op::Operand::from_value(&mut ah);
        //     asm::aaa(&mut al, &mut ah, &mut flags);
        // }
        // inst::aaa(al.mref(), &mut ah, &mut flags);
        // println!("{}{}", ah, al.getU8());
        asm::add(&al, &mut ah, &mut flags);
    }
    println!("{}", _ah);
}

// rep inst
// rep(inst: (cpu: &mut Emulator) -> ()) -> ()
