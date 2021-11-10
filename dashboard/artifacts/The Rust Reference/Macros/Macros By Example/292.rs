// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
#[macro_use]
mod inner {
    macro_rules! m {
        () => {};
    }
}

m!();
}