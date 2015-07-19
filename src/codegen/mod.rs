#![allow(unused_variables)]
#![allow(dead_code)]

use std::ptr;
use self::os::*;

const PAGE_SIZE : usize = 4 * 1024;

macro_rules! jit_assert {
    () => {
        panic!("jit assert");
    };
    ( $expr:expr ) => {
        assert!($expr);
    }
}

pub mod x86;
pub mod x86_64;
mod os;

struct Writer {
    stream: Vec<u8>
}

impl Writer {
    fn new() -> Writer {
        Writer {
            stream: Vec::new()
        }
    }
    
    fn push(&mut self, b: u8) {
        self.stream.push(b);
    }
    
    fn set_at(&mut self, b: u8, pos: usize) {
        self.stream[pos] = b;
    }
    
    fn get(&self) -> u8 {
        self.get_at(0)
    }
    
    fn get_at(&self, pos: usize) -> u8 {
        self.stream[pos]
    }
    
    fn len(&self) -> usize {
        self.stream.len()
    }
    
    fn build(&self) -> JitFunction {
        let size = (self.stream.len() + (PAGE_SIZE - 1)) & !(PAGE_SIZE - 1);
        let memory = Memory::alloc(size).unwrap();
        
        unsafe { ptr::copy(self.stream.as_ptr(), memory.ptr(), self.stream.len()); }
        
        JitFunction {
            memory: memory
        }
    }
}

pub struct Jit;

impl Jit {
    pub fn new() -> Jit {
        Jit
    }
}

pub struct JitFunction {
    memory: Memory
}

impl JitFunction {
    pub unsafe fn ptr(&self) -> *const u8 {
        self.memory.ptr()
    }
}
