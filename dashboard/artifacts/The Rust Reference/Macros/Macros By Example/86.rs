// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
macro_rules! foo {
    ($l:expr) => { bar!($l); }
// ERROR:               ^^ no rules expected this token in macro call
}

macro_rules! bar {
    (3) => {}
}

foo!(3);
}