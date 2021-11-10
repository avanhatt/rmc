// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let a = "foobar";
let b = "foo\
         bar";

assert_eq!(a,b);
}