// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let x: i32 = 2 + 3 * 4;
let y: i32 = (2 + 3) * 4;
assert_eq!(x, 14);
assert_eq!(y, 20);
}