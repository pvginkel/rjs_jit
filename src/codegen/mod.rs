#![allow(unused_variables)]
#![allow(dead_code)]

macro_rules! jit_assert {
    () => {
        panic!("jit assert");
    };
    ( $expr:expr ) => {
        assert!($expr);
    }
}

mod x86;

struct Emit;

impl Emit {
    fn emit(&mut self, b: u8) {
        unimplemented!();
    }
    
    fn emit_at(&mut self, b: u8, pos: i32) {
        unimplemented!();
    }
    
    fn get(&self) -> u8 {
        self.get_at(0)
    }
    
    fn get_at(&self, offset: i32) -> u8 {
        unimplemented!();
    }
}
