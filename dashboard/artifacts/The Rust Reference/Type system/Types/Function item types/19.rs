// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn foo<T>() { }
let x = &mut foo::<i32>;
*x = foo::<u32>; //~ ERROR mismatched types
}