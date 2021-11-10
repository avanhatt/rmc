// compile-flags: --edition 2018
#![allow(unused)]
// file: src/main.rs
use std::ptr;

#[inline(never)]
pub fn main() {
    let xs = [0u32; 2];

    // force LLVM to allocate `xs` on the stack
    unsafe { ptr::read_volatile(&xs.as_ptr()); }
}