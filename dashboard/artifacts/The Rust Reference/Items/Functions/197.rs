// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
extern fn new_i32() -> i32 { 0 }
let fptr: extern fn() -> i32 = new_i32;
}