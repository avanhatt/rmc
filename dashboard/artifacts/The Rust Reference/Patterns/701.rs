// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let pair = (10, "ten");
let (a, b) = pair;

assert_eq!(a, 10);
assert_eq!(b, "ten");
}