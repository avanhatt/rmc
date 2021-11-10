// compile-flags: --edition 2018
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn add(a: i32, b: i32) -> i32 {
    let c: i32;
    unsafe {
        llvm_asm!("add $2, $0"
             : "=r"(c)
             : "0"(a), "r"(b)
             );
    }
    c
}
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn add(a: i32, b: i32) -> i32 { a + b }

pub fn main() {
    assert_eq!(add(3, 14159), 14162)
}