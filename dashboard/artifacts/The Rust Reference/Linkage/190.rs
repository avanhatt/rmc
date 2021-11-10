// compile-flags: --edition 2018
// rmc-flags: --cbmc-args --unwind 0
#![allow(unused)]
use std::env;

pub fn main() {
    let linkage = env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or(String::new());

    if linkage.contains("crt-static") {
        println!("the C runtime will be statically linked");
    } else {
        println!("the C runtime will be dynamically linked");
    }
}