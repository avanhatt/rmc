// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
let mut x: u64 = 3;
unsafe {
    asm!("add {0}, {number}", inout(reg) x, number = const 5);
}
assert_eq!(x, 8);
}