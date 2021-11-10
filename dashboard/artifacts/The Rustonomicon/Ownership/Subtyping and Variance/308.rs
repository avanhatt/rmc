// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
fn evil_feeder<T>(input: &mut T, val: T) {
    *input = val;
}
}