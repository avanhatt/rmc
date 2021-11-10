// compile-flags: --edition 2018
#![allow(unused)]
#![feature(llvm_asm)]
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn main() {
let result: i32;
unsafe {
   llvm_asm!("mov eax, 2" : "={eax}"(result) : : : "intel")
}
println!("eax is currently {}", result);
}
#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
pub fn main() {}