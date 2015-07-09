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

struct Emit {
    stream: Vec<u8>
}

impl Emit {
    fn push(&mut self, b: u8) {
        self.stream.push(b);
    }
    
    fn set_at(&mut self, b: u8, pos: i32) {
        let offset = (self.stream.len() as isize + pos as isize) as usize;
        self.stream[offset] = b;
    }
    
    fn get(&self) -> u8 {
        self.get_at(0)
    }
    
    fn get_at(&self, pos: i32) -> u8 {
        self.stream[(self.stream.len() as isize + pos as isize) as usize]
    }
}
