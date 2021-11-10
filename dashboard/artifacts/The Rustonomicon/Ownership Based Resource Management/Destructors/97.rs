// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Boxy<T> {
    data1: Box<T>,
    data2: Box<T>,
    info: u32,
}
}