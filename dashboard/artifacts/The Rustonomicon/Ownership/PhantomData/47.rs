// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
struct Vec<T> {
    data: *const T, // *const for variance!
    len: usize,
    cap: usize,
}
}