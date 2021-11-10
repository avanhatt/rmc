// compile-flags: --edition 2018
#![allow(unused)]
#![feature(test)]

extern crate test;

pub fn main() {
struct X;
impl X { fn iter<T, F>(&self, _: F) where F: FnMut() -> T {} } let b = X;
b.iter(|| {
    let n = test::black_box(1000);

    (0..n).fold(0, |a, b| a ^ b)
})
}