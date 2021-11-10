// compile-flags: --edition 2018
#![allow(unused)]
struct Foo<'a> { x: &'a i8 }

pub fn main() {
    Foo { x: &mut 42 };
}