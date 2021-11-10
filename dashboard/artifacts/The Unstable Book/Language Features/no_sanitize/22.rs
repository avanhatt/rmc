// compile-flags: --edition 2018
#![allow(unused)]
#![feature(no_sanitize)]

pub fn main() {
#[no_sanitize(address)]
fn foo() {
  // ...
}
}