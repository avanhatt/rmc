// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
let x: u64;
unsafe {
    asm!("mov {}, 5", out(reg) x);
}
assert_eq!(x, 5);
}