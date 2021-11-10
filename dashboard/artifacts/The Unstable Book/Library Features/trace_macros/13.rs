// compile-flags: --edition 2018
#![allow(unused)]
#![feature(trace_macros)]

pub fn main() {
    trace_macros!(true);
    println!("Hello, Rust!");
    trace_macros!(false);
}