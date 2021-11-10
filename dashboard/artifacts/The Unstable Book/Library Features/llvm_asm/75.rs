// compile-flags: --edition 2018
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn main() { unsafe {
llvm_asm!("xor %eax, %eax" ::: "eax");
} }
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn main() {}