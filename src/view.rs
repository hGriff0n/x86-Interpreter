use std::{mem, ops};
use bit_vec::BitVec;

pub struct Memory<'a> {
    loc: &'a mut [u8],
    pub cpu_flags: &'a BitVec,
}

impl<'a> Memory<'a> {
    pub fn new(vec: &'a BitVec, loc: &'a mut [u8]) -> Memory<'a> {
        Memory{ cpu_flags: vec, loc: loc }
    }

    fn imm_transmute<T>(&self) -> &'a T {
        unsafe { mem::transmute(self.loc.as_ptr()) }
    }

    fn transmute<T>(&mut self) -> &'a mut T {
        unsafe { mem::transmute(self.loc.as_mut_ptr()) }
    }

    fn check_value_size(&self, val: i32) -> bool {
        val > match self.loc.len() {
            4 => i32::max_value(),
            2 => i16::max_value() as i32,
            1 => i8::max_value() as i32,
            _ => panic!("Invalid Memory Size")
        }
    }

    // 'Getters'
    pub fn get_unsigned(&self) -> u32 {
        *self.imm_transmute()
    }
    pub fn get(&self) -> i32 {
        *self.imm_transmute()
    }

    // 'Setters'
    pub fn set<T: Into<i32>>(&mut self, value: T) {
        let value = value.into();
        if self.check_value_size(value) {
            panic!("Memory is too small for the given value type")
        }

        match self.loc.len() {
            4 => *self.transmute::<i32>() = value,
            2 => *self.transmute::<i16>() = value as i16,
            1 => *self.transmute::<i8>() = value as i8,
            _ => panic!("Invalid Memory size")
        }
    }
}

macro_rules! impl_assign_op {
    ($_type:ident, $_fn: ident) => {
        impl<'a, T: Into<i32>> ops::$_type<T> for Memory<'a> {
            fn $_fn(&mut self, value: T) {
                let value = value.into();
                if self.check_value_size(value) {
                    panic!("Memory is too small for the given value type")
                }

                match self.loc.len() {
                    4 => self.transmute::<i32>().$_fn(value),
                    2 => self.transmute::<i16>().$_fn(value as i16),
                    1 => self.transmute::<i8>().$_fn(value as i8),
                    _ => panic!("Invalid Memory size")
                }
            }
        }
        
        // NOTE: `i32` probably implements `Into<u32> + Into<i32>`
        // That makes the unsigned interface unhandleable
        // impl<'a> ops::$_type<u32> for Memory<'a> {}
    };
}

impl_assign_op!(AddAssign, add_assign);
impl_assign_op!(BitAndAssign, bitand_assign);
impl_assign_op!(BitOrAssign, bitor_assign);
impl_assign_op!(BitXorAssign, bitxor_assign);
impl_assign_op!(DivAssign, div_assign);
impl_assign_op!(MulAssign, mul_assign);
impl_assign_op!(RemAssign, rem_assign);
impl_assign_op!(SubAssign, sub_assign);


impl<'a, T: Into<i8>> ops::ShlAssign<T> for Memory<'a> {
    fn shl_assign(&mut self, value: T) {
        let value = value.into();
        match self.loc.len() {
            4 => *self.transmute::<i32>() <<= value,
            2 => *self.transmute::<i16>() <<= value,
            1 => *self.transmute::<i8>() <<= value,
            _ => panic!("Invalid Memory size")
        }
    }
}

impl<'a, T: Into<i8>> ops::ShrAssign<T> for Memory<'a> {
    fn shr_assign(&mut self, value: T) {
        let value = value.into();
        match self.loc.len() {
            4 => *self.transmute::<i32>() >>= value,
            2 => *self.transmute::<i16>() >>= value,
            1 => *self.transmute::<i8>() >>= value,
            _ => panic!("Invalid Memory size")
        }
    }
}