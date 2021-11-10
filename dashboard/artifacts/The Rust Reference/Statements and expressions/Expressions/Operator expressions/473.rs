// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let mut x = 5;
x += 1;
assert!(x == 6);
}