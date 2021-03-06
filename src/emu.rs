
use std::mem;
use std::collections::HashMap;
use bit_vec::BitVec;
use ximpl;
use view::Memory;

// #[disable(non_snake_case)]

// 1024 * 1024 overflows the stack
// const MEM_SIZE: usize = 1024 * 1024;
const MEM_SIZE: usize = 1024;

// TODO: Look at abstracting this organization to accomodate different architectures
#[allow(non_snake_case)]
impl Emulator {
    pub fn new() -> Emulator {
        unsafe {
            Emulator{
                eax: mem::transmute(1 as u32),
                ecx: mem::transmute(4200656 as u32),
                edx: [0;4],
                ebx: mem::transmute(2138112 as u32),
                esp: mem::transmute(MEM_SIZE as u32 - 4),
                ebp: mem::transmute(MEM_SIZE as u32 - 4),
                esi: mem::transmute(4199136 as u32),
                edi: mem::transmute(4199136 as u32),
                eflags: BitVec::with_capacity(32),

                pc: 0,
                mem: [0 as u8; MEM_SIZE],
                jumps: HashMap::new(),
                exit_flag: false,
            }
        }
    }

    // TODO: Look into changing the interface (switch String with Argument)
    pub fn getReg(&mut self, reg: &str) -> Result<Memory, String> {
        let reg = match reg {
            // Access the 32bit Memorys
            "eax" => Ok(&mut self.eax[0..4]),
            "ecx" => Ok(&mut self.ecx[0..4]),
            "edx" => Ok(&mut self.edx[0..4]),
            "ebx" => Ok(&mut self.ebx[0..4]),
            "esp" => Ok(&mut self.esp[0..4]),
            "ebp" => Ok(&mut self.ebp[0..4]),
            "esi" => Ok(&mut self.esi[0..4]),
            "edi" => Ok(&mut self.edi[0..4]),
            // There's apparently more than this
            // rXXd, rXXw, rXXb

            // Access the 16bit Memorys
            "ax" => Ok(&mut self.eax[0..2]),
            "cx" => Ok(&mut self.ecx[0..2]),
            "dx" => Ok(&mut self.edx[0..2]),
            "bx" => Ok(&mut self.ebx[0..2]),
            "si" => Ok(&mut self.esi[0..2]),
            "di" => Ok(&mut self.edi[0..2]),
            "sp" => Ok(&mut self.esp[0..2]),
            "bp" => Ok(&mut self.ebp[0..2]),

            // Access the 8bit Memorys
            "ah" => Ok(&mut self.eax[1..2]),
            "al" => Ok(&mut self.eax[0..1]),
            "ch" => Ok(&mut self.ecx[1..2]),
            "cl" => Ok(&mut self.ecx[0..1]),
            "dh" => Ok(&mut self.edx[1..2]),
            "dl" => Ok(&mut self.edx[0..1]),
            "bh" => Ok(&mut self.ebx[1..2]),
            "bl" => Ok(&mut self.ebx[0..1]),
            "sih" => Ok(&mut self.esi[1..2]),
            "sil" => Ok(&mut self.esi[0..1]),
            "dih" => Ok(&mut self.edi[1..2]),
            "dil" => Ok(&mut self.edi[0..1]),
            "sph" => Ok(&mut self.esp[1..2]),
            "spl" => Ok(&mut self.esp[0..1]),
            "bph" => Ok(&mut self.ebp[1..2]),
            "bpl" => Ok(&mut self.ebp[0..1]),

            _ => Err("Attempt to use unsupported registers".to_owned())
        };

        match reg {
            Ok(reg) => Ok(Memory::new(&mut self.eflags, reg)),
            Err(e) => Err(e)
        }
    }

    // 'getReg' type functions that work on the memory tape
    pub fn getMemory(&mut self, loc: i32) -> Memory {
        self.getMemorySized(loc, 4)
    }
    pub fn getMemorySized(&mut self, loc: i32, len: i32) -> Memory {
        let (loc, end) = min_max(loc as usize, (loc + len) as usize);
        Memory::new(&mut self.eflags, &mut self.mem[loc..end])
    }

    // Look at and modify cpu flags
    pub fn getFlag(&self, flag: ximpl::Flag) -> bool {
        self.eflags.get(ximpl::mask_shift(flag)).unwrap_or(false)
    }
    pub fn setFlag(&mut self, flag: ximpl::Flag, val: bool) {
        self.eflags.set(ximpl::mask_shift(flag), val)
    }
    pub fn clearFlags(&mut self) {
        self.eflags.clear()
    }

    
    // Allow for exiting evaluation at any time
    pub fn exit(&mut self) {
        self.exit_flag = true;
    }    
    pub fn run(&self) -> bool {
        !self.exit_flag
    }


    // Look at and modify the program counter
    pub fn updatePC(&mut self) {
        self.pc += 1;
    }
    pub fn setPC(&mut self, pc: usize) {
        self.pc = pc;
    }
    pub fn getPC(&self) -> usize {
        self.pc
    }


    // Work with assembly labels
    pub fn addLabel(&mut self, lbl: &str, idx: usize) {
        self.jumps.entry(lbl.to_string()).or_insert(idx);
    }
    pub fn gotoLabel(&mut self, lbl: &str) {
        match self.jumps.get(lbl) {
            Some(val) => self.pc = *val,
            None => {
                println!("{:?} isn't a valid label!", lbl);
                // Ignore this instruction for now
                self.pc += 1
            }
        }
    }


    // Dump the internals of the Emulator
    pub fn dumpRegisters(&self) {
        println!("\n   ::: x86 Emulator Memory Dump :::");
        unsafe {
            // TODO: Look into switching bits to outputting binary instead
            println!("  %eax: {1:>8}   byts: {0:?}", self.eax, mem::transmute::<[u8;4], u32>(self.eax));
            println!("  %ecx: {1:>8}   byts: {0:?}", self.ecx, mem::transmute::<[u8;4], u32>(self.ecx));
            println!("  %edx: {1:>8}   byts: {0:?}", self.edx, mem::transmute::<[u8;4], u32>(self.edx));
            println!("  %ebx: {1:>8}   byts: {0:?}", self.ebx, mem::transmute::<[u8;4], u32>(self.ebx));
            println!("  %esp: {1:>8}   byts: {0:?}", self.esp, mem::transmute::<[u8;4], u32>(self.esp));
            println!("  %ebp: {1:>8}   byts: {0:?}", self.ebp, mem::transmute::<[u8;4], u32>(self.ebp));
            println!("  %esi: {1:>8}   byts: {0:?}", self.esi, mem::transmute::<[u8;4], u32>(self.esi));
            println!("  %edi: {1:>8}   byts: {0:?}", self.edi, mem::transmute::<[u8;4], u32>(self.edi));
        }

        // println!("eflags: {0:>8}   bits: {0:b}", self.eflags);
        println!("    pc: {0:>8}   bits: 0b{0:b}", self.pc);
    }

    pub fn dumpLabels(&self) {
        println!("\n   ::: x86 Emulator Label Dump :::");

        for (ref label, &val) in self.jumps.iter() {
            println!("  {:<12} -> {}", label, val);
        }
    }

    pub fn dump_all(&self) {
        self.dumpRegisters();
        self.dumpLabels();
    }

    // TODO: Add in function to dump contents of used tape
}

fn min_max(loc: usize, end: usize) -> (usize, usize) {
    if loc > end {
        (end, loc)
    } else {
        (loc, end)
    }
}

pub struct Emulator {
    eax: [u8;4], ecx: [u8;4], esp: [u8;4], ebp: [u8;4],
    edx: [u8;4], ebx: [u8;4], esi: [u8;4], edi: [u8;4],
    eflags: BitVec,

    mem: [u8; MEM_SIZE],

    pc: usize,
    exit_flag: bool,
    jumps: HashMap<String, usize>
}