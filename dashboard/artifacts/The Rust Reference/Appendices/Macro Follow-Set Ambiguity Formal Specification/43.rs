// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
macro_rules! i_am_an_mbe {
    (start $foo:expr $($i:ident),* end) => ($foo)
}
}