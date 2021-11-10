// compile-flags: --edition 2018
#![allow(unused)]
#![feature(unsized_tuple_coercion)]

pub fn main() {
    let x : ([i32; 3], [i32; 3]) = ([1, 2, 3], [4, 5, 6]);
    let y : &([i32; 3], [i32]) = &x;
    assert_eq!(y.1[0], 4);
}