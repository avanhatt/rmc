// compile-flags: --edition 2018
#![allow(unused)]
use std::mem::MaybeUninit;

pub fn main() {
    unsafe {
        let a = MaybeUninit::<[usize; 4]>::uninit();
        let a = a.assume_init();
        println!("{}", a[2]);
    }
}