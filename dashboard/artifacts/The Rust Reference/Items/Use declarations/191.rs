// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
macro_rules! m {
    ($item: item) => { $item $item }
}

m!(use std as _;);
// This expands to:
// use std as _;
// use std as _;
}