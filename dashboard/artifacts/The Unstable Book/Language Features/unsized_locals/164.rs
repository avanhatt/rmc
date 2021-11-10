// compile-flags: --edition 2018
#![allow(unused)]
#![feature(unsized_locals)]

pub fn main() {
    for _ in 0..10 {
        let x: Box<[i32]> = Box::new([1, 2, 3, 4, 5]);
        let _x = *x;
    }
}