// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
unsafe {
    asm!("nop");
}
}