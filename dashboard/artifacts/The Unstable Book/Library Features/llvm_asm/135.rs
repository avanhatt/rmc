// compile-flags: --edition 2018
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn main() { unsafe {
// Put the value 0x200 in eax:
llvm_asm!("mov $$0x200, %eax" : /* no outputs */ : /* no inputs */ : "eax");
} }
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn main() {}