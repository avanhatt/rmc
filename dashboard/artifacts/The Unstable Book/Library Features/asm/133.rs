// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
let x: u64 = 3;
let y: u64;
unsafe {
    asm!("add {0}, {number}", inout(reg) x => y, number = const 5);
}
assert_eq!(y, 8);
}