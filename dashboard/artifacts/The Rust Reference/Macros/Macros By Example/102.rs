// compile-flags: --edition 2018
#![allow(unused)]
// compiles OK
pub fn main() {
macro_rules! foo {
    ($l:tt) => { bar!($l); }
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
}