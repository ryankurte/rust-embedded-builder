
use core::slice;
use core::ptr::{read_volatile, write_volatile};

// Region helper wraps regions of a given type in volatile read and writes
#[derive(Debug, PartialEq)]
pub struct Region<T: 'static> (&'static mut[T]);

// From implemenation converts regions from tuples of (address: usize, size: usize)
impl <T>From<(usize, usize)> for Region<T> {
    fn from(v: (usize, usize)) -> Region<T> {
        Region::new(v.0, v.1)
    }
}

// Generic region implementation
impl <T>Region<T> {
    // Read an object from a memory address
     pub fn read_addr(addr: u32) -> T {
        unsafe {
            read_volatile(addr as *const T)
        }
    }
    // Write an object to a memory address
    pub fn write_addr(addr: u32, v: T) {
        unsafe {
            write_volatile(addr as *mut T, v)
        }
    }

    // New creates a new indexable memory region with the provided type
    pub fn new(addr: usize, len: usize) -> Region<T> {
        unsafe {
            let data : &mut [T] = slice::from_raw_parts_mut(addr as *mut T, len);
            Region::<T>(data)
        }
    }
    // Read an object from the provided index
    pub fn read_index(&self, i: usize) -> &T {
        &self.0[i]
    }
    // Write an object to the provided index
    pub fn write_index(&mut self, i: usize, v: T) {
        self.0[i] = v;
    }
}