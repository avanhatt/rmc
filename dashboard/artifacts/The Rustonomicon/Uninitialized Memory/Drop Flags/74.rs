// compile-flags: --edition 2018
#![allow(unused)]
pub fn main() {
let condition = true;
if condition {
    let x = Box::new(0);
    println!("{}", x);
}
}