// Generic register type for builder style register interaction
// Copyright 2018 Ryan Kurte

use core::ptr::{read_volatile, write_volatile};
use core::ops::{Add, Sub, Not, BitAnd, BitOr, Shl, Shr, BitAndAssign, BitOrAssign};

// Zero trait for RegisterType implementations
#[doc = "Zero trait allows types to be created with a value of zero"]
pub trait Zero {
    fn zero() -> Self;
}

// One trait for RegisterType implementations
#[doc = "One trait allows types to be created with a value of one"]
pub trait One {
    fn one() -> Self;
}

#[doc = "RegisterType trait allows register implementations to be generic over unsigned integer types"]
pub trait RegisterType<T>: Zero + One
                    + Not<Output=T> + Add<T, Output=T> + Sub<T, Output=T>
                    + BitAnd<T, Output=T> + BitOr<T, Output=T> + BitAndAssign<T> + BitOrAssign<T> 
                    + Shl<T, Output=T> + Shr<T, Output=T>
                    + Clone + Copy + Default + PartialEq {}

#[doc = "Helper macro to generate RegisterType implementations for a given type"]
#[macro_export]
macro_rules! register_impl {
    ($t: ty) => {
        impl RegisterType<$t> for $t {}
        impl One for $t {
            fn one() -> $t { 1 }
        }
        impl Zero for $t {
            fn zero() -> $t { 0 }
        }
    }
}

// RegisterType implementations for viable register types
register_impl!(u8);
register_impl!(u16);
register_impl!(u32);
register_impl!(u64);

// Register helper structure
// This uses an internal value and builder approach to simplify interacting with registers.
#[derive(Debug, PartialEq, Clone)]
pub struct Register<T: RegisterType<T>> (pub(crate) usize, pub(crate) T);

impl <T: RegisterType<T>>Register<T> {
    #[doc = "Creates a new register of the provided type with the specified address"]
    #[doc = "Note that `impl RegisterType<T> for T {}` is required for unimplemented types"]
    pub fn new(addr: usize) -> Register<T> {
        Register(addr, T::default())
    }

    #[doc = "Creates a new 16-bit ride register"]
    pub fn u16(addr: usize) -> Register<u16> {
        Register::<u16>::new(addr)
    }

    #[doc = "Creates a new 32-bit register"]
    pub fn u32(addr: usize) -> Register<u32> {
        Register::<u32>::new(addr)
    }

    #[doc = "Reads the register value and returns a new instance with internal value set."]
    pub fn read(&mut self) -> Register<T> {
        let mut reg = self.clone();
        unsafe {
            reg.1 = read_volatile(self.0 as *const T)
        }
        reg
    }

    #[doc = "clears the internal register value"]
    pub fn zero(&mut self) -> Register<T>  {
        let mut reg = self.clone();
        reg.1 = T::zero();
        reg
    }

    #[doc = "returns the register value"]
    pub fn value(&self) -> T {
        self.1.clone()
    }

    #[doc = "sets the internal value of the register"]
    pub fn set(mut self, val: T) -> Register<T>  {
        self.1 = val;
        self
    }

    #[doc = "boolean and the provided and current values"]
    pub fn and(mut self, val: T) -> Register<T> {
        self.1 = self.1 & val;
        self
    }

    #[doc = "ors the provided and current values"]
    pub fn or(mut self, val: T) -> Register<T> {
        self.1 |= val;
        self
    }

    #[doc = "clears the masked area of the provided value"]
    pub fn clear(mut self, mask: T) -> Register<T> {
        self.1 &= !mask;
        self
    }

    #[doc = "returns a boolean consisting to the indexed bit"]
    pub fn get_bit(&self, i: T) -> bool {
        self.1.clone() & (T::one() << i) != T::zero()
    }

    #[doc = "Sets a bit in the current value"]
    pub fn set_bit(mut self, i: T, v: bool) -> Register<T> {
        self.1 = match v {
            true => self.1 | (T::one() << i),
            false => self.1 & !(T::one() << i),
        };
        self
    }

    #[doc = "Fetches a value with the provided mask and shift"]
    #[doc = "Note that shift is applied prior to masking, so mask should always start at 0b1"]
    pub fn get_masked(&self, shift: T, mask: T) -> T  {
        read_masked!(self.1, shift, mask)
    }

    #[doc = "Sets a value with a provided mask and shift"]
    #[doc = "Note that mask is applied before shifting, so mask should always start at 0b1"]
    pub fn set_masked(mut self, shift: T, mask: T, val: T) -> Register<T>  {
        //self.clear(mask.clone()).or((val & mask) << shift);
        write_masked!(self.1, shift, mask, val);
        self
    }

    #[doc = "Writes the internal value to the register"]
    pub fn write(self) {
        unsafe {
            write_volatile(self.0 as *mut T, self.1)
        }
    }
}

#[cfg(test)]
mod tests {
    use ::register::Register;

    #[test]
    fn set() {
        let mut r = Register::<u16>(0, 0);
        assert_eq!(0, r.value());
        r = r.set(100);
        assert_eq!(100, r.value());
    }

    #[test]
    fn zero() {
        let mut r = Register::<u16>(0, 100);
        assert_eq!(100, r.value());
        r = r.zero();
        assert_eq!(0, r.value());
    }

    #[test]
    fn and() {
        let mut r = Register::<u16>(0, 0xFFFF);
        r = r.and(0xF0F0);
        assert_eq!(0xF0F0, r.value());
    }

    #[test]
    fn or() {
        let mut r = Register::<u16>(0, 0xF0F0);
        r = r.or(0x0F00);
        assert_eq!(0xFFF0, r.value());
    }

    #[test]
    fn clear() {
        let mut r = Register::<u16>(0, 0xF0F0);
        r = r.clear(0xF000);
        assert_eq!(0x00F0, r.value());
    }

    #[test]
    fn get_bit() {
        let r = Register::<u16>(0, 0b0101);
        assert_eq!(true,  r.get_bit(0));
        assert_eq!(false, r.get_bit(1));
        assert_eq!(true,  r.get_bit(2));
        assert_eq!(false, r.get_bit(3));
    }

    #[test]
    fn set_bit() {
        let mut r = Register::<u16>(0, 0b0001);
        r = r.set_bit(2, true);
        assert_eq!(0b0101, r.value());
        r = r.set_bit(2, false);
        assert_eq!(0b0001, r.value());
    }

    #[test]
    fn get_masked() {
        let mut r = Register::<u16>(0, 0xFAF0);
        assert_eq!(0x00, r.get_masked(0, 0xf));
        assert_eq!(0x0F, r.get_masked(4, 0xf));
        assert_eq!(0xFA, r.get_masked(8, 0xff));
    }

    #[test]
    fn set_masked() {
        let mut r = Register::<u16>(0, 0x0000);
        r = r.set_masked(0, 0xFF, 0xF0);
        assert_eq!(0x00F0, r.value());
        r = r.set_masked(8, 0xF, 0xA);
        assert_eq!(0x0AF0, r.value());
        r = r.set_masked(12, 0xF, 0xB);
        assert_eq!(0xBAF0, r.value());
    }
}

