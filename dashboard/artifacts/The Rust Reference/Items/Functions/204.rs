// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
extern "C" fn new_i32() -> i32 { 0 }
let fptr: extern "C" fn() -> i32 = new_i32;
}