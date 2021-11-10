// compile-flags: --edition 2018
#![allow(unused)]
#![feature(generators, generator_trait)]

use std::ops::Generator;
use std::pin::Pin;

pub fn main() {
    let ret = "foo";
    let mut generator = move || {
        yield 1;
        return ret
    };

    Pin::new(&mut generator).resume(());
    Pin::new(&mut generator).resume(());
}