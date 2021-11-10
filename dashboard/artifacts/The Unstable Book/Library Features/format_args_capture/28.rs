// compile-flags: --edition 2018
// rmc-flags: --cbmc-args --unwind 5
#![allow(unused)]
#![feature(format_args_capture)]

pub fn main() {
let precision = 2;
let s = format!("{:.precision$}", 1.324223);

assert_eq!(&s, "1.32");
}