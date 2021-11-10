// compile-flags: --edition 2018
#![allow(unused)]
#![feature(inline_const)]

fn add_one(x: i32) -> i32 { x + 1 }
pub fn main() {
    let x = add_one(const { 1 + 2 * 3 / 4 });
}