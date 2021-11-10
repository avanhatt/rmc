// compile-flags: --edition 2018
#![allow(unused)]
#![feature(intrinsics)]
pub fn main() {}

extern "rust-intrinsic" {
    fn transmute<T, U>(x: T) -> U;

    fn offset<T>(dst: *const T, offset: isize) -> *const T;
}