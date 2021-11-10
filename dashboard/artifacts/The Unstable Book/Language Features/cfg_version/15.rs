// compile-flags: --edition 2018
#![allow(unused)]
#![feature(cfg_version)]

pub fn main() {
#[cfg(version("1.42"))] // 1.42 and above
fn a() {
    // ...
}

#[cfg(not(version("1.42")))] // 1.41 and below
fn a() {
    // ...
}

fn b() {
    if cfg!(version("1.42")) {
        // ...
    } else {
        // ...
    }
}
}