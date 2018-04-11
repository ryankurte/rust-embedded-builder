// Region type for type-safe memory mapping
// Copyright 2018 Ryan Kurte

use core::slice;
use core::ptr::{read_volatile, write_volatile};

// Region helper wraps regions of a given type in volatile read and writes
#[doc = "Region type describes a memory region containing an array of objects"]
#[doc = "This can be used to memory map lists of objects, for example, pixels in a framebuffer"]
#[derive(Debug, PartialEq)]
pub struct Region<T: 'static> (&'static mut[T]);

#[doc = "From implementation creates regions from tuples of (address: usize, size: usize)"]
impl <T>From<(usize, usize)> for Region<T> {
    fn from(v: (usize, usize)) -> Region<T> {
        Region::new(v.0, v.1)
    }
}

// Generic region implementation
impl <T>Region<T> {
    #[doc = "Read an object from the provided (absolute) address"]
    pub fn read_addr(addr: u32) -> T {
        unsafe {
            read_volatile(addr as *const T)
        }
    }
    #[doc = "Write an object to the provided (absolute) address"]
    pub fn write_addr(addr: u32, v: T) {
        unsafe {
            write_volatile(addr as *mut T, v)
        }
    }

    #[doc = "Create a new indexable memory region of the provided type"]
    pub fn new(addr: usize, len: usize) -> Region<T> {
        unsafe {
            let data : &mut [T] = slice::from_raw_parts_mut(addr as *mut T, len);
            Region::<T>(data)
        }
    }
    #[doc = "Read an object from the provided index"]
    pub fn read_index(&self, i: usize) -> &T {
        &self.0[i]
    }
    #[doc = "Write an object to the provided index"]
    pub fn write_index(&mut self, i: usize, v: T) {
        self.0[i] = v;
    }
}