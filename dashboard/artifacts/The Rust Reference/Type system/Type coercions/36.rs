// compile-flags: --edition 2018
#![allow(unused)]
fn bar(_: &i8) { }

pub fn main() {
    bar(&mut 42);
}