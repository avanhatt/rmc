// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
let mut a: u64 = 4;
let b: u64 = 4;
unsafe {
    asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
}
assert_eq!(a, 8);
}