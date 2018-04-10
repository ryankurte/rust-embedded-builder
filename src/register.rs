
use core::ptr::{read_volatile, write_volatile};
use core::ops::{Add, Sub, Not, BitAnd, BitOr, Shl, Shr, BitAndAssign, BitOrAssign};

// Zero trait for RegisterType implementations
pub trait Zero {
    fn zero() -> Self;
}

// One trait for RegisterType implementations
pub trait One {
    fn one() -> Self;
}

// Unsigned integer trait
// Allows register implementations to be generic over unsigned integer types
pub trait RegisterType<T>: Zero + One
                    + Not<Output=T> + Add<T, Output=T> + Sub<T, Output=T>
                    + BitAnd<T, Output=T> + BitOr<T, Output=T> + BitAndAssign<T> + BitOrAssign<T> 
                    + Shl<T, Output=T> + Shr<T, Output=T>
                    + Clone + Copy + Default + PartialEq {}

// Helper macro for generating register type implementations
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
    // new creates a new register of the provided type with the specified address
    // Note that `impl RegisterType<T> for T {}` is required for unimplemented types
    pub fn new(addr: usize) -> Register<T> {
        Register(addr, T::default())
    }

    // u16 creates a new 16-bit ride register
    pub fn u16(addr: usize) -> Register<u16> {
        Register::<u16>::new(addr)
    }

    // u32 creates a new 32-bit register
    pub fn u32(addr: usize) -> Register<u32> {
        Register::<u32>::new(addr)
    }

    // read reads the register value and returns a new instance with
    // internal value set.
    pub fn read(&mut self) -> Register<T> {
        let mut reg = self.clone();
        unsafe {
            reg.1 = read_volatile(self.0 as *const T)
        }
        reg
    }

    // zero clears the internal register value
    pub fn zero(&mut self) -> Register<T>  {
        let mut reg = self.clone();
        reg.1 = T::zero();
        reg
    }

    // value returns the register value
    pub fn value(&self) -> T {
        self.1.clone()
    }

    // set sets the internal value of the register
    pub fn set(mut self, val: T) -> Register<T>  {
        self.1 = val;
        self
    }

    // and boolean and the provided and current values
    pub fn and(mut self, val: T) -> Register<T> {
        self.1 = self.1 & val;
        self
    }

    // or ors the provided and current values
    pub fn or(mut self, val: T) -> Register<T> {
        self.1 |= val;
        self
    }

    // clear clears the masked area of the provided value
    pub fn clear(mut self, mask: T) -> Register<T> {
        self.1 &= !mask;
        self
    }

    // get_bit returns a boolean consisting to the indexed bit
    pub fn get_bit(&self, i: T) -> bool {
        self.1.clone() & (T::one() << i) != T::zero()
    }

    // set_bit sets a bit in the current value
    pub fn set_bit(mut self, i: T, v: bool) -> Register<T> {
        self.1 = match v {
            true => self.1 | (T::one() << i),
            false => self.1 & !(T::one() << i),
        };
        self
    }

    // get_masked fetches a value with the provided mask and shift
    // Note that shift is applied prior to masking, so mask should always start at 0b1
    pub fn get_masked(&self, shift: T, mask: T) -> T  {
        read_masked!(self.1, shift, mask)
    }

    // set_masked sets a value with a provided mask and shift
    // Note that mask is applied before shifting, so mask should always start at 0b1
    pub fn set_masked(mut self, shift: T, mask: T, val: T) -> Register<T>  {
        //self.clear(mask.clone()).or((val & mask) << shift);
        write_masked!(self.1, shift, mask, val);
        self
    }

    // write writes the internal value to the register
    pub fn write(self) {
        unsafe {
            write_volatile(self.0 as *mut T, self.1)
        }
    }
}
