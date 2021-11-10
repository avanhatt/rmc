// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let x = false || true; // true
let y = false && panic!(); // false, doesn't evaluate `panic!()`
}