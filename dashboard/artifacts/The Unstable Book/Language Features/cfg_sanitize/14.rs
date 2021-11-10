// compile-flags: --edition 2018
#![allow(unused)]
#![feature(cfg_sanitize)]

pub fn main() {
#[cfg(sanitize = "thread")]
fn a() {
    // ...
}

#[cfg(not(sanitize = "thread"))]
fn a() {
    // ...
}

fn b() {
    if cfg!(sanitize = "leak") {
        // ...
    } else {
        // ...
    }
}
}