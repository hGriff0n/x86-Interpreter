
use std::mem;
use std::slice;
use std::u8;

pub struct Operand<'a> {
    loc: &'a mut[u8],
    pub origin: OpType,
}

// TODO: Figure out how to construct from numbers
// TODO: Implement the trait that allows for indexing
// TODO: Can I genericize this at all?
#[allow(non_snake_case)]
impl<'a> Operand<'a> {
    pub fn memory(loc: &'a mut [u8]) -> Self {
        Self::ensure_valid_size(loc.len());
        Self{ loc: loc, origin: OpType::MEM }
    }
    pub fn register(loc: &'a mut [u8]) -> Self {
        Self::ensure_valid_size(loc.len());
        Self{ loc: loc, origin: OpType::REG }
    }
    pub fn from_value<T>(val: &'a mut T) -> Self {
        Self::ensure_valid_size(mem::size_of::<T>());
        Self{ loc: Self::mk_slice(val), origin: OpType::IMM }
    }

    // Size queries
    pub fn len(&self) -> usize {
        self.loc.len()
    }

    // Fit queries
    // FIXME: Delete ???
    pub fn isFittableT<T>(&self) -> bool {
        mem::size_of::<T>() <= self.len()
    }
    pub fn isFittableOp<'b>(&self, op: &'b Operand) -> bool {
        op.loc.len() <= self.len()
    }

    // Generic access
    pub fn iref<T>(&self) -> &'a T {
        unsafe{ mem::transmute(self.loc.as_ptr()) }
    }
    pub fn mref<T>(&mut self) -> &'a mut T {
        Self::ensure_valid_size(mem::size_of::<T>());

        if !self.isFittableT::<T>() {
            panic!("Attempt to extract mutable reference to an invalid sized type");
        }

        unsafe{ mem::transmute(self.loc.as_ptr()) }
    }

    // Value insertion
    // TODO: I have some concerns about this being used with smaller values
    // FIXME: Delte ???
    pub fn setValue<T>(&mut self, val: &'a mut T) {
        if !self.isFittableT::<T>() {
            panic!("Attempt to set an operand from an invalid sized value");
        }

        self.loc.clone_from_slice(unsafe{ Self::mk_slice(val) });
    }
    pub fn setOperand(&mut self, op: &'a mut Operand) {
        self.loc.clone_from_slice(op.loc);
    }

    // Value extraction
    // TODO: Figure out if these are all necessary
    pub fn getU8(&self) -> u8 {
        *self.iref()
    }
    pub fn getU16(&self) -> u16 {
        match self.loc.len() {
            4 => *self.iref(),
            2 => *self.iref(),
            1 => self.getU8() as u16,
            _ => panic!("Invalid operand size")
        }
    }
    pub fn getU32(&self) -> u32 {
        match self.loc.len() {
            4 => *self.iref(),
            2 => self.getU16() as u32,
            1 => self.getU8() as u32,
            _ => panic!("Invalid operand size")
        }
    }
    pub fn getI8(&self) -> i8 {
        *self.iref()
    }
    pub fn getI16(&self) -> i16 {
        match self.loc.len() {
            4 => *self.iref(),
            2 => *self.iref(),
            1 => self.getI8() as i16,
            _ => panic!("Invalid operand size")
        }
    }
    pub fn getI32(&self) -> i32 {
        match self.loc.len() {
            4 => *self.iref(),
            2 => self.getI16() as i32,
            1 => self.getI8() as i32,
            _ => panic!("Invalid operand size")
        }
    }

    // Extension methods
    // TODO: May need some work with i8 interface
    pub fn sxb(&self, idx: usize) -> u8 {
        let sign = self.getI32() < 0;
        // println!("{:b}:{}", self.getI32(), sign);

        if idx < self.len() {
            // NOTE: The way integers are stored on my system means this line is wrong
            // self.loc[self.len() - idx - 1]
            self.loc[idx]
        } else if sign {
            u8::MAX
        } else {
            0
        }
    }

    pub fn zxb(&self, idx: usize) -> u8 {
        if idx < self.len() {
            // self.loc[self.len() - idx - 1]
            self.loc[idx]
        } else {
            0
        }
    }

    // TODO: Work on error handling interface ???
    pub fn at(&mut self, idx: usize) -> &mut u8 {
        if idx < self.len() {
            // &mut self.loc[self.len() - idx - 1]
            &mut self.loc[idx]
        } else {
            panic!("Index out of bounds")
        }
    }

    pub fn msb(&self) -> bool {
        (self.getI32() ^ (1 << 31)) != 0
    }

    // Private helper functions
    fn ensure_valid_size(len: usize) {
        if len != 4 && len != 2 && len != 1 {
            panic!("Operand only accepts values of 1/2/4 bytes")
        }
    }
    fn mk_slice<T>(val: &'a mut T) -> &'a mut [u8] {
        let val: *mut T = val;
        let val: *mut u8 = val as *mut _;
        let size = mem::size_of::<T>();

        unsafe{ slice::from_raw_parts_mut(val, size) }
    }
}

use std;
impl<'a> std::convert::From<&'a Operand<'a>> for u32 {
    fn from(op: &'a Operand) -> u32 {
        op.getU32()
    }
}
impl<'a> std::convert::From<&'a Operand<'a>> for i32 {
    fn from(op: &'a Operand) -> i32 {
        op.getI32()
    }
}

#[derive(Debug,PartialEq,Eq)]
pub enum OpType {
    MEM,
    REG,
    IMM
}
