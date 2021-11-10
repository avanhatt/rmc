// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
macro_rules! ambiguity {
    ($($i:ident)* $j:ident) => { };
}

ambiguity!(error); // Error: local ambiguity
}