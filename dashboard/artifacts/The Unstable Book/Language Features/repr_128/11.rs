// compile-flags: --edition 2018
#![allow(unused)]
#![feature(repr128)]

pub fn main() {
#[repr(u128)]
enum Foo {
    Bar(u64),
}
}