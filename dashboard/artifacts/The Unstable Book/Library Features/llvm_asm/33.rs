// compile-flags: --edition 2018
#![allow(unused)]
#![feature(llvm_asm)]

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
fn foo() {
    unsafe {
        llvm_asm!("NOP");
    }
}

// Other platforms:
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
fn foo() { /* ... */ }

pub fn main() {
    // ...
    foo();
    // ...
}