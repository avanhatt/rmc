// rmc-check-fail
// compile-flags: --edition 2018
#![allow(unused)]
#![feature(rustc_attrs)]

pub fn main() {
#[rustc_layout(abi, size)]
pub enum X {
    Y(u8, u8, u8),
    Z(isize),
}
}