// compile-flags: --edition 2018
#![allow(unused)]
#![feature(asm)]
pub fn main() {
let cmd = 0xd1;
unsafe {
    asm!("out 0x64, eax", in("eax") cmd);
}
}