extern crate rjs_jit;

use rjs_jit::codegen::x86_64::prologue::*;
use std::mem::transmute;

type CallbackFn = extern "C" fn() -> u64;
type ExternalFn = extern "C" fn(a: u64, b: u64) -> u64;

extern "C" fn callback(a: u64, b: u64) -> u64 {
    a + b
}

extern "C" fn my_fn(a: u64, b: u64) -> u64 {
    callback(a + 10, b + 10) - 10
}

fn main() {
    println!("Hello, world!");
    
    my_fn(3, 5);
    
    let mut gen = Codegen::new();
    
    // Prolog
    
    gen.push(RBP);
    gen.mov(RBP, RSP);
    
    /* ** SIMPLE ADD **
    // Add the values
    
    gen.add(RDX, RCX);
    
    // Write the result
    
    gen.mov(RAX, RDX);
    */
    
    // Reserve arguments space (minimum of 0x20 for shadow stack)
    
    gen.sub(RSP, 0x20);
    
    // Call the function
    
    gen.mov(RAX, callback as u64);
    gen.call(RAX);
    
    // Pop arguments
    
    gen.add(RSP, 0x20);
    
    // Epilog
    
    gen.mov(RSP, RBP);
    gen.pop(RBP);
    
    gen.ret();
    
    unsafe {
        let jit_fn = gen.build();
        let f : ExternalFn = transmute(jit_fn.ptr());
        let result = f(3, 5);
        println!("Result {}", result);
    }
    
    println!("Success");
}
