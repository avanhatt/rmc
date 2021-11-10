// compile-flags: --edition 2018
#![allow(unused)]
#![feature(unboxed_closures)]

extern "rust-call" fn add_args(args: (u32, u32)) -> u32 {
    args.0 + args.1
}

pub fn main() {}