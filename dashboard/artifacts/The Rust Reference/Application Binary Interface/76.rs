// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[no_mangle]
#[link_section = ".example_section"]
pub static VAR1: u32 = 1;
}