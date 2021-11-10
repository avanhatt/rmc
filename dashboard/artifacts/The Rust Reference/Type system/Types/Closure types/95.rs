// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let mut b = false;
let x = &mut b;
{
    let mut c = || { *x = true; };
    // The following line is an error:
    // let y = &x;
    c();
}
let z = &x;
}