// compile-flags: --edition 2018
#![allow(unused)]
#![feature(test)]

extern crate test;
pub fn main() {
use test::Bencher;

#[bench]
fn bench_xor_1000_ints(b: &mut Bencher) {
    b.iter(|| {
        (0..1000).fold(0, |old, new| old ^ new);
    });
}
}