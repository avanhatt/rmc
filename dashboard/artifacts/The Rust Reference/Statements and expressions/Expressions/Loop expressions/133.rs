// compile-flags: --edition 2018
// rmc-flags: --cbmc-args --unwind 4
#![allow(unused)]
pub fn main() {
let v = &["apples", "cake", "coffee"];

for text in v {
    println!("I like {}.", text);
}
}