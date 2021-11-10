// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
    let xs = [0, 1, 2, 3];
    let _y = unsafe { *xs.as_ptr().offset(4) };
}