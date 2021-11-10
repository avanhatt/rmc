// compile-flags: --edition 2018
#![allow(unused)]
#![feature(half_open_range_patterns)]
#![feature(exclusive_range_pattern)]
pub fn main() {
let x = 5;
    match x {
        ..0 => println!("negative!"), // "RangeTo" pattern. Unstable.
        0 => println!("zero!"),
        1.. => println!("positive!"), // "RangeFrom" pattern. Stable.
    }
}