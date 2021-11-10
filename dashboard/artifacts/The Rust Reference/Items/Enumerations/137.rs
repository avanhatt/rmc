// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
enum ZeroVariants {}
let x: ZeroVariants = panic!();
let y: u32 = x; // mismatched type error
}