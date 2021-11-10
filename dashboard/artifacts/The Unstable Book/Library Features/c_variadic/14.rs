// compile-flags: --edition 2018
#![allow(unused)]
#![feature(c_variadic)]

pub fn main() {
use std::ffi::VaList;

pub unsafe extern "C" fn vadd(n: usize, mut args: VaList) -> usize {
    let mut sum = 0;
    for _ in 0..n {
        sum += args.arg::<usize>();
    }
    sum
}
}